use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    size: usize,
    dimensions: Vec<f64>,
}

impl Vector {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            dimensions: vec![0.; size],
        }
    }

    // Todo:
    // #[allow(dead_code)]
    // pub fn cross_product(&self, rhs: Self) {

    // }
}

impl<T: Into<f64>> From<Vec<T>> for Vector {
    fn from(vector: Vec<T>) -> Self {
        Self {
            size: vector.len(),
            dimensions: vector.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector: {:?}", self.dimensions)
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Access an out of range index of Vector");
        }

        &self.dimensions[index]
    }
}
impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!("Access an out of range index of Vector");
        }

        &mut self.dimensions[index]
    }
}

// Add
// @desc: Add two vector, you know
impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Adding vectors with different dimension number");
        }
        let mut result: Self::Output = Vector::new(self.size);

        for i in 0..self.size {
            result[i] = self[i] + rhs[i];
        }

        result
    }
}
// Add
// @desc: Add two vector but the second vector are being negative, you know
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Subtracting two vectors with different dimension number");
        }
        let mut result: Self::Output = Vector::new(self.size);

        for i in 0..self.size {
            result[i] = self[i] - rhs[i];
        }

        result
    }
}

// Dot Product
// @desc: This return sum of all product of two side dimensions
impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Multiplying two vectors with different dimension number");
        }
        let mut result: Self::Output = 0.;

        for i in 0..self.size {
            result += self[i] * rhs[i];
        }

        result
    }
}

// Scalar Product
// @desc: This make: 10*vector and vector*10 available
impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for i in 0..result.size {
            result[i] *= f64::from(rhs);
        }
        result
    }
}
impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

// Scalar Division
// @desc: just divide it bro
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for i in 0..result.size {
            result[i] /= f64::from(rhs);
        }
        result
    }
}
impl Div<Vector> for f64 {
    type Output = Vector;
    fn div(self, rhs: Vector) -> Self::Output {
        let mut result = rhs.clone();
        for i in 0..result.size {
            result[i] = self / result[i];
        }
        result
    }
}