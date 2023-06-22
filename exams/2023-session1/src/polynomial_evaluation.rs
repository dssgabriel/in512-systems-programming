//! # Exercise 1 - Polynomial evaluation (5 pts)
//!
//! The goal of this exercise is to write a Rust struct that evaluates polynomial equations of the
//! form:
//! $$
//! P(x) = a_n x^n + ... + a_2 x^2 + a_1 x + a_0
//! $$
//!
//! We start with the following struct:
//! ```rust
//! struct Polynomial {
//!     coefs: Vec<f64>,
//! }
//! ```
//! which stores the coefficients as double precision floating-point values into a vector.
//! The i-th index of the `Vec` corresponds to the coefficient $a_i$.
//!
//! 1. Implement a function [`new(coefs: &Vec<f64>)`](Polynomial::new) which builds an instance of
//! [`Polynomial`](Polynomial) with its coefficients passed as parameters. Do not change the type of
//! the argument.
//! > Author's note: there is no good reason to pass a reference to a vector rather than a slice
//! > reference. This is dumb and not idiomatic Rust. [Passing borrowed arguments is the prefered
//! way](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)
//! > as it is more flexible and behaves virtually the same. This correction proposal does not
//! > respect this bit of the exercise's instructions.
//!
//! 2. Implement the function [`deg2(a2: f64, a1: f64, a0: f64)`](Polynomial::deg2) which builds a
//! second-degree [`Polynomial`](Polynomial). For example, `deg2(4.0, -1.0, 3.0)` builds an instance
//! of the second-degree polynomial $4x^2 - x + 3$.
//!
//! 3. Implement the associated function [`eval(&self, x: f64)`](Polynomial::eval) which evaluates
//! the polynomial equation in $x$ and returns a `f64`.   
//! In this first implementation, we compute the exponents using [`std::f64::powi`](f64::powi),
//! which returns the n-th power of a double precision floating-point number, e.g.:
//! ```rust
//! assert_eq!(2.0_f64.powi(3), 8.0);
//! ```
//!
//! 4. The use of [`powi`](f64::powi) to compute the exponents is inefficient, propose a more
//! optimized version of [`eval`](Polynomial::eval), within a new function [`eval_opt`](Polynomial::eval_opt).
//!
//! 5. Justify the optimizations made. Explain how they improve the runtime and energy efficiency.

/// Represents a polynomial equation of arbitrary degree.
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    /// Stores the coefficients in relation to their degree, i.e. `coefs[i]` corresponds to
    /// $a_i x^i$.
    pub coefs: Vec<f64>,
}

impl Polynomial {
    /// Creates a `Polynomial` given  a list of coefficients.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::polynomial_evaluation::Polynomial;
    /// let p = Polynomial::new(&[3.0, -1.0, 4.0]);
    /// assert_eq!(p, Polynomial { coefs: vec![3.0, -1.0, 4.0] });
    /// ```
    pub fn new(coefs: &[f64]) -> Self {
        Self {
            coefs: coefs.clone().into(),
        }
    }

    /// Creates a `Polynomial` of degree 2.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::polynomial_evaluation::Polynomial;
    /// let p = Polynomial::deg2(4.0, -1.0, 3.0);
    /// assert_eq!(p, Polynomial { coefs: vec![3.0, -1.0, 4.0] });
    /// ```
    pub fn deg2(a2: f64, a1: f64, a0: f64) -> Self {
        Self {
            coefs: vec![a0, a1, a2],
        }
    }

    /// Evaluates a `Polynomial` given a value of `x`, using [`powi`](f64::powi).
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::polynomial_evaluation::Polynomial;
    /// let p = Polynomial::deg2(4.0, -1.0, 3.0);
    /// assert_eq!(p.eval(8.0), 251.0);
    /// ```
    pub fn eval(&self, x: f64) -> f64 {
        self.coefs
            .iter()
            .enumerate()
            .fold(0.0, |acc, (n, c)| acc + c * x.powi(n as i32))
    }

    /// Evaluates a `Polynomial` given a value of `x`, in a more optimized way.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::polynomial_evaluation::Polynomial;
    /// let p = Polynomial::deg2(4.0, -1.0, 3.0);
    /// assert_eq!(p.eval_opt(8.0), 251.0);
    /// ```
    ///
    /// # Answer to question 5:
    /// The [`powi`](f64::powi) function is a very expensive operation. In this implementation, we
    /// remove the need for an explicit exponent calculation by gradually "building" it at each
    /// iteration.
    pub fn eval_opt(&self, x: f64) -> f64 {
        let mut n = 1.0;
        self.coefs.iter().fold(0.0, |acc, c| {
            let partial = acc + c * n;
            n *= x;
            partial
        })
    }

    /// Variant of [`eval_opt`](Polynomial::eval_opt) using a raw `for` loop instead of iterators.
    ///
    /// From the benchmarks' results the performance is identical, i.e. same code generation (on
    /// `rustc 1.72.0-nightly`).   
    /// For more details, we would need to have a look at the generated assembly.
    pub fn eval_opt_variant(&self, x: f64) -> f64 {
        let mut n = 1.0;
        let mut result = 0.0;
        for c in self.coefs.iter() {
            result += c * n;
            n *= x;
        }
        result
    }
}

/// This test submodule demonstrates the performance improvements of [`eval_opt`](Polynomial::eval_opt)
/// over [`eval`](Polynomial::eval) using Rust's nightly [test::Bencher](test::Bencher) benchmarking
/// manager.
#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use nanorand::{Rng, WyRand};
    use test::Bencher;

    #[bench]
    fn bench_eval(b: &mut Bencher) {
        let mut rng = WyRand::new();
        let mut coefs = vec![0.0; 10_000];
        rng.fill(&mut coefs);

        let p = Polynomial::new(&coefs);
        b.iter(|| {
            let x = rng.generate::<f64>();
            test::black_box(p.eval(x));
        })
    }

    #[bench]
    fn bench_eval_opt(b: &mut Bencher) {
        let mut rng = WyRand::new();
        let mut coefs = vec![0.0; 10_000];
        rng.fill(&mut coefs);

        let p = Polynomial::new(&coefs);
        b.iter(|| {
            let x = rng.generate::<f64>();
            test::black_box(p.eval_opt(x));
        })
    }

    #[bench]
    fn bench_eval_opt_variant(b: &mut Bencher) {
        let mut rng = WyRand::new();
        let mut coefs = vec![0.0; 10_000];
        rng.fill(&mut coefs);

        let p = Polynomial::new(&coefs);
        b.iter(|| {
            let x = rng.generate::<f64>();
            test::black_box(p.eval_opt_variant(x));
        })
    }
}
