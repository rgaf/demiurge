use crate::geometry::RealPoint;
use crate::random::StatelessRand;
use crate::utils;
use super::NoiseNode;

pub struct StaticNode<const DIM: usize> {
    rng: StatelessRand,
    min: f64,
    max: f64
}

impl<const DIM: usize> StaticNode<DIM> {
    pub fn new(seed: u64, min: f64, max: f64) -> Self {
        Self { rng: StatelessRand::from_seed(seed), min, max }
    }
}

impl NoiseNode<1> for StaticNode<1> {
    fn value_at(&self, point: RealPoint<1>) -> f64 {
        let x_bits = point[0].to_bits();
        let hash = self.rng.hash_1u64(x_bits);

        utils::f64_from_mantissa(hash, self.min, self.max)
    }
}

impl NoiseNode<2> for StaticNode<2> {
    fn value_at(&self, point: RealPoint<2>) -> f64 {
        let x_bits = point[0].to_bits();
        let y_bits = point[1].to_bits();
        let hash = self.rng.hash_2u64(x_bits, y_bits);

        utils::f64_from_mantissa(hash, self.min, self.max)
    }
}

impl NoiseNode<3> for StaticNode<3> {
    fn value_at(&self, point: RealPoint<3>) -> f64 {
        let x_bits = point[0].to_bits();
        let y_bits = point[1].to_bits();
        let z_bits = point[2].to_bits();
        let hash = self.rng.hash_3u64(x_bits, y_bits, z_bits);

        utils::f64_from_mantissa(hash, self.min, self.max)
    }
}

impl<const DIM: usize> NoiseNode<DIM> for StaticNode<DIM> {
    default fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let hash = self.rng.hash_bytes(point.as_bytes());

        utils::f64_from_mantissa(hash, self.min, self.max)
    }
}
