use super::{LatticePoint, RealPoint};

 //-------------------------------------------------------------------------------------------------
// LatticeNeighbhorhood

pub struct LatticeNeighborhood<const DIM: usize> {
    origin: LatticePoint<DIM>,
    include_self: bool,
    current_index: usize
}

impl<const DIM: usize> LatticeNeighborhood<DIM> {
    const NUM_NEIGHBORS: usize = 3_usize.pow(DIM as u32);
    const SELF_INDEX: usize = Self::NUM_NEIGHBORS / 2;

    pub fn new(origin: LatticePoint<DIM>, include_self: bool) -> Self {
        Self { origin, include_self, current_index: 0 }
    }
}

impl<const DIM: usize> Iterator for LatticeNeighborhood<DIM> {
    type Item = LatticePoint<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < Self::NUM_NEIGHBORS {
            if !self.include_self && self.current_index == Self::SELF_INDEX {
                self.current_index += 1;

                return self.next();
            };

            let mut coordinates = [0_i32; DIM];

            for dim in 0..DIM {
                let idx = self.current_index as i32;
                let divisor = 3_i32.pow(dim as u32);
                let modulus = divisor * 3;

                coordinates[dim] = (idx % modulus) / divisor - 1;
            };

            self.current_index += 1;

            Some(self.origin + LatticePoint::<DIM>::new(coordinates))
        } else {
            None
        }
    }
}

 //-------------------------------------------------------------------------------------------------
// VertexNeighbhorhood

pub struct VertexNeighborhood<const DIM: usize> {
    origin: RealPoint<DIM>,
    current_index: usize
}

impl<const DIM: usize> VertexNeighborhood<DIM> {
    const NUM_VERTICES: usize = 2_usize.pow(DIM as u32);

    pub fn new(point: RealPoint<DIM>) -> Self {
        Self { origin: point.floor(), current_index: 0 }
    }
}

impl<const DIM: usize> Iterator for VertexNeighborhood<DIM> {
    type Item = RealPoint<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < Self::NUM_VERTICES {
            let mut coordinates = [0.0_f64; DIM];

            for dim in 0..DIM {
                if (self.current_index & (1 << dim)) > 0 {
                    coordinates[dim] = 1.0_f64;
                };
            };

            self.current_index += 1;

            Some(self.origin + RealPoint::<DIM>::new(coordinates))
        } else {
            None
        }
    }
}

 //-------------------------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod test {
    use crate::geometry::{LatticePoint, RealPoint};

    #[test]
    fn lattice_neighborhood_iterate() {
        let point = LatticePoint::<2>::new([5, 2]);

        let neighbors: Vec<LatticePoint<2>> = point.neighbors().collect();
        let neighbors_and_self: Vec<LatticePoint<2>> = point.neighbors_and_self().collect();

        assert_eq!(neighbors, vec![
            LatticePoint::<2>::new([4, 1]),
            LatticePoint::<2>::new([5, 1]),
            LatticePoint::<2>::new([6, 1]),

            LatticePoint::<2>::new([4, 2]),
            LatticePoint::<2>::new([6, 2]),

            LatticePoint::<2>::new([4, 3]),
            LatticePoint::<2>::new([5, 3]),
            LatticePoint::<2>::new([6, 3])
        ]);

        assert_eq!(neighbors_and_self, vec![
            LatticePoint::<2>::new([4, 1]),
            LatticePoint::<2>::new([5, 1]),
            LatticePoint::<2>::new([6, 1]),

            LatticePoint::<2>::new([4, 2]),
            LatticePoint::<2>::new([5, 2]),
            LatticePoint::<2>::new([6, 2]),

            LatticePoint::<2>::new([4, 3]),
            LatticePoint::<2>::new([5, 3]),
            LatticePoint::<2>::new([6, 3])
        ]);
    }

    #[test]
    fn vertex_neighborhood_iterate() {
        let point = RealPoint::<3>::new([1.2, -5.9, 8.0]);
        let vertices: Vec<RealPoint<3>> = point.vertex_neighborhood().collect();

        assert_eq!(vertices, vec![
            RealPoint::<3>::new([1.0, -6.0, 8.0]),
            RealPoint::<3>::new([2.0, -6.0, 8.0]),
            RealPoint::<3>::new([1.0, -5.0, 8.0]),
            RealPoint::<3>::new([2.0, -5.0, 8.0]),
            RealPoint::<3>::new([1.0, -6.0, 9.0]),
            RealPoint::<3>::new([2.0, -6.0, 9.0]),
            RealPoint::<3>::new([1.0, -5.0, 9.0]),
            RealPoint::<3>::new([2.0, -5.0, 9.0])
        ]);
    }
}
