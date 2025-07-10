use super::Di;

pub trait VectorLike {
    fn transform<V>(&self, grammar: &mut Di<V>) -> Vec<f64>;
    fn transform_flat<V>(&self, grammar: &Di<V>) -> Vec<f64>;
}
