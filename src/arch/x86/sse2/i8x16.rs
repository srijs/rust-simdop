extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl CmpEq<M16<i8>> for SSE2 {
#[inline(always)]
  fn cmpeq(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    Multi::wrap(a.unwrap() == b.unwrap())
  }
}

impl CmpGt<M16<i8>> for SSE2 {
#[inline(always)]
  fn cmpgt(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    Multi::wrap(a.unwrap() > b.unwrap())
  }
}

impl CmpLt<M16<i8>> for SSE2 {
#[inline(always)]
  fn cmplt(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    Multi::wrap(a.unwrap() < b.unwrap())
  }
}

impl Set1<M16<i8>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i8) -> M16<i8> {
    Twice{
      lo: Twice{
        lo: Twice{
          lo: Twice{lo: a, hi: a},
          hi: Twice{lo: a, hi: a}
        },
        hi: Twice{
          lo: Twice{lo: a, hi: a},
          hi: Twice{lo: a, hi: a}
        },
      },
      hi: Twice{
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
}

impl Add<M16<i8>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl AddS<M16<i8>> for SSE2 {
#[inline(always)]
  fn adds(&self, a: M16<i8>, b: M16<i8>) -> M16<i8> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_padds_b(a.unwrap(), b.unwrap()))
    }
  }
}
