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
    pub grammar: Grammar,
    pub epsilon: f64,
}

// #[derive(Default)]
// pub enum Tokenizer {
//     #[default]
//     Word,
//     CharNgram(usize),
// }

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

            for (i, ((existing_vec, existing_norm), _)) in self.keys.iter().enumerate() {
                let sim = Self::cosine_similarity(&vec, norm, &existing_vec, *existing_norm);
                if sim >= self.epsilon {
                    self.values[i].push(cmd.value);
                    continue;
                }
            }

            self.keys.push(((vec, norm), self.values.len()));
            self.values.push(vec![cmd.value]);
        }

        self.cmds = Some(vec![])
    }

    fn fit_tokens(&self, matcher: &SkimMatcherV2, text: &str) -> Vec<String> {
        let mut normalized = vec![];

        for token in tokenize(text) {
            if let Some((best_token, _score)) = self
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
                .max_by_key(|&(_, score)| score)
            {
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

        let mut out = vec![];

        for (i, ((existing_vec, norm), _)) in self.keys.iter().enumerate() {
            let sim = Self::cosine_similarity(&query_vec, query_norm, existing_vec, *norm);

            out.push((&self.values[i], sim));
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

// fn tokenize_ngrams(text: &str, n: usize) -> Vec<String> {
//     let chars: Vec<char> = text.to_lowercase().chars().collect();
//     chars.windows(n).map(|w| w.iter().collect()).collect()
// }
