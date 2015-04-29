//! # A Library for Safe SIMD Operations
//!
//! The `simdop` library provides SIMD functionality
//! for building applications using fast operations
//! on homogeneous vector types.
//! It provides safety on a type-level using traits.
//!
//! ## CPU identification and feature detection
//!
//! In order to use a specific operations on a specific vector type,
//! you have to provide proof that the operation is valid on your architecture.
//! This is achieved by executing one or more detection functions, which return
//! an `Option<T>` type. You need to retrieve the wrapped value, which provides access
//! to the operations in the detected feature set in the form of trait implementations.
//!
//! ## Vector representation and encoding
//!
//! Since the number of elements in a vector is always a power of two,
//! vectors are represented as a perfect binary tree, which encodes the length
//! of the vector as a logarithm in the height of the tree.
//! This makes structural modifications very easy, while providing strong type guarantees,
//! e.g. when splitting or joining vectors.
//!
//! To perform SIMD operation, the tree is flattened into an array, and restored afterwards.
//! When combining vector operations, the compiler is able to perform "deforestation"
//! optimisations on the data structures, elminating most of the structural overhead. 

/// Core data structures and traits.
pub mod core {

  #[derive(Debug, Copy, Clone)]
  pub struct Twice<N> {
    pub lo: N,
    pub hi: N
  }

  pub trait Half {
    type Half;
  }

  impl<N> Half for Twice<N> {
    type Half = N;
  }

  pub trait ElemTwice {
    type ElemTwice;
  }

  impl ElemTwice for Twice<Twice<i32>> {
    type ElemTwice = Twice<i64>;
  }

  pub trait Multi {
    type Elem;
    type Repr;
    fn wrap(Self::Repr) -> Self;
    fn unwrap(self) -> Self::Repr;
  }

}

use self::core::*;

/// A vector of 2 elements of type `N`.
pub type M2<N> = Twice<N>;
/// A vector of 4 elements of type `N`.
pub type M4<N> = Twice<M2<N>>;
/// A vector of 8 elements of type `N`.
pub type M8<N> = Twice<M4<N>>;
/// A vector of 16 elements of type `N`.
pub type M16<N> = Twice<M8<N>>;
/// A vector of 32 elements of type `N`.
pub type M32<N> = Twice<M16<N>>;

/// The `Set1` trait is used to specify broadcasting functionality.
pub trait Set1<M: Multi> {
/// Broadcasts `e` to all elements of the vector.
  fn set1(&self, e: M::Elem) -> M;
}

/// The `Add` trait is used to specify element-wise addition functionality.
pub trait Add<M: Multi> {
/// Adds elements in a and b.
  fn add(&self, a: M, b: M) -> M;
}

/// The `Shli` trait is used to specify immediate bit-wise left shift functionality.
pub trait Shli<M: Multi> {
/// Shifts all elements in the `m` left by `i` bits.
  fn shli(&self, m: M, i: i32) -> M;
}

/// The `Shri` trait is used to specify immediate bit-wise right shift functionality.
pub trait Shri<M: Multi> {
/// Shifts all elements in the `m` right by `i` bits.
  fn shri(&self, M, i32) -> M;
}

/// The `Mullo` trait is used to specify low element-wise multiplication functionality.
pub trait Mullo<M: Multi + ElemTwice> {
/// Multiplies the elements in `a` and `b` and stores the lower halves of the results.
  fn mullo(&self, a: M, b: M) -> M::ElemTwice;
}

/// CPU identification and feature detection, as well as trait implementations.
pub mod arch {

  enum Token { Token }

  pub struct X86(Token);

  pub mod x86;

}
