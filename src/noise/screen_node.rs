use crate::geometry::RealPoint;
use super::NoiseNode;

pub struct ScreenNode<'a, const DIM: usize, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    lhs: &'a Lhs,
    rhs: &'a Rhs
}

impl<'a, const DIM: usize, Lhs, Rhs> ScreenNode<'a, DIM, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    pub fn new(lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self { lhs, rhs }
    }
}

impl<'a, const DIM: usize, Lhs, Rhs> NoiseNode<DIM> for ScreenNode<'a, DIM, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let lhs_value = self.lhs.value_at(point);
        let rhs_value = self.rhs.value_at(point);

        1.0 - (1.0 - lhs_value) * (1.0 - rhs_value)
    }
}
