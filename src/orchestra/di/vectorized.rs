use super::Di;

pub trait VectorLike {
    fn transform<V>(&self, di: &mut Di<V>) -> Vec<f64>;
    fn transform_flat<V>(&self, di: &Di<V>) -> Vec<f64>;
}
