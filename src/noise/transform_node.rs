use crate::geometry::{LinearMap, RealPoint};
use super::NoiseNode;

pub struct TransformNode<'a, const DIM: usize, Source>
where Source: NoiseNode<DIM> {
    source: &'a Source,
    linear_map: LinearMap<DIM>
}

impl<'a, const DIM: usize, Source> TransformNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    pub fn new(source: &'a Source, linear_map: LinearMap<DIM>) -> Self {
        Self { source, linear_map }
    }
}

impl<'a, const DIM: usize, Source> NoiseNode<DIM> for TransformNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        self.source.value_at(self.linear_map.apply(point))
    }
}
