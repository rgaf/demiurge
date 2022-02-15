use crate::geometry::RealPoint;
use super::NoiseNode;

pub struct SoftLightNode<'a, const DIM: usize, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    lhs: &'a Lhs,
    rhs: &'a Rhs
}

impl<'a, const DIM: usize, Lhs, Rhs> SoftLightNode<'a, DIM, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    pub fn new(lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self { lhs, rhs }
    }
}

impl<'a, const DIM: usize, Lhs, Rhs> NoiseNode<DIM> for SoftLightNode<'a, DIM, Lhs, Rhs>
where Lhs: NoiseNode<DIM>, Rhs: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let lhs_value = self.lhs.value_at(point);
        let rhs_value = self.rhs.value_at(point);

        if rhs_value <= 0.5 {
            let lhs_curve = lhs_value * (1.0 - lhs_value);

            rhs_value.mul_add(-2.0, 1.0).mul_add(-lhs_curve, lhs_value)
        } else {
            let g_value = if lhs_value <= 0.25 {
                lhs_value.mul_add(16.0, -12.0).mul_add(lhs_value, 4.0) * lhs_value
            } else {
                lhs_value.sqrt()
            };

            rhs_value.mul_add(2.0, -1.0).mul_add(g_value - lhs_value, lhs_value)
        }
    }
}
