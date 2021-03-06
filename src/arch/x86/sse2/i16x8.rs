extern crate llvmint;

use super::super::super::super::*;
use super::super::super::super::core::*;
use super::super::SSE2;

impl CmpEq<M8<i16>> for SSE2 {
#[inline(always)]
  fn cmpeq(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() == b.unwrap())
  }
}

impl CmpGt<M8<i16>> for SSE2 {
#[inline(always)]
  fn cmpgt(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() > b.unwrap())
  }
}

impl CmpLt<M8<i16>> for SSE2 {
#[inline(always)]
  fn cmplt(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() < b.unwrap())
  }
}

impl Set1<M8<i16>> for SSE2 {
#[inline(always)]
  fn set1(&self, a: i16) -> M8<i16> {
    mvec![a,a,a,a,a,a,a,a]
  }
}

impl Add<M8<i16>> for SSE2 {
#[inline(always)]
  fn add(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() + b.unwrap())
  }
}

impl AddS<M8<i16>> for SSE2 {
#[inline(always)]
  fn adds(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_padds_w(a.unwrap(), b.unwrap()))
    }
  }
}

impl MAdd<M8<i16>> for SSE2 {
#[inline(always)]
  fn madd(&self, a: M8<i16>, b: M8<i16>) -> M4<i32> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pmadd_wd(a.unwrap(), b.unwrap()))
    }
  }
}

impl Min<M8<i16>> for SSE2 {
#[inline(always)]
  fn min(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pmins_w(a.unwrap(), b.unwrap()))
    }
  }
}

impl Max<M8<i16>> for SSE2 {
#[inline(always)]
  fn max(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pmaxs_w(a.unwrap(), b.unwrap()))
    }
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

impl Mullo<M8<i16>> for SSE2 {
#[inline(always)]
  fn mullo(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    Multi::wrap(a.unwrap() * b.unwrap())
  }
}

impl Mulhi<M8<i16>> for SSE2 {
#[inline(always)]
  fn mulhi(&self, a: M8<i16>, b: M8<i16>) -> M8<i16> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_pmulh_w(a.unwrap(), b.unwrap()))
    }
  }
}

impl PackS<M8<i16>> for SSE2 {
#[inline(always)]
  fn packs(&self, a: M8<i16>, b: M8<i16>) -> M16<i8> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_packsswb_128(a.unwrap(), b.unwrap()))
    }
  }
}

impl PackUS<M8<i16>> for SSE2 {
#[inline(always)]
  fn packus(&self, a: M8<i16>, b: M8<i16>) -> M16<i8> {
    unsafe {
      Multi::wrap(llvmint::x86::sse2_packuswb_128(a.unwrap(), b.unwrap()))
    }
  }
}
