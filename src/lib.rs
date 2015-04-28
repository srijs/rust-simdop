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

pub type M2<N> = Twice<N>;
pub type M4<N> = Twice<M2<N>>;
pub type M8<N> = Twice<M4<N>>;
pub type M16<N> = Twice<M8<N>>;
pub type M32<N> = Twice<M16<N>>;

pub trait Set1<M: Multi> {
  fn set1(&self, M::Elem) -> M;
}

pub trait Add<M: Multi> {
  fn add(&self, M, M) -> M;
}

pub trait Shli<M: Multi> {
  fn shli(&self, M, i32) -> M;
}

pub trait Shri<M: Multi> {
  fn shri(&self, M, i32) -> M;
}

pub trait Mullo<M: Multi + ElemTwice> {
  fn mullo(&self, M, M) -> M::ElemTwice;
}

pub mod arch {

  enum Token { Token }

  pub struct X86(Token);

  pub mod x86;

}
