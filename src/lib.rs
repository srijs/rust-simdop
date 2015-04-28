#[derive(Debug, Copy, Clone)]
pub struct Twice<N>(N,N);

pub trait Half {
  type Half;
}

impl<N> Half for Twice<N> {
  type Half = N;
}

pub type M2<N> = Twice<N>;
pub type M4<N> = Twice<M2<N>>;
pub type M8<N> = Twice<M4<N>>;
pub type M16<N> = Twice<M8<N>>;
pub type M32<N> = Twice<M16<N>>;

pub trait ElemTwice {
  type ElemTwice;
}

impl ElemTwice for M4<i32> {
  type ElemTwice = M2<i64>;
}

pub trait Multi {
  type Elem;
  type Repr;
  fn wrap(Self::Repr) -> Self;
  fn unwrap(self) -> Self::Repr;
}

pub trait Set1: Multi {
  fn set1(Self::Elem) -> Self;
}

pub trait Add: Multi {
  fn add(self, Self) -> Self;
}

pub trait Shli: Multi {
  fn shli(self, i32) -> Self;
}

pub trait Shri: Multi {
  fn shri(self, i32) -> Self;
}

pub trait Mullo: Multi + ElemTwice {
  fn mullo(self, Self) -> Self::ElemTwice;
}

pub fn set1<M: Set1>(e: M::Elem) -> M {
  Set1::set1(e)
}

pub mod x86;
