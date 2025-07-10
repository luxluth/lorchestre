use std::collections::HashMap;

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use kiddo::float::kdtree::KdTree;
use kiddo::{NearestNeighbour, SquaredEuclidean};
pub use vectorized::VectorLike;

mod vectorized;

fn vec_to_array<const N: usize>(v: &[f32]) -> [f32; N] {
    let mut arr = [0.0; N];
    let len = v.len().min(N);
    arr[..len].copy_from_slice(&v[..len]);
    arr
}

impl VectorLike for String {
    fn transform<V>(&self, di: &mut Di<V>) -> Vec<f32> {
        for word in self.split_whitespace() {
            let token = word.to_lowercase();
            if !di.grammar.contains(&token) {
                di.grammar.push(token.clone());
                di.token_index.insert(token, di.grammar.len() - 1);
            }
        }

        let mut counts = vec![0.0f32; di.grammar.len()];
        for word in self.split_whitespace() {
            if let Some(idx) = di.token_index.get(word) {
                counts[*idx] += 1.0;
            }
        }
        let total: f32 = counts.iter().sum();
        counts.into_iter().map(|c| c / total).collect()
    }

    fn transform_flat<V>(&self, di: &Di<V>) -> Vec<f32> {
        let mut counts = vec![0.0f32; di.grammar.len()];
        for word in self.split_whitespace() {
            if let Some(idx) = di.token_index.get(word) {
                counts[*idx] += 1.0;
            }
        }
        let total: f32 = counts.iter().sum();
        counts.into_iter().map(|c| c / total).collect()
    }
}

impl VectorLike for Vec<String> {
    fn transform<V>(&self, di: &mut Di<V>) -> Vec<f32> {
        for word in self {
            let token = word.to_lowercase();
            if !di.grammar.contains(&token) {
                di.grammar.push(token.clone());
                di.token_index.insert(token, di.grammar.len() - 1);
            }
        }

        let mut counts = vec![0.0f32; di.grammar.len()];
        for word in self {
            if let Some(idx) = di.token_index.get(word) {
                counts[*idx] += 1.0;
            }
        }
        let total: f32 = counts.iter().sum();
        counts.into_iter().map(|c| c / total).collect()
    }

    fn transform_flat<V>(&self, di: &Di<V>) -> Vec<f32> {
        let mut counts = vec![0.0f32; di.grammar.len()];
        for word in self {
            if let Some(idx) = di.token_index.get(word) {
                counts[*idx] += 1.0;
            }
        }
        let total: f32 = counts.iter().sum();
        counts.into_iter().map(|c| c / total).collect()
    }
}

#[derive(Debug)]
pub struct Di<V> {
    keys: Vec<((Vec<f32>, f32), usize)>,
    values: Vec<Vec<V>>,
    pub grammar: Vec<String>,
    pub token_index: HashMap<String, usize>,
    kd_tree: KdTree<f32, usize, 100, 32, u16>,
    pub epsilon: f32,
}

// #[derive(Default)]
// pub enum Tokenizer {
//     #[default]
//     Word,
//     CharNgram(usize),
// }

impl<V> Di<V> {
    pub fn new(epsilon: f32) -> Self {
        Self {
            keys: vec![],
            values: vec![],
            grammar: vec![],
            token_index: HashMap::new(),
            kd_tree: KdTree::new(),
            epsilon,
        }
    }

    pub fn insert<K: VectorLike>(&mut self, key: K, value: V) {
        let vec = key.transform(self);
        let norm = Self::cosine_similarity_norm(&vec);
        let id = self.values.len();
        self.kd_tree.add(&vec_to_array::<100>(&vec), id);

        // for (i, (existing_vec, _)) in self.keys.iter().enumerate() {
        //     let sim = Self::cosine_similarity(&vec, &existing_vec.0);
        //     if sim >= self.epsilon {
        //         self.values[i].push(value);
        //         return;
        //     }
        // }

        self.keys.push(((vec, norm), self.values.len()));
        self.values.push(vec![value]);
    }

    fn fit_tokens(&self, matcher: &SkimMatcherV2, text: &str) -> Vec<String> {
        let mut normalized = vec![];
        for token in text.split_whitespace() {
            if let Some((best, _score)) = self
                .grammar
                .iter()
                .filter_map(|key| {
                    let score = matcher.fuzzy_match(key, &token).unwrap_or(0);
                    if score > 0 {
                        Some((key.clone(), score))
                    } else {
                        None
                    }
                })
                .max_by_key(|&(_, score)| score)
            {
                normalized.push(best);
            }
        }

        normalized
    }

    pub fn search<K: VectorLike + ToString>(&self, query: K, max: usize) -> Vec<(&V, f32)> {
        let matcher = SkimMatcherV2::default().smart_case();
        let query_tokens = self.fit_tokens(&matcher, &query.to_string());
        let query_vec = query_tokens.transform_flat(self);
        let query_array = vec_to_array::<100>(&query_vec);
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
            for val in &self.values[index] {
                out.push((val, sim));
            }
        }

        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        out
    }

    fn cosine_similarity_norm(v: &[f32]) -> f32 {
        v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    // fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    //     let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    //     let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    //     let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    //     dot / (norm_a * norm_b + 1e-8)
    // }

    fn cosine_similarity(query: &[f32], query_norm: f32, key: &[f32], key_norm: f32) -> f32 {
        let dot = query.iter().zip(key).map(|(x, y)| x * y).sum::<f32>();
        dot / (query_norm * key_norm + 1e-8)
    }
}

// fn tokenize_ngrams(text: &str, n: usize) -> Vec<String> {
//     let chars: Vec<char> = text.to_lowercase().chars().collect();
//     chars.windows(n).map(|w| w.iter().collect()).collect()
// }
