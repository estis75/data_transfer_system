use super::complex::Complex;
use super::cp;

fn dft(vec: Vec<Complex>) -> Vec<Complex> {
  let length = vec.len();
  
  use std::f64::consts::PI;
  (0..length).map(|i| 
    vec.iter()
      .enumerate()
      .fold(Complex::zero(), |sum, (j, &a)| sum + a * Complex::new(0.,(i * j) as f64 / length as f64 * (2. * PI )).exp().divr((length as f64).sqrt()))
  ).collect::<Vec<Complex>>()
}

fn idft(vec: Vec<Complex>) -> Vec<Complex> {
  let length = vec.len();
  
  use std::f64::consts::PI;
  (0..length).map(|i| 
    vec.iter()
      .enumerate()
      .fold(Complex::zero(), |sum, (j, &a)| sum + a * cp!((0.) + ( (i * j) as f64 / length as f64 * (- 2. * PI )) i).exp()).divr((length as f64).sqrt())
  ).collect::<Vec<Complex>>()
}

/// 使うときはfftのサイズが2べきになって帰ってきちゃうので, 必要に応じて情報を潰してください
fn fft(mut vec: Vec<Complex>) -> Vec<Complex> {
  let mut length = 1;
  while vec.len() > length { length <<= 1; }

  vec.extend(vec![Complex::zero(); length-vec.len()]);
  fft_sub(vec).iter().map(|&c| c.divr((length as f64).sqrt())).collect()
}

fn fft_sub(vec: Vec<Complex>) -> Vec<Complex> {
  if vec.len() == 1 {
    vec
  }else{
    let even = fft_sub(vec.iter().step_by(2).map(|&c| c).collect());
    let odd = fft_sub(vec.iter().skip(1).step_by(2).map(|&c| c).collect());
    let mut half = even.iter().zip(odd.iter()).enumerate().map(|(i, (&l, &r))| l + r * cp!((2. * std::f64::consts::PI * i as f64 / vec.len() as f64) i).exp()).collect::<Vec<Complex>>();
    let mut af = even.iter().zip(odd.iter()).enumerate().map(|(i, (&l, &r))| l - r * cp!((2. * std::f64::consts::PI * i as f64 / vec.len() as f64) i).exp()).collect::<Vec<Complex>>();
    half.append(&mut af);
    half
  }
}

pub fn ifft(mut vec: Vec<Complex>) -> Vec<Complex> {
  let mut length = 1;
  while vec.len() > length { length <<= 1; }

  vec.extend(vec![Complex::zero(); length-vec.len()]);
  ifft_sub(vec).iter().map(|&c| c.divr((length as f64).sqrt())).collect()
}

fn ifft_sub(vec: Vec<Complex>) -> Vec<Complex> {
  if vec.len() == 1 {
    vec
  }else{
    let even = fft_sub(vec.iter().step_by(2).map(|&c| c).collect());
    let odd = fft_sub(vec.iter().skip(1).step_by(2).map(|&c| c).collect());
    let mut half = even.iter().zip(odd.iter()).enumerate().map(|(i, (&l, &r))| l + r * cp!((-2. * std::f64::consts::PI * i as f64 / vec.len() as f64) i).exp()).collect::<Vec<Complex>>();
    let mut af = even.iter().zip(odd.iter()).enumerate().map(|(i, (&l, &r))| l - r * cp!((-2. * std::f64::consts::PI * i as f64 / vec.len() as f64) i).exp()).collect::<Vec<Complex>>();
    half.append(&mut af);
    half
  }
}

#[cfg(test)]
mod test {
  use crate::cp;
  use super::*;
  #[test]
  fn dft_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i)];
    for (&l, &r) in v.clone().iter().zip(idft(dft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }
  #[test]
  fn fft_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i), cp!((4.)+(2.5)i)];
    for (&l, &r) in v.clone().iter().zip(idft(fft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }
  #[test]
  fn ifft_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i), cp!((4.)+(2.5)i)];
    for (&l, &r) in v.clone().iter().zip(ifft(dft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }
  #[test]
  fn fft_ifft_2pow_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i), cp!((4.)+(2.5)i)];
    for (&l, &r) in v.clone().iter().zip(ifft(fft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }

  #[test]
  fn fft_ifft_free_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i)];
    for (&l, &r) in v.clone().iter().zip(ifft(fft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }

  #[test]
  fn ifft_fft_free_test() {
    let v = vec![cp!((3.)+(2.)i), cp!((2.)+(4.)i), cp!((3.)+(1.)i)];
    for (&l, &r) in v.clone().iter().zip(fft(ifft(v)).iter()) {
      assert!((l - r).abs() < 0.000000001)
    }
  }
}