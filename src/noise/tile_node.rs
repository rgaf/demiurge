use crate::geometry::RealPoint;
use crate::random::StatelessRand;
use crate::utils;
use super::NoiseNode;

pub struct TileNode<const DIM: usize> {
    rng: StatelessRand
}

impl<const DIM: usize> TileNode<DIM> {
    pub fn new(seed: u64) -> Self {
        Self { rng: StatelessRand::from_seed(seed) }
    }
}

impl<const DIM: usize> NoiseNode<DIM> for TileNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let hash = self.rng.hash_bytes(point.floor().as_bytes());

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}
