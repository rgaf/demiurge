use crate::geometry::RealPoint;
use super::NoiseNode;

pub struct InvertNode<'a, const DIM: usize, Source>
where Source: NoiseNode<DIM> {
    source: &'a Source
}

impl<'a, const DIM: usize, Source> InvertNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    pub fn new(source: &'a Source) -> Self {
        Self { source }
    }
}

impl<'a, const DIM: usize, Source> NoiseNode<DIM> for InvertNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        1.0 - self.source.value_at(point)
    }
}
