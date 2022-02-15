use crate::geometry::RealPoint;
use crate::utils;
use super::NoiseNode;

pub struct SigmoidNode<'a, const DIM: usize, Source>
where Source: NoiseNode<DIM> {
    source: &'a Source,
    beta: f64
}

impl<'a, const DIM: usize, Source> SigmoidNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    pub fn new(source: &'a Source, beta: f64) -> Self {
        Self { source, beta }
    }
}

impl<'a, const DIM: usize, Source> NoiseNode<DIM> for SigmoidNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        utils::sigmoid(self.beta, self.source.value_at(point))
    }
}
