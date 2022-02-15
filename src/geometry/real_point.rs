use std::fmt;
use std::mem;
use std::ops::*;
use std::slice;

use super::{DistanceMetric, EuclideanMetric, LatticePoint, VertexNeighborhood};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct RealPoint<const DIM: usize> {
    coordinates: [f64; DIM]
}

impl<const DIM: usize> RealPoint<DIM> {
    pub fn new<T: Into<f64>>(coordinates: [T; DIM]) -> Self {
        Self { coordinates: coordinates.map(|c| c.into()) }
    }

    pub fn origin() -> Self {
        Self { coordinates: [0.0_f64; DIM] }
    }

    pub fn diagonal<T: Into<f64>>(scalar: T) -> Self {
        Self { coordinates: [scalar.into(); DIM] }
    }

    pub fn iter(&self) -> slice::Iter<'_, f64> {
        self.coordinates.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, f64> {
        self.coordinates.iter_mut()
    }

    pub fn vertex_neighborhood(self) -> VertexNeighborhood<DIM> {
        VertexNeighborhood::<DIM>::new(self)
    }

    pub fn abs(&self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.abs()) }
    }

    pub fn floor(self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.floor()) }
    }

    pub fn ceil(self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.ceil()) }
    }

    pub fn round(self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.round()) }
    }

    pub fn trunc(self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.trunc()) }
    }

    pub fn fract(self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.fract()) }
    }

    pub fn powi(self, exp: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.powi(exp)) }
    }

    pub fn powf(self, exp: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.powf(exp)) }
    }

    pub fn is_finite(self) -> bool {
        self.iter().all(|c| c.is_finite())
    }

    pub fn sum(self) -> f64 {
        self.iter().fold(0.0_f64, |acc, &elem| acc + elem)
    }

    pub fn dot_product(self, rhs: Self) -> f64 {
        self.mul(rhs).sum()
    }

    pub fn magnitude<Metric: DistanceMetric>(self) -> f64 {
        Metric::real_magnitude::<DIM>(self)
    }

    pub fn normalize(self) -> Self {
        self.div(self.magnitude::<EuclideanMetric>())
    }

    pub fn to_lattice_point(self) -> LatticePoint<DIM> {
        LatticePoint::<DIM>::new(self.coordinates.map(|c| c.floor() as i32))
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&a, (&b, &c))) in self.iter().zip(a.iter().zip(b.iter())).enumerate() {
            coordinates[idx] = a.mul_add(b, c);
        };

        Self { coordinates }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = &self.coordinates as *const f64;
        let num_bytes = DIM * mem::size_of::<f64>();

        unsafe { slice::from_raw_parts(ptr as *const u8, num_bytes) }
    }
}

impl<const DIM: usize> fmt::Display for RealPoint<DIM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut coordinates = self.iter();

        write!(f, "(")?;
        write!(f, "{:?}", coordinates.next().unwrap())?;

        for coord in coordinates {
            write!(f, ", {:?}", coord)?;
        };

        write!(f, ")")
    }
}

 //-------------------------------------------------------------------------------------------------
// From

impl<const DIM: usize> From<[f64; DIM]> for RealPoint<DIM> {
    fn from(coordinates: [f64; DIM]) -> Self {
        Self { coordinates }
    }
}

 //-------------------------------------------------------------------------------------------------
// Index/IndexMut

impl<const DIM: usize> Index<usize> for RealPoint<DIM> {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.coordinates[idx]
    }
}

impl<const DIM: usize> IndexMut<usize> for RealPoint<DIM> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.coordinates[idx]
    }
}

 //-------------------------------------------------------------------------------------------------
// Add/AddAssign

impl<const DIM: usize> Add for RealPoint<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs + rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Add<f64> for RealPoint<DIM> {
    type Output = Self;

    fn add(self, scalar: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c + scalar) }
    }
}

impl<const DIM: usize> AddAssign for RealPoint<DIM> {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs += rhs;
        };
    }
}

impl<const DIM: usize> AddAssign<f64> for RealPoint<DIM> {
    fn add_assign(&mut self, scalar: f64) {
        for coord in self.iter_mut() {
            *coord += scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Sub/SubAssign

impl<const DIM: usize> Sub for RealPoint<DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs - rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Sub<f64> for RealPoint<DIM> {
    type Output = Self;

    fn sub(self, scalar: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c - scalar) }
    }
}

impl<const DIM: usize> SubAssign for RealPoint<DIM> {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        };
    }
}

impl<const DIM: usize> SubAssign<f64> for RealPoint<DIM> {
    fn sub_assign(&mut self, scalar: f64) {
        for coord in self.iter_mut() {
            *coord -= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Mul/MulAssign

impl<const DIM: usize> Mul for RealPoint<DIM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs * rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Mul<f64> for RealPoint<DIM> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c * scalar) }
    }
}

impl<const DIM: usize> MulAssign for RealPoint<DIM> {
    fn mul_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs *= rhs;
        };
    }
}

impl<const DIM: usize> MulAssign<f64> for RealPoint<DIM> {
    fn mul_assign(&mut self, scalar: f64) {
        for coord in self.iter_mut() {
            *coord *= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Div/DivAssign

impl<const DIM: usize> Div for RealPoint<DIM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs / rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Div<f64> for RealPoint<DIM> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c / scalar) }
    }
}

impl<const DIM: usize> DivAssign for RealPoint<DIM> {
    fn div_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs /= rhs;
        };
    }
}

impl<const DIM: usize> DivAssign<f64> for RealPoint<DIM> {
    fn div_assign(&mut self, scalar: f64) {
        for coord in self.iter_mut() {
            *coord /= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Rem/RemAssign

impl<const DIM: usize> Rem for RealPoint<DIM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let mut coordinates = [0.0_f64; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs % rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Rem<f64> for RealPoint<DIM> {
    type Output = Self;

    fn rem(self, scalar: f64) -> Self {
        Self { coordinates: self.coordinates.map(|c| c % scalar) }
    }
}

impl<const DIM: usize> RemAssign for RealPoint<DIM> {
    fn rem_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs %= rhs;
        };
    }
}

impl<const DIM: usize> RemAssign<f64> for RealPoint<DIM> {
    fn rem_assign(&mut self, scalar: f64) {
        for coord in self.iter_mut() {
            *coord %= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod test {
    use super::RealPoint;

    #[test]
    fn add() {
        let mut lhs = RealPoint::<3>::new([3.5, 4.125, 5.25]);
        let rhs = RealPoint::<3>::new([9.5, 1.25, 20.0]);

        let result = RealPoint::<3>::new([13.0, 5.375, 25.25]);

        assert_eq!(lhs + rhs, result);

        lhs += rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn sub() {
        let mut lhs = RealPoint::<3>::new([3.5, 4.125, 5.25]);
        let rhs = RealPoint::<3>::new([9.5, 1.25, 20.0]);

        let result = RealPoint::<3>::new([-6.0, 2.875, -14.75]);

        assert_eq!(lhs - rhs, result);

        lhs -= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn mul() {
        let mut lhs = RealPoint::<3>::new([2.5, 9.0, 10.875]);
        let rhs = RealPoint::<3>::new([1.25, 12.0, 0.125]);

        let result = RealPoint::<3>::new([3.125, 108.0, 1.359375]);

        assert_eq!(lhs * rhs, result);

        lhs *= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn div() {
        let mut lhs = RealPoint::<3>::new([2.5, 9.0, 10.875]);
        let rhs = RealPoint::<3>::new([1.25, 12.0, 0.125]);

        let result = RealPoint::<3>::new([2.0, 0.75, 87.0]);

        assert_eq!(lhs / rhs, result);

        lhs /= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn rem() {
        let mut lhs = RealPoint::<3>::new([11.5, 15.0, 8.125]);
        let rhs = RealPoint::<3>::new([7.5, 7.5, 1.75]);

        let result = RealPoint::<3>::new([4.0, 0.0, 1.125]);

        assert_eq!(lhs % rhs, result);

        lhs %= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn dot_product() {
        let lhs = RealPoint::<4>::new([2.5, 3.125, 1.125, 5.25]);
        let rhs = RealPoint::<4>::new([10.0, -4.0, 8.0, 2.75]);

        // 25.0 - 12.5 + 9.0 + 14.4375
        assert_eq!(lhs.dot_product(rhs), 35.9375);
    }
}
