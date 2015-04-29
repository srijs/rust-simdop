extern crate simdty;
extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl Multi for M8<i16> {
  type Elem = i16;
  type Repr = simdty::i16x8;
#[inline(always)]
  fn wrap(s: Self::Repr) -> Self {
    Twice{
      lo: Twice{
        lo: Twice{lo: s.0, hi: s.1},
        hi: Twice{lo: s.2, hi: s.3},
      },
      hi: Twice{
        lo: Twice{lo: s.4, hi: s.5},
        hi: Twice{lo: s.6, hi: s.7}
      }
    }
  }
#[inline(always)]
  fn unwrap(self) -> Self::Repr {
    simdty::i16x8(
      self.lo.lo.lo, self.lo.lo.hi, self.lo.hi.lo, self.lo.hi.hi,
      self.hi.lo.lo, self.hi.lo.hi, self.hi.hi.lo, self.hi.hi.hi
    )
  }
}

impl Set1<M8<i16>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i16) -> M8<i16> {
    Twice{
      lo: Twice{
        lo: Twice{lo: a, hi: a},
        hi: Twice{lo: a, hi: a}
      },
      hi: Twice{
        lo: Twice{lo: a, hi: a},
        hi: Twice{lo: a, hi: a}
      }
    }
  }
}

impl Add<M8<i16>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl Shli<M8<i16>> for SSE2 {
#[inline(always)]
  fn shli(&self, a: M8<i16>, i: i32) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pslli_w(a.unwrap(), i))
    }
  }
}

impl Shri<M8<i16>> for SSE2 {
#[inline(always)]
  fn shri(&self, a: M8<i16>, i: i32) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_psrli_w(a.unwrap(), i))
    }
  }
}
