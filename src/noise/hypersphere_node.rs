use std::marker::PhantomData;

use crate::geometry::{DistanceMetric, RealPoint};
use crate::utils;
use super::NoiseNode;

pub struct HypersphereNode<const DIM: usize, Metric>
where Metric: DistanceMetric {
    frequency: f64,
    phantom: PhantomData<Metric>
}

impl<const DIM: usize, Metric> HypersphereNode<DIM, Metric>
where Metric: DistanceMetric {
    pub fn new(frequency: f64) -> Self {
        Self { frequency, phantom: PhantomData }
    }
}

impl<const DIM: usize, Metric> NoiseNode<DIM> for HypersphereNode<DIM, Metric>
where Metric: DistanceMetric {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let distance_from_origin = (point * self.frequency).magnitude::<Metric>();

        let inner_distance = distance_from_origin - distance_from_origin.floor();
        let outer_distance = 1.0 - inner_distance;
        let nearest_distance = inner_distance.min(outer_distance);

        utils::sigmoid(-1.2, nearest_distance.mul_add(-2.0, 1.0))
    }
}
