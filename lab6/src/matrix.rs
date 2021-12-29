use rand::distributions::{Distribution, Uniform};
use std::ops::{Index, IndexMut};

type Element = f64;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    n: usize,
    values: Vec<Element>,
}

impl Matrix {
    pub fn new(n: usize, values: Vec<Element>) -> Self {
        assert!(n != 0, "invalid dimension");
        assert_eq!(n * n, values.len(), "invalid dimension");
        Self { n, values }
    }

    pub fn from_value(n: usize, value: Element) -> Self {
        assert!(n != 0, "invalid dimension");
        Self { n, values: vec![value; n * n] }
    }

    pub fn zeroes(n: usize) -> Self {
        Self::from_value(n, 0f64)
    }

    pub fn ones(n: usize) -> Self {
        Self::from_value(n, 1f64)
    }

    pub fn eye(n: usize, value: Element) -> Self {
        let mut m = Self::zeroes(n);
        for i in 0..n {
            m[(i, i)] = value;
        }

        m
    }

    pub fn identity(n: usize) -> Self {
        Self::eye(n, 1f64)
    }

    pub fn random(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::new_inclusive(-1f64, 1f64);

        let mut values = Vec::with_capacity(n * n);
        for _ in 0..values.capacity() {
            values.push(between.sample(&mut rng));
        }

        Self::new(n, values)
    }

    pub fn multiply(a: &Self, b: &Self) -> Self {
        assert_eq!(a.n, b.n);
        let mut c = Self::zeroes(a.n);

        for i in 0..(c.n) {
            for k in 0..(c.n) {
                let loc = a[(i, k)];
                for j in 0..(c.n) {
                    c[(i, j)] += loc * b[(k, j)];
                }
            }
        }

        c
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Element;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.values[i * self.n + j]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.values[i * self.n + j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let m = Matrix::new(2, vec![1f64, 2f64, 3f64, 4f64]);
        assert_eq!(m.n, 2);
        assert_eq!(m.values, vec![1f64, 2f64, 3f64, 4f64]);
    }

    #[test]
    fn builds_from_value() {
        let m = Matrix::from_value(2, 2f64);
        assert_eq!(m.n, 2);
        assert_eq!(m.values, vec![2f64, 2f64, 2f64, 2f64]);
    }

    #[test]
    fn builds_zeroes() {
        let m = Matrix::zeroes(2);
        assert_eq!(m[(0, 0)], 0f64);
        assert_eq!(m[(0, 1)], 0f64);
        assert_eq!(m[(1, 0)], 0f64);
        assert_eq!(m[(1, 1)], 0f64);
    }

    #[test]
    fn builds_ones() {
        let m = Matrix::ones(2);
        assert_eq!(m[(0, 0)], 1f64);
        assert_eq!(m[(0, 1)], 1f64);
        assert_eq!(m[(1, 0)], 1f64);
        assert_eq!(m[(1, 1)], 1f64);
    }

    #[test]
    fn builds_eye() {
        let m = Matrix::eye(2, 2f64);
        assert_eq!(m[(0, 0)], 2f64);
        assert_eq!(m[(0, 1)], 0f64);
        assert_eq!(m[(1, 0)], 0f64);
        assert_eq!(m[(1, 1)], 2f64);
    }

    #[test]
    fn builds_identity() {
        let m = Matrix::identity(2);
        assert_eq!(m[(0, 0)], 1f64);
        assert_eq!(m[(0, 1)], 0f64);
        assert_eq!(m[(1, 0)], 0f64);
        assert_eq!(m[(1, 1)], 1f64);
    }

    #[test]
    fn builds_random() {
        let m = Matrix::random(2);
        assert!(m[(0, 0)] <= 1f64 && m[(0, 0)] >= -1f64);
        assert!(m[(0, 1)] <= 1f64 && m[(0, 0)] >= -1f64);
        assert!(m[(1, 0)] <= 1f64 && m[(0, 0)] >= -1f64);
        assert!(m[(1, 1)] <= 1f64 && m[(0, 0)] >= -1f64);
    }

    #[test]
    fn multiplies() {
        let a = Matrix::random(2);
        let b = Matrix::random(2);
        let c = Matrix::multiply(&a, &b);

        assert_eq!(c[(0, 0)], a[(0, 0)] * b[(0, 0)] + a[(0, 1)] * b[(1, 0)]);
        assert_eq!(c[(0, 1)], a[(0, 0)] * b[(0, 1)] + a[(0, 1)] * b[(1, 1)]);
        assert_eq!(c[(1, 0)], a[(1, 0)] * b[(0, 0)] + a[(1, 1)] * b[(1, 0)]);
        assert_eq!(c[(1, 1)], a[(1, 0)] * b[(0, 1)] + a[(1, 1)] * b[(1, 1)]);
    }

    #[test]
    fn indexes() {
        let mut m = Matrix::new(2, vec![1f64, 2f64, 3f64, 4f64]);
        assert_eq!(m[(0, 0)], 1f64);
        assert_eq!(m[(1, 0)], 3f64);

        m[(1, 0)] = 5f64;
        assert_eq!(m[(1, 0)], 5f64);
    }
}
