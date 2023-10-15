//! # Exercise 3 - More polynomials (10 pts)
//!
//! In this exercise, we will represent polynomials using the following code snippet:
//! ```rust
//! type Degree = u32;
//! type Coef = f64;
//! type Link = Option<Box<Term>>;
//!
//! pub struct Polynomial {
//!     coefs: Link,
//! }
//!
//! struct Term {
//!     coef: (Degree, Coef),
//!     next: Link,
//! }
//! ```
//!
//! With this implementation, only the non-null coefficients are represented.
//!
//! 1. What are the differences between `cargo build` and `cargo build --release`?
//!
//! 2. What is the goal of the `clippy` CLI tool?
//!
//! 3. What is the usecase of the Rust type [`Option<T>`](Option)? What does `T` represent?
//!
//! 4. What are the differences between [`Option<T>`](Option) and [`Result<T, E>`](Result)?
//!
//! 5. Give the Rust code that declares a variable instantiating the polynomial $x^2 - 4$.
//!
//! 6. Implement [`zero()`](Polynomial::zero) which builds the null [`Polynomial`](Polynomial).
//!
//! 7. Implement [`push(&mut self, coef: Coef, degree: Degree)`](Polynomial::push) which adds a new
//!    term to a polynomial.
//!    If the term of degree `degree` already exists, its coefficient is replaced by `coef`.
//!
//! 8. Implement [`eval(&self, x: f64)`](Polynomial::eval) which evaluates the polynomial in `x` and
//!    returns the result as a `f64`.
//!
//! 9. Give the Rust code of a unit test that checks the expected behavior of the [`eval`](Polynomial::eval)
//!    method for the polynomial $x^2 - 4$ in $x = 8$.
//!
//! 10. Implement the trait [`std::ops::Index<Degree>`](std::ops::Index) for the [`Polynomial`](Polynomial)
//!     struct so that we can retrieve a coefficient given a degree, e.g. given the polynomial
//!     $p(x) = x^2 - 2$, we have: `p[0] == -2`, `p[1] == 0` and `p[2] == 1`.
//!
//! See the README.md file for answers to questions 1-5.

/// Represents the degree of a polynomial's coefficient.
type Degree = u32;

/// Represents a polynomial's coefficient.
type Coef = f64;

/// Represents a link to the next item in the linked list.
type Link = Option<Box<Term>>;

/// Represents a polynomial equation.
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    pub coefs: Link,
}

/// Represents a polynomial's term.
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub coef: (Degree, Coef),
    pub next: Link,
}

impl Polynomial {
    /// Constructs the null polynomial.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::more_polynomials::Polynomial;
    /// let p = Polynomial::zero();
    /// assert_eq!(p, Polynomial { coefs: None });
    /// ```
    pub fn zero() -> Self {
        Self { coefs: None }
    }

    /// Creates a new `Term` and pushes into the linked list.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::more_polynomials::{Polynomial, Term};
    /// let mut p1 = Polynomial::zero();
    /// p1.push(1.0, 2);
    /// p1.push(-4.0, 0);
    /// # let p2 = Polynomial {
    /// #     coefs: Some(Box::new(Term {
    /// #         coef: (0, -4.0),
    /// #         next: Some(Box::new(Term {
    /// #             coef: (2, 1.0),
    /// #             next: None,
    /// #         })),
    /// #     })),
    /// # };
    /// # for (t1, t2) in p1.iter().zip(p2.iter()) {
    /// #     assert_eq!(t1, t2);
    /// # }
    /// ```
    pub fn push(&mut self, coef: Coef, degree: Degree) {
        if self[degree] != 0.0 {
            let mut target = self.iter_mut().skip_while(|term| term.0 != degree);
            target.next().unwrap().1 = coef;
        } else {
            let new_term = Box::new(Term {
                coef: (degree, coef),
                next: self.coefs.take(),
            });
            self.coefs = Some(new_term);
        }
    }

    /// Evaluates the `Polynomial` in `x`.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::more_polynomials::Polynomial;
    /// let mut p = Polynomial::zero();
    /// p.push(1.0, 2);
    /// p.push(-4.0, 0);
    /// assert_eq!(p.eval(8.0), 60.0);
    /// ```
    pub fn eval(&self, x: f64) -> f64 {
        let mut result = 0.0;

        for (degree, coef) in self.iter() {
            result += coef * x.powi(*degree as i32);
        }

        result
    }

    /// Returns an `Iter` struct that implements the `Iterator` trait.
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.coefs.as_deref(),
        }
    }

    /// Returns an `IterMut` struct that implements the `Iterator` trait.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            next: self.coefs.as_deref_mut(),
        }
    }
}

/// Immutable iterator for [`Polynomial`](Polynomial) linked lists, created thanks to the
/// [`iter`](Polynomial::iter) method.
pub struct Iter<'a> {
    next: Option<&'a Term>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a (Degree, Coef);
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|term| {
            self.next = term.next.as_deref();
            &term.coef
        })
    }
}

/// Mutable iterator for [`Polynomial`](Polynomial) linked lists, created thanks to the
/// [`iter_mut`](Polynomial::iter) method.
pub struct IterMut<'a> {
    next: Option<&'a mut Term>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut (Degree, Coef);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|term| {
            self.next = term.next.as_deref_mut();
            &mut term.coef
        })
    }
}

impl std::ops::Index<Degree> for Polynomial {
    type Output = f64;

    /// Index into a `Polynomial` linked list using a `degree`. Returns the coefficient correponding
    /// to the given degree.
    ///
    /// # Example
    /// ```rust
    /// # use in512_exam_2023_session1::more_polynomials::Polynomial;
    /// let mut p = Polynomial::zero();
    /// p.push(1.0, 2);
    /// p.push(-2.0, 0);
    /// assert_eq!(p[2], 1.0);
    /// assert_eq!(p[1], 0.0);
    /// assert_eq!(p[0], -2.0);
    /// ```
    fn index(&self, index: Degree) -> &Self::Output {
        for (degree, coef) in self.iter() {
            if *degree == index {
                return coef;
            }
        }
        &0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_x_square_minus_four() {
        let mut p1 = Polynomial::zero();
        p1.push(1.0, 2);
        p1.push(-4.0, 0);
        let p2 = Polynomial {
            coefs: Some(Box::new(Term {
                coef: (0, -4.0),
                next: Some(Box::new(Term {
                    coef: (2, 1.0),
                    next: None,
                })),
            })),
        };
        for (t1, t2) in p1.iter().zip(p2.iter()) {
            assert_eq!(t1, t2);
        }
    }

    #[test]
    fn evaluate_x_square_minus_four_in_x_equals_eight() {
        let mut p = Polynomial::zero();
        p.push(1.0, 2);
        p.push(-4.0, 0);
        assert_eq!(p.eval(8.0), 60.0);
    }
}
