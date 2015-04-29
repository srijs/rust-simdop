extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::NEON;

impl Set1<M4<i32>> for NEON {
#[inline(always)]
  fn set1(&self, a: i32) -> M4<i32> {
    Twice{
      lo: Twice{lo: a, hi: a},
      hi: Twice{lo: a, hi: a}
    }
  }
}

impl Add<M4<i32>> for NEON {
#[inline(always)]
  fn add(&self, a: M4<i32>, b: M4<i32>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::arm::neon_vpadd_v4i32(a.unwrap(), b.unwrap()))
    }
  }
}
