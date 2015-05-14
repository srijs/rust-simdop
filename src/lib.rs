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
//! Elements are layed out as leaves in depth-first traversal order.
//! This construction makes structural modifications very easy,
//! while providing strong type guarantees, e.g. when splitting or joining vectors.
//!
//! To perform SIMD operation, the tree is flattened into an array, and restored afterwards.
//! Because the data layout of the elements is equivalent for the tree and the array
//! representations, this can be done without any overhead.

extern crate simdty;

/// Core data structures and traits.
pub mod core {

  #[derive(Debug, Copy, Clone)]
  #[repr(C, packed)]
  pub struct Twice<N> {
    pub lo: N,
    pub hi: N
  }

  pub trait Multi {
    type Elem;
    type Repr;
    type Array;
    fn wrap(Self::Repr) -> Self;
    fn unwrap(self) -> Self::Repr;
    fn serialize(self) -> Self::Array;
    fn unserialize(Self::Array) -> Self;
  }

  pub trait Half {
    type Half;
  }

  impl<N> Half for Twice<N> {
    type Half = N;
  }

  pub trait Widen {
    type Widen;
  }

  impl<T: Widen> Widen for Twice<T> {
    type Widen = Twice<T::Widen>;
  }

  impl Widen for Twice<i16> {
    type Widen = i32;
  }

  impl Widen for Twice<i32> {
    type Widen = i64;
  }

  pub trait Narrow {
    type Narrow;
  }

  impl<T: Narrow> Narrow for Twice<T> {
    type Narrow = Twice<T::Narrow>;
  }

  impl Narrow for Twice<i16> {
    type Narrow = Twice<Twice<i8>>;
  }

  impl Narrow for Twice<i32> {
    type Narrow = Twice<Twice<i16>>;
  }

}

use std::mem;
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

macro_rules! multi {
  ($m:ty, $e:ty, $r:ty, $a:ty) => {
    impl Multi for $m {
      type Elem = $e;
      type Repr = $r;
      type Array = $a;
#[inline(always)]
      fn wrap(s: Self::Repr) -> Self {
        unsafe { mem::transmute(s) }
      }
#[inline(always)]
      fn unwrap(self) -> Self::Repr {
        unsafe { mem::transmute(self) }
      }
#[inline(always)]
      fn unserialize(s: Self::Array) -> Self {
        unsafe { mem::transmute(s) }
      }
#[inline(always)]
      fn serialize(self) -> Self::Array {
        unsafe { mem::transmute(self) }
      }
    }
  }
}

multi!(M2<i64>, i64, simdty::i64x2, [i64;2]);
multi!(M4<i32>, i32, simdty::i32x4, [i32;4]);
multi!(M8<i16>, i16, simdty::i16x8, [i16;8]);
multi!(M16<i8>,  i8, simdty::i8x16, [i8;16]);

#[macro_export]
macro_rules! mvec {
  ($($y:expr),*) => {
    $crate::core::Multi::unserialize([$($y),*])
  };
}

/// The `CmpEq` trait is used to specify equality comparison funtionality.
pub trait CmpEq<M: Multi> {
/// Compares elements in `a` and `b` for equality.
  fn cmpeq(&self, a: M, b: M) -> M;
}

/// The `CmpGt` trait is used to specify greater-than comparison funtionality.
pub trait CmpGt<M: Multi> {
/// Compares elements in `a` and `b` for greater-than.
  fn cmpgt(&self, a: M, b: M) -> M;
}

/// The `CmpLt` trait is used to specify less-than comparison funtionality.
pub trait CmpLt<M: Multi> {
/// Compares elements in `a` and `b` for less-than.
  fn cmplt(&self, a: M, b: M) -> M;
}

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

/// The `AddS` trait is used to specify element-wise saturated addition functionality.
pub trait AddS<M: Multi> {
/// Adds elements in a and b using saturation.
  fn adds(&self, a: M, b: M) -> M;
}

/// The `MAdd` trait is used to specify multiply-add functionality.
pub trait MAdd<M: Multi + Widen> {
/// Multiplies elements from `a` and `b`, producing intermediate elements of twice the size. Horizontally adds adjacent pairs of intermediate elements.
  fn madd(&self, a: M, b: M) -> M::Widen;
}

/// The `Min` trait is used to specify minimum comparison functionality.
pub trait Min<M: Multi> {
/// Compares elements in `a` and `b`, and finds the minimum values.
  fn min(&self, a: M, b: M) -> M;
}

/// The `Max` trait is used to specify maximum comparison functionality.
pub trait Max<M: Multi> {
/// Compares elements in `a` and `b`, and finds the maximum values.
  fn max(&self, a: M, b: M) -> M;
}

/// The `Shli` trait is used to specify immediate bit-wise left shift functionality.
pub trait Shli<M: Multi> {
/// Shifts all elements in the `a` left by `i` bits.
  fn shli(&self, a: M, i: i32) -> M;
}

/// The `Shri` trait is used to specify immediate bit-wise right shift functionality.
pub trait Shri<M: Multi> {
/// Shifts all elements in the `a` right by `i` bits.
  fn shri(&self, a: M, i32) -> M;
}

/// The `Mullo` trait is used to specify low element-wise multiplication functionality.
pub trait Mullo<M: Multi> {
/// Multiplies the elements in `a` and `b` and stores the lower halves of the results.
  fn mullo(&self, a: M, b: M) -> M;
}

/// The `Mulhi` trait is used to specify high element-wise multiplication functionality.
pub trait Mulhi<M: Multi> {
/// Multiplies the elements in `a` and `b` and stores the higher halves of the results.
  fn mulhi(&self, a: M, b: M) -> M;
}

/// The `Abs` trait is used to specify absolute value computation functionality.
pub trait Abs<M: Multi> {
/// Computes the absolute value of the elements in `a`
  fn abs(&self, a: M) -> M;
}

/// The `HAdd` trait is used to specify horizontal addition functionality.
pub trait HAdd<M: Multi> {
/// Horizontally adds adjacent pairs of elements in `a` and `b`.
  fn hadd(&self, a: M, b: M) -> M;
}

/// The `HSub` trait is used to specify horizontal subtraction functionality.
pub trait HSub<M: Multi> {
/// Horizontally subtracts adjacent pairs of elements in `a` and `b`.
  fn hsub(&self, a: M, b: M) -> M;
}

/// The `Sign` trait is used to specify negation functionality.
pub trait Sign<M: Multi> {
/// Negates elements in `a` if the corresponding element in `b` is negative.
/// Zeroes out the element if the corresponding element in `b` is zero.
  fn sign(&self, a: M, b: M) -> M;
}

/// The `PackS` trait is used to specify signed element narrowing functionality.
pub trait PackS<M: Multi + Narrow> {
/// Narrows elements from `a` and `b` using signed saturation.
  fn packs(&self, a: M, b: M) -> M::Narrow;
}

/// The `PackUS` trait is used to specify unsigned element narrowing functionality.
pub trait PackUS<M: Multi + Narrow> {
/// Narrows elements from `a` and `b` using unsigned saturation.
  fn packus(&self, a: M, b: M) -> M::Narrow;
}

/// CPU identification and feature detection, as well as trait implementations.
pub mod arch {

  enum Token { Token }

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  pub mod x86;

#[cfg(target_arch = "arm")]
  pub mod arm;

}
