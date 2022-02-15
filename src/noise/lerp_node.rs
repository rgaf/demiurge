use crate::geometry::RealPoint;
use crate::utils;
use super::NoiseNode;

pub struct LerpNode<'a, const DIM: usize, Bias, Lhs, Rhs>
where Bias: NoiseNode<DIM>, Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    bias: &'a Bias,
    lhs: &'a Lhs,
    rhs: &'a Rhs
}


impl<'a, const DIM: usize, Bias, Lhs, Rhs> LerpNode<'a, DIM, Bias, Lhs, Rhs>
where Bias: NoiseNode<DIM>, Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    pub fn new(bias: &'a Bias, lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self { bias, lhs, rhs }
    }
}

impl<'a, const DIM: usize, Bias, Lhs, Rhs> NoiseNode<DIM> for LerpNode<'a, DIM, Bias, Lhs, Rhs>
where Bias: NoiseNode<DIM>, Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let bias = self.bias.value_at(point);
        let lhs = self.lhs.value_at(point);
        let rhs = self.rhs.value_at(point);

        utils::lerp(bias, lhs, rhs)
    }
}
