extern crate simdty;
extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl Multi for M4<i32> {
  type Elem = i32;
  type Repr = simdty::i32x4;
#[inline(always)]
  fn wrap(s: Self::Repr) -> Self {
    Twice{
      lo: Twice{lo: s.0, hi: s.1},
      hi: Twice{lo: s.2, hi: s.3}
    }
  }
#[inline(always)]
  fn unwrap(self) -> Self::Repr {
    simdty::i32x4(self.lo.lo, self.lo.hi, self.hi.lo, self.hi.hi)
  }
}

impl Set1<M4<i32>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i32) -> M4<i32> {
    Twice{
      lo: Twice{lo: a, hi: a},
      hi: Twice{lo: a, hi: a}
    }
  }
}

impl Add<M4<i32>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl Shli<M4<i32>> for SSE2 {
#[inline(always)]
  fn shli(&self, a: M4<i32>, i: i32) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_d(a.unwrap(), i))
    }
  }
}

impl Shri<M4<i32>> for SSE2 {
#[inline(always)]
  fn shri(&self, a: M4<i32>, i: i32) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_d(a.unwrap(), i))
    }
  }
}
