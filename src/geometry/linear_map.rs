use super::RealPoint;

#[derive(Clone, PartialEq, Debug)]
pub struct LinearMap<const DIM: usize> {
    rows: [RealPoint<DIM>; DIM]
}

impl<const DIM: usize> LinearMap<DIM> {
    pub fn new<T: Into<RealPoint<DIM>>>(rows: [T; DIM]) -> Self {
        Self { rows: rows.map(|row| row.into()) }
    }

    pub fn apply(&self, point: RealPoint<DIM>) -> RealPoint<DIM> {
        RealPoint::<DIM>::new(self.rows.map(|row| row.dot_product(point)))
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::RealPoint;
    use super::LinearMap;

    #[test]
    fn apply() {
        let point = RealPoint::<2>::new([-1.0, 2.0]);
        let linear_map = LinearMap::<2>::new([
            [ 1.0, 3.0],
            [-2.0, 0.0]
        ]);

        let result = RealPoint::<2>::new([5.0, 2.0]);

        assert_eq!(linear_map.apply(point), result);
    }
}
