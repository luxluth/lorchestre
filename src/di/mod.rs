use std::collections::HashMap;

use bincode::enc::write::Writer;
use bincode::{BorrowDecode, Decode, Encode};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use kiddo::float::kdtree::KdTree;
use kiddo::{NearestNeighbour, SquaredEuclidean};
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

/// Compress zeros in a vector:
/// - Leading/trailing zeros removed
/// - Internal runs of zeros collapsed to a single 0.0
/// Example:
/// [0.0, 1.0, 0.0, 0.0, 2.0, 0.0, 3.0, 0.0] → [1.0, 0.0, 2.0, 0.0, 3.0]
fn compress_zeros_between_values(v: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(v.len());
    let mut seen_nonzero = false;
    let mut pending_zero = false;

    for &val in v {
        if val == 0.0 {
            if seen_nonzero {
                pending_zero = true;
            }
        } else {
            if pending_zero {
                out.push(0.0); // single zero for any previous run
                pending_zero = false;
            }
            out.push(val);
            seen_nonzero = true;
        }
    }

    out
}

fn vec_to_array<const N: usize>(v: &[f64]) -> [f64; N] {
    let compressed = v;
    let mut arr = [0.0; N];
    let len = compressed.len().min(N);
    arr[..len].copy_from_slice(&compressed[..len]);
    arr
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
            compress_zeros_between_values(&counts)
        } else {
            compress_zeros_between_values(
                &counts.into_iter().map(|c| c / total).collect::<Vec<f64>>(),
            )
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
            compress_zeros_between_values(&counts)
        } else {
            compress_zeros_between_values(
                &counts.into_iter().map(|c| c / total).collect::<Vec<f64>>(),
            )
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
            compress_zeros_between_values(&counts)
        } else {
            compress_zeros_between_values(
                &counts.into_iter().map(|c| c / total).collect::<Vec<f64>>(),
            )
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
            compress_zeros_between_values(&counts)
        } else {
            compress_zeros_between_values(
                &counts.into_iter().map(|c| c / total).collect::<Vec<f64>>(),
            )
        }
    }
}

#[derive(Debug, Default)]
pub struct Di<V> {
    cmds: Option<Vec<InsertCommand<V>>>,
    pub keys: Vec<((Vec<f64>, f64), usize)>,
    pub values: Vec<V>,
    pub grammar: Grammar,
    kd_tree: KdTree<f64, usize, 32, 500, u16>,
    // pub epsilon: f32,
}

impl<V: Encode> Encode for Di<V> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let config = encoder.config().clone();
        let w = encoder.writer();
        w.write(&bincode::encode_to_vec(&self.keys, config.clone())?)?;
        w.write(&bincode::encode_to_vec(&self.values, config.clone())?)?;
        w.write(&bincode::encode_to_vec(&self.grammar, config.clone())?)?;

        let pairs: Vec<(usize, [f64; 32])> = self.kd_tree.iter().collect();
        w.write(&bincode::encode_to_vec(pairs, config.clone())?)?;

        Ok(())
    }
}

impl<Context, V> Decode<Context> for Di<V>
where
    V: Decode<Context>,
{
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let keys = bincode::Decode::decode(decoder)?;
        let values = bincode::Decode::decode(decoder)?;
        let grammar = bincode::Decode::decode(decoder)?;
        let pairs: Vec<(usize, [f64; 32])> = bincode::Decode::decode(decoder)?;

        let mut kd_tree: KdTree<f64, usize, 32, 500, u16> = KdTree::new();

        for (item, query) in pairs {
            kd_tree.add(&query, item)
        }

        Ok(Self {
            cmds: Some(vec![]),
            keys,
            values,
            grammar,
            kd_tree,
        })
    }
}

impl<'de, Context, V> BorrowDecode<'de, Context> for Di<V>
where
    V: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let keys = bincode::BorrowDecode::borrow_decode(decoder)?;
        let values = bincode::BorrowDecode::borrow_decode(decoder)?;
        let grammar = bincode::BorrowDecode::borrow_decode(decoder)?;
        let pairs: Vec<(usize, [f64; 32])> = bincode::BorrowDecode::borrow_decode(decoder)?;

        let mut kd_tree: KdTree<f64, usize, 32, 500, u16> = KdTree::new();

        for (item, query) in pairs {
            kd_tree.add(&query, item)
        }

        Ok(Self {
            cmds: Some(vec![]),
            keys,
            values,
            grammar,
            kd_tree,
        })
    }
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

impl<V> Di<V> {
    pub fn new() -> Self {
        Self {
            cmds: Some(vec![]),
            keys: vec![],
            values: vec![],
            grammar: Grammar::new(),
            kd_tree: KdTree::new(),
            // epsilon,
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
            let id = self.values.len();
            self.kd_tree.add(&vec_to_array::<32>(&vec), id);

            self.keys.push(((vec, norm), self.values.len()));
            self.values.push(cmd.value);
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

    pub fn search<K: ToString>(&self, query: K, max: usize) -> Vec<(&V, f64)> {
        let matcher = SkimMatcherV2::default().smart_case();
        let query_tokens = self.fit_tokens(&matcher, &query.to_string());
        let query_vec = query_tokens.transform_flat(self);
        let query_array = vec_to_array::<32>(&query_vec);
        let query_norm = Self::cosine_similarity_norm(&query_vec);

        let neighbors = self
            .kd_tree
            .nearest_n::<SquaredEuclidean>(&query_array, max);
        let mut out = vec![];

        for NearestNeighbour {
            item: index,
            distance: _,
        } in neighbors
        {
            let (existing_vec, norm) = &self.keys[index].0;
            let sim = Self::cosine_similarity(&query_vec, query_norm, existing_vec, *norm);

            out.push((&self.values[index], sim));
        }

        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        out
    }

    fn cosine_similarity_norm(v: &[f64]) -> f64 {
        v.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    // fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    //     let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    //     let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    //     let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    //     dot / (norm_a * norm_b + 1e-8)
    // }

    fn cosine_similarity(query: &[f64], query_norm: f64, key: &[f64], key_norm: f64) -> f64 {
        let dot = query.iter().zip(key).map(|(x, y)| x * y).sum::<f64>();
        dot / (query_norm * key_norm + 1e-8)
    }
}

// fn tokenize_ngrams(text: &str, n: usize) -> Vec<String> {
//     let chars: Vec<char> = text.to_lowercase().chars().collect();
//     chars.windows(n).map(|w| w.iter().collect()).collect()
// }
