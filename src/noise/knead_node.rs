use crate::geometry::RealPoint;
use crate::utils;
use super::NoiseNode;

pub struct KneadNode<'a, const DIM: usize, Source>
where Source: NoiseNode<DIM> {
    source: &'a Source
}

impl<'a, const DIM: usize, Source> KneadNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    pub fn new(source: &'a Source) -> Self {
        Self { source }
    }
}

impl<'a, const DIM: usize, Source> NoiseNode<DIM> for KneadNode<'a, DIM, Source>
where Source: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        utils::unit_to_neg_unit(self.source.value_at(point)).abs()
    }
}
