use std::collections::HashMap;

use bincode::{Decode, Encode};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
pub use vectorized::VectorLike;

mod vectorized;

#[derive(Debug, Default, Decode, Encode)]
pub struct Grammar {
    pub tokens: Vec<(String, usize)>,  // token + frequency
    pub index: HashMap<String, usize>, // token → index in tokens
}

impl Grammar {
    pub fn insert_token(&mut self, token: &str) -> usize {
        if let Some(&idx) = self.index.get(token) {
            self.tokens[idx].1 += 1; // increment frequency
            idx
        } else {
            let idx = self.tokens.len();
            self.tokens.push((token.to_string(), 1));
            self.index.insert(token.to_string(), idx);
            idx
        }
    }

    fn new() -> Self {
        Self::default()
    }
}

fn tokenize(s: &str) -> impl Iterator<Item = String> + '_ {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
}

impl VectorLike for String {
    fn transform<V>(&self, di: &mut Di<V>) -> Vec<f64> {
        let mut counts = vec![0.0f64; di.grammar.tokens.len()];

        for token in tokenize(&self) {
            let idx = if let Some(&idx) = di.grammar.index.get(&token) {
                di.grammar.tokens[idx].1 += 1;
                idx
            } else {
                let idx = di.grammar.tokens.len();
                di.grammar.tokens.push((token.clone(), 1));
                di.grammar.index.insert(token.clone(), idx);
                counts.resize(idx + 1, 0.0); // extend vector if needed
                idx
            };

            counts[idx] += 1.0;
        }

        let total: f64 = counts.iter().sum();
        if total == 0.0 {
            counts
        } else {
            counts.into_iter().map(|c| c / total).collect::<Vec<f64>>()
        }
    }

    fn transform_flat<V>(&self, di: &Di<V>) -> Vec<f64> {
        let mut counts = vec![0.0f64; di.grammar.tokens.len()];

        for token in tokenize(self) {
            if let Some(&idx) = di.grammar.index.get(&token) {
                counts[idx] += 1.0;
            }
        }

        let total: f64 = counts.iter().sum();

        if total == 0.0 {
            counts
        } else {
            counts.into_iter().map(|c| c / total).collect::<Vec<f64>>()
        }
    }
}

impl VectorLike for Vec<String> {
    fn transform<V>(&self, di: &mut Di<V>) -> Vec<f64> {
        let mut counts = vec![0.0f64; di.grammar.tokens.len()];

        for token in self {
            let idx = if let Some(&idx) = di.grammar.index.get(token) {
                di.grammar.tokens[idx].1 += 1;
                idx
            } else {
                let idx = di.grammar.tokens.len();
                di.grammar.tokens.push((token.clone(), 1));
                di.grammar.index.insert(token.clone(), idx);
                counts.resize(idx + 1, 0.0); // extend vector if needed
                idx
            };

            counts[idx] += 1.0;
        }

        let total: f64 = counts.iter().sum();

        if total == 0.0 {
            counts
        } else {
            counts.into_iter().map(|c| c / total).collect::<Vec<f64>>()
        }
    }

    fn transform_flat<V>(&self, di: &Di<V>) -> Vec<f64> {
        let mut counts = vec![0.0f64; di.grammar.tokens.len()];

        for word in self {
            if let Some(&idx) = di.grammar.index.get(word) {
                counts[idx] += 1.0;
            }
        }

        let total: f64 = counts.iter().sum();

        if total == 0.0 {
            counts
        } else {
            counts.into_iter().map(|c| c / total).collect::<Vec<f64>>()
        }
    }
}

#[derive(Debug, Default, Encode, Decode)]
pub struct Di<V> {
    cmds: Option<Vec<InsertCommand<V>>>,
    pub keys: Vec<((Vec<f64>, f64), usize)>,
    pub values: Vec<Vec<V>>,
    pub postings: HashMap<usize, Vec<usize>>,
    pub grammar: Grammar,
    pub epsilon: f64,
}

#[derive(Debug, Default, Decode, Encode)]
struct InsertCommand<V> {
    key: String,
    value: V,
}

impl<V> Di<V>
where
    V: Copy,
{
    pub fn new(epsilon: f64) -> Self {
        Self {
            cmds: Some(vec![]),
            keys: vec![],
            values: vec![],
            postings: HashMap::new(),
            grammar: Grammar::new(),
            epsilon,
        }
    }

    pub fn insert<K: ToString>(&mut self, key: K, value: V) {
        key.to_string().transform(self); // just to feed the grammar

        if let Some(cmds) = self.cmds.as_mut() {
            cmds.push(InsertCommand {
                key: key.to_string(),
                value,
            });
        }
    }

    pub fn finalize(&mut self) {
        for cmd in self.cmds.take().unwrap() {
            let vec = cmd.key.transform_flat(self);
            let norm = Self::cosine_similarity_norm(&vec);

            let mut inserted = false;
            for (i, ((existing_vec, existing_norm), _)) in self.keys.iter().enumerate() {
                let sim = Self::cosine_similarity(&vec, norm, &existing_vec, *existing_norm);
                if sim >= self.epsilon {
                    self.values[i].push(cmd.value);
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                let key_idx = self.keys.len();
                self.keys.push(((vec.clone(), norm), self.values.len()));
                self.values.push(vec![cmd.value]);

                for (token_idx, &weight) in vec.iter().enumerate() {
                    if weight > 0.0 {
                        self.postings
                            .entry(token_idx)
                            .or_insert_with(Vec::new)
                            .push(key_idx);
                    }
                }
            }
        }

        self.cmds = Some(vec![])
    }

    fn fit_tokens(&self, matcher: &SkimMatcherV2, text: &str) -> Vec<String> {
        let mut normalized = vec![];

        for token in tokenize(text) {
            let mut matches: Vec<(String, i64)> = self
                .grammar
                .tokens
                .iter()
                .filter_map(|(grammar_token, _freq)| {
                    let score = matcher.fuzzy_match(grammar_token, &token).unwrap_or(0);
                    if score > 0 {
                        Some((grammar_token.clone(), score))
                    } else {
                        None
                    }
                })
                .collect();

            matches.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

            for (best_token, _) in matches.into_iter().take(10) {
                normalized.push(best_token);
            }
        }

        normalized
    }

    pub fn search<K: ToString>(&self, query: K, max: usize) -> Vec<(&Vec<V>, f64)> {
        let matcher = SkimMatcherV2::default().smart_case();
        let query_tokens = self.fit_tokens(&matcher, &query.to_string());
        let query_vec = query_tokens.transform_flat(self);
        let query_norm = Self::cosine_similarity_norm(&query_vec);

        // Find candidate keys using inverted index
        let mut candidates: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for (token_idx, &weight) in query_vec.iter().enumerate() {
            if weight > 0.0 {
                if let Some(postings) = self.postings.get(&token_idx) {
                    candidates.extend(postings.iter().copied());
                }
            }
        }

        let mut out = vec![];

        if candidates.is_empty() && query_norm > 0.0 {
            // Fallback for rare edge cases or empty postings
            // If candidates is empty but we have query vector, it means no document has these tokens.
            // So return empty result.
        } else {
            for key_idx in candidates {
                let ((existing_vec, norm), value_idx) = &self.keys[key_idx];
                let sim = Self::cosine_similarity(&query_vec, query_norm, existing_vec, *norm);
                out.push((&self.values[*value_idx], sim));
            }
        }

        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        out.into_iter().take(max).collect()
    }

    fn cosine_similarity_norm(v: &[f64]) -> f64 {
        v.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    fn cosine_similarity(query: &[f64], query_norm: f64, key: &[f64], key_norm: f64) -> f64 {
        let dot = query.iter().zip(key).map(|(x, y)| x * y).sum::<f64>();
        dot / (query_norm * key_norm + 1e-8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_multiple_results() {
        let mut di = Di::<&str>::new(0.9);

        di.insert("Le temps", "Song 1");
        di.insert("Lemon tree", "Song 2");
        di.insert("La mer", "Song 3");

        di.finalize();

        // "le" matches "Le temps" (exact token "le").
        // "le" matches "Lemon tree" (fuzzy token "lemon").
        let results = di.search("le", 10);
        println!("Found {} results", results.len());
        for (vals, score) in &results {
            println!("Score: {}, Values: {:?}", score, vals);
        }

        assert!(
            results.len() >= 2,
            "Should return at least 2 results (Le temps and Lemon tree)"
        );
    }
}
