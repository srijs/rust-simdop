extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSSE3;

impl Abs<M8<i16>> for SSSE3 {
#[inline(always)]
  fn abs(&self, a: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_pabs_w_128(a.unwrap()))
    }
  }
}

impl HAdd<M8<i16>> for SSSE3 {
#[inline(always)]
  fn hadd(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_phadd_w_128(a.unwrap(), b.unwrap()))
    }
  }
}

impl HSub<M8<i16>> for SSSE3 {
#[inline(always)]
  fn hsub(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_phsub_w_128(a.unwrap(), b.unwrap()))
    }
  }
}

impl Sign<M8<i16>> for SSSE3 {
#[inline(always)]
  fn sign(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::ssse3_psign_w_128(a.unwrap(), b.unwrap()))
    }
  }
}
