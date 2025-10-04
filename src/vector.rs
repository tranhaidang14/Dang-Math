use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    size: usize,
    dimensions: Vec<f64>
}

impl Vector {
    pub fn new(size: usize) -> Self {
        Self { size, dimensions: vec![0.; size] }
    }

    // Todo:
    #[allow(dead_code)]
    pub fn cross_product(&self, rhs: Self) {

    }
}

impl<T: Into<f64>> From<Vec<T>> for Vector {
    fn from(vector: Vec<T>) -> Self {
        Self { size: vector.len(), dimensions: vector.into_iter().map(Into::into).collect() }
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

// Scalar multiplication
impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();

        for i in 0..result.size {
            result[i] *= rhs;
        }

        result
    }
}

// Dot product
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

// Scalar
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();

        for i in 0..result.size {
            result[i] /= rhs;
        }

        result
    }
}