use std::collections::HashMap;
use std::cell::RefCell;
use std::marker::PhantomData;

use crate::geometry::{EuclideanMetric, DistanceMetric, LatticePoint, RealPoint};
use crate::random::{StatefulRand, StatelessRand};
use crate::utils;
use super::NoiseNode;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WorleyPaintMethod {
    Value,
    Distance
}

pub struct WorleyNode<const DIM: usize, Metric>
where Metric: DistanceMetric {
    stateless_rng: StatelessRand,
    stateful_rng: RefCell<StatefulRand>,
    paint_method: WorleyPaintMethod,
    hashmap: RefCell<HashMap<String, Vec<String>>>,
    phantom: PhantomData<Metric>
}

impl<const DIM: usize, Metric> WorleyNode<DIM, Metric>
where Metric: DistanceMetric {
    pub fn new(seed: u64, paint_method: WorleyPaintMethod) -> Self {
        Self {
            stateless_rng: StatelessRand::from_seed(seed),
            stateful_rng: RefCell::new(StatefulRand::from_seed(seed)),
            paint_method,
            hashmap: RefCell::new(HashMap::new()),
            phantom: PhantomData
        }
    }

    pub fn hypercube_seed_point(&self, point: RealPoint<DIM>, hypercube: LatticePoint<DIM>) -> (RealPoint<DIM>, u64) {
        let real_hypercube = hypercube.to_real_point();
        let hash = self.stateless_rng.hash_bytes(hypercube.as_bytes());

        let mut rng = self.stateful_rng.borrow_mut();

        rng.set_word_pos(0);
        rng.set_stream(hash);

        let mut coordinates = [0.0_f64; DIM];

        for dim in 0..DIM {
            let fp_mod = utils::f64_from_mantissa(rng.next_u64(), 0.0, 1.0);

            coordinates[dim] = real_hypercube[dim] + fp_mod;
        };

        (RealPoint::<DIM>::new(coordinates), hash)
    }

    pub fn display(&self) {
        println!("{:?}", self.hashmap.borrow());
    }
}

impl<const DIM: usize, Metric> NoiseNode<DIM> for WorleyNode<DIM, Metric>
where Metric: DistanceMetric {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let mut candidates = point.to_lattice_point().neighbors_and_self().map(|hypercube| {
            let (seed_point, seed_value) = self.hypercube_seed_point(point, hypercube);
            let distance = (seed_point - point).magnitude::<Metric>();

            (seed_value, distance)
        }).collect::<Vec<(u64, f64)>>();

        candidates.sort_by(|a, b| {
            let lhs_distance = a.1;
            let rhs_distance = b.1;

            rhs_distance.partial_cmp(&lhs_distance).unwrap()
        });

        let (seed_value, distance) = candidates.pop().unwrap();

        match self.paint_method {
            WorleyPaintMethod::Value => {
                utils::f64_from_mantissa(seed_value, 0.0, 1.0)
            },

            WorleyPaintMethod::Distance => {
                let (_, other_distance) = candidates.pop().unwrap();

                let max_distance = Metric::hypercube_diagonal_magnitude::<DIM>();
                let unbiased_value = (other_distance - distance) / max_distance;

                distance
            }
        }
    }
}
