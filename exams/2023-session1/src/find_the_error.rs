//! # Exercise 2 - Find the error (5 pts)
//!
//! 1. The functions [`a`], [`b`], [`c`] and the struct [`D`] do not compile. For each: (1) explain
//! the issue and (2) propose a correction. Minimize the modifications made to the base code and do
//! not remove any existing functionality (e.g. do not remove entire lines of code).
//!
//! 2. Using the corrected [`D`] struct, write a code that initializes a linked list of three pairs,
//! such as: $(1, 2) \rightarrow (3, 4) \rightarrow (5, 6)$.

/// Computes the sum of a `Vec` of `i32`s.
///
/// # Solution(s)
/// * Append `.iter()` to `values` in the raw `for` loop.
/// * Use `values.iter().sum()`.
pub fn a() {
    let values = vec![1, 2, 3];
    let mut sum = 0;
    // Iterating directly over `values` consumes the vector by implictly moving it into the loop
    // through `into_iter()`.
    for x in values.iter() {
        sum += x;
    }
    println!("{} {:?}", sum, values); // Here, `values` is not valid anymore if consumed.
}

/// Prints a given [`String`] twice, as shared references.
///
/// # Solution
/// Remove the mutable borrows as they are not needed to call [`println!`].
/// We can also remove the `mut` on `s` as we don't modify the string.
pub fn b(/* mut */ s: String) {
    let r1 = &/* mut */s;
    let r2 = &/* mut */s;
    // It is not possible to mutably borrow a variable more than once at a time.
    println!("{}, {}", r1, r2);
}

/// Copies a [`String`] into another.
///
/// # Solution
/// Use the [`.to_string`](str::to_string) method on `y` instead of dereferencing it.
pub fn c(x: &mut String, y: &mut String) {
    // Cannot move the memory held by `y` into `x` because `String` does not implement the `Copy`
    // trait.
    *x = y.to_string();
}

/// Represents some kind of linked list holding tuples generic over a type `T`.
///
/// # Solution
/// Add a layer of indirection to avoid infinite recursion. In this case, we use
/// `Option<Box<T>>`.
#[allow(dead_code)]
pub struct D<T> {
    value: (T, T),
    // Recursion without indirection, needs to be a pointer (e.g. `Box<_>`) to `D<T>` instead.
    next: Option<Box<D<T>>>,
}

/// Initializes a linked list of three pairs using the fixed [`D`] struct.
pub fn d() {
    let _list = D {
        value: (1, 2),
        next: Some(Box::new(D {
            value: (3, 4),
            next: Some(Box::new(D {
                value: (5, 6),
                next: None,
            })),
        })),
    };
}
