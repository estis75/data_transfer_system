#[derive(Copy, Clone)]
pub struct Complex {
  real: f64,
  imaginary: f64,
}

/// macroに関しての話
/// 
/// parseされるタイミングがおそいせいでsuper::Complexとかにすると, 使われる場所次第でsuper::super::Complexになったりする.
/// やばい
/// 
#[allow(unused)]
#[macro_export]
macro_rules! cp {
  ( ($r:expr) + ($c:expr) i ) => {
    Complex::new($r, $c)
  };
  ( $r:expr ) => {
    Complex::new($r, 0.)
  };
  ( ($c:expr)i) => {
    Complex::new(0., $c)
  }; 
}

impl Complex {
  pub fn new(real: f64, imaginary: f64) -> Complex {
    Complex{ real, imaginary }
  }

  pub fn zero() -> Complex {
    cp!((0.) + (0.) i)
  }
}

use std::ops::{Add, Sub, Mul, Div, Neg};
impl Add for Complex {
  type Output=Self;
  fn add(self, other: Self) -> Self {
    Self{
      real: self.real + other.real,
      imaginary: self.imaginary + other.imaginary
    }
  }
}
impl Sub for Complex {
  type Output=Self;
  fn sub(self, other: Self) -> Self {
    Self{
      real: self.real - other.real,
      imaginary: self.imaginary - other.imaginary
    }
  }
}
impl Mul for Complex {
  type Output=Self;
  fn mul(self, other: Self) -> Self {
    Self{
      real: self.real*other.real - self.imaginary*other.imaginary,
      imaginary: self.real*other.imaginary + self.imaginary*other.real
    }
  }
}
impl Complex {
  pub fn complement(self) -> Self {
    Self {
      real: self.real,
      imaginary: -self.imaginary
    }
  }

  pub fn abs(self) -> f64 {
    (self * self.complement()).real
  }

  pub fn mulr(self, other: f64) -> Self {
    Self{
      real: self.real * other,
      imaginary: self.imaginary * other
    }
  }
  
  pub fn divr(self, other: f64) -> Self {
    Self{
      real: self.real / other,
      imaginary: self.imaginary / other
    }
  }
}

impl Div for Complex {
  type Output=Self;
  fn div(self, other: Self) -> Self {
    if other.abs() == 0.0 {
      panic!("invalid divisor {:?}", other)
    }

    (self * other.complement()).divr(other.abs())
  }
}
impl Neg for Complex {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Complex::zero() - self
  }
}

impl Complex {
  pub fn exp(self) -> Self {
    Self{
      real: self.imaginary.cos(),
      imaginary: self.imaginary.sin()
    }.mulr(self.real.exp())
  }
}

impl std::cmp::PartialEq<Complex> for Complex {
  fn eq(&self, other: &Self) -> bool {
    self.real == other.real && self.imaginary == other.imaginary
  }
}

impl std::fmt::Debug for Complex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Complex")
      .field("real", &self.real)
      .field("imaginary", &self.imaginary)
      .finish()
  }
}

#[cfg(test)]
mod test{
  use super::Complex;
  #[test]
  fn macro_test() {
    assert_eq!(cp!((3.)+(2.)i), Complex::new(3., 2.));
    assert_ne!(cp!((4.)+(3.)i), Complex::new(3., 2.));
  }

  #[test]
  #[allow(unused)]
  fn calculation_test(){
    assert_eq!(cp!((3.)+(2.)i).complement(), Complex::new(3., -2.));
    assert_eq!(cp!((2.) + (3.)i) + cp!((4.)+(3.)i), Complex::new(6., 6.));
    assert_eq!(cp!((2.) + (3.)i) - cp!((4.)+(3.)i), Complex::new(-2., 0.));
    assert_eq!(cp!((2.) + (3.)i) * cp!((4.)+(3.)i), Complex::new(-1., 18.));
    assert_eq!(cp!((2.) + (3.)i) / cp!((4.)+(3.)i), Complex::new(17., 6.).divr(cp!((4.)+(3.)i).abs()));
  }

  #[test]
  #[should_panic]
  #[allow(unused)]
  fn calculation_test_panic(){
    cp!((2.) + (3.)i) / cp!((0.)+(0.)i);
  }

}