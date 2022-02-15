use std::fmt;
use std::mem;
use std::ops::*;
use std::slice;

use super::{DistanceMetric, LatticeNeighborhood, RealPoint};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct LatticePoint<const DIM: usize> {
    coordinates: [i32; DIM]
}

impl<const DIM: usize> LatticePoint<DIM> {
    pub fn new<T: Into<i32>>(coordinates: [T; DIM]) -> Self {
        Self { coordinates: coordinates.map(|c| c.into()) }
    }

    pub fn origin() -> Self {
        Self { coordinates: [0_i32; DIM] }
    }

    pub fn diagonal<T: Into<i32>>(scalar: T) -> Self {
        Self { coordinates: [scalar.into(); DIM] }
    }

    pub fn iter(&self) -> slice::Iter<'_, i32> {
        self.coordinates.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, i32> {
        self.coordinates.iter_mut()
    }

    pub fn neighbors(self) -> LatticeNeighborhood<DIM> {
        LatticeNeighborhood::<DIM>::new(self, false)
    }

    pub fn neighbors_and_self(self) -> LatticeNeighborhood<DIM> {
        LatticeNeighborhood::<DIM>::new(self, true)
    }

    pub fn abs(&self) -> Self {
        Self { coordinates: self.coordinates.map(|c| c.abs()) }
    }

    pub fn sum(self) -> i32 {
        self.iter().fold(0_i32, |acc, &elem| acc + elem)
    }

    pub fn dot_product(self, rhs: Self) -> i32 {
        self.mul(rhs).sum()
    }

    pub fn magnitude<T: DistanceMetric>(self) -> i32 {
        T::lattice_magnitude::<DIM>(self)
    }

    pub fn to_real_point(self) -> RealPoint<DIM> {
        RealPoint::<DIM>::new(self.coordinates.map(|c| c as f64))
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = &self.coordinates as *const i32;
        let num_bytes = DIM * mem::size_of::<i32>();

        unsafe { slice::from_raw_parts(ptr as *const u8, num_bytes) }
    }
}

impl<const DIM: usize> fmt::Display for LatticePoint<DIM> {
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

impl<const DIM: usize> From<[i32; DIM]> for LatticePoint<DIM> {
    fn from(coordinates: [i32; DIM]) -> Self {
        Self { coordinates }
    }
}

 //-------------------------------------------------------------------------------------------------
// Index/IndexMut

impl<const DIM: usize> Index<usize> for LatticePoint<DIM> {
    type Output = i32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.coordinates[idx]
    }
}

impl<const DIM: usize> IndexMut<usize> for LatticePoint<DIM> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.coordinates[idx]
    }
}

 //-------------------------------------------------------------------------------------------------
// Add/AddAssign

impl<const DIM: usize> Add for LatticePoint<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coordinates = [0_i32; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs + rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Add<i32> for LatticePoint<DIM> {
    type Output = Self;

    fn add(self, scalar: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c + scalar) }
    }
}

impl<const DIM: usize> AddAssign for LatticePoint<DIM> {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs += rhs;
        };
    }
}

impl<const DIM: usize> AddAssign<i32> for LatticePoint<DIM> {
    fn add_assign(&mut self, scalar: i32) {
        for coord in self.iter_mut() {
            *coord += scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Sub/SubAssign

impl<const DIM: usize> Sub for LatticePoint<DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut coordinates = [0_i32; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs - rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Sub<i32> for LatticePoint<DIM> {
    type Output = Self;

    fn sub(self, scalar: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c - scalar) }
    }
}

impl<const DIM: usize> SubAssign for LatticePoint<DIM> {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        };
    }
}

impl<const DIM: usize> SubAssign<i32> for LatticePoint<DIM> {
    fn sub_assign(&mut self, scalar: i32) {
        for coord in self.iter_mut() {
            *coord -= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Mul/MulAssign

impl<const DIM: usize> Mul for LatticePoint<DIM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut coordinates = [0_i32; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs * rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Mul<i32> for LatticePoint<DIM> {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c * scalar) }
    }
}

impl<const DIM: usize> MulAssign for LatticePoint<DIM> {
    fn mul_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs *= rhs;
        };
    }
}

impl<const DIM: usize> MulAssign<i32> for LatticePoint<DIM> {
    fn mul_assign(&mut self, scalar: i32) {
        for coord in self.iter_mut() {
            *coord *= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Div/DivAssign

impl<const DIM: usize> Div for LatticePoint<DIM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut coordinates = [0_i32; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs / rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Div<i32> for LatticePoint<DIM> {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c / scalar) }
    }
}

impl<const DIM: usize> DivAssign for LatticePoint<DIM> {
    fn div_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs /= rhs;
        };
    }
}

impl<const DIM: usize> DivAssign<i32> for LatticePoint<DIM> {
    fn div_assign(&mut self, scalar: i32) {
        for coord in self.iter_mut() {
            *coord /= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Rem/RemAssign

impl<const DIM: usize> Rem for LatticePoint<DIM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let mut coordinates = [0_i32; DIM];

        for (idx, (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
            coordinates[idx] = lhs % rhs;
        };

        Self { coordinates }
    }
}

impl<const DIM: usize> Rem<i32> for LatticePoint<DIM> {
    type Output = Self;

    fn rem(self, scalar: i32) -> Self {
        Self { coordinates: self.coordinates.map(|c| c % scalar) }
    }
}

impl<const DIM: usize> RemAssign for LatticePoint<DIM> {
    fn rem_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
            *lhs %= rhs;
        };
    }
}

impl<const DIM: usize> RemAssign<i32> for LatticePoint<DIM> {
    fn rem_assign(&mut self, scalar: i32) {
        for coord in self.iter_mut() {
            *coord %= scalar;
        };
    }
}

 //-------------------------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod test {
    use super::LatticePoint;

    #[test]
    fn add() {
        let mut lhs = LatticePoint::<2>::new([15, 8]);
        let rhs = LatticePoint::<2>::new([5, 3]);

        let result = LatticePoint::<2>::new([20, 11]);

        assert_eq!(lhs + rhs, result);

        lhs += rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn sub() {
        let mut lhs = LatticePoint::<2>::new([15, 8]);
        let rhs = LatticePoint::<2>::new([5, 3]);

        let result = LatticePoint::<2>::new([10, 5]);

        assert_eq!(lhs - rhs, result);

        lhs -= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn mul() {
        let mut lhs = LatticePoint::<2>::new([15, 8]);
        let rhs = LatticePoint::<2>::new([5, 3]);

        let result = LatticePoint::<2>::new([75, 24]);

        assert_eq!(lhs * rhs, result);

        lhs *= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn div() {
        let mut lhs = LatticePoint::<2>::new([15, 8]);
        let rhs = LatticePoint::<2>::new([5, 3]);

        let result = LatticePoint::<2>::new([3, 2]);

        assert_eq!(lhs / rhs, result);

        lhs /= rhs;

        assert_eq!(lhs, result);
    }

    #[test]
    fn rem() {
        let mut lhs = LatticePoint::<2>::new([15, 8]);
        let rhs = LatticePoint::<2>::new([5, 3]);

        let result = LatticePoint::<2>::new([0, 2]);

        assert_eq!(lhs % rhs, result);

        lhs %= rhs;

        assert_eq!(lhs, result);
    }
}
