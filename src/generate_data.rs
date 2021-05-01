/// generate {0, 1} arrays which length is "length"
/// 2021-05-02 0:00 note: type changed grb(usize) -> Vec<f64> into grb(usize) -> Vec<u128>
pub fn generate_random_bits(length: usize) -> Vec<bool> {
  (0..length).map(|_| rand::random::<bool>()).collect()
}

/// generate arrays which length is "length" and the each element follows N(0,1) independently
pub fn generate_noises(length: usize) -> Vec<f64> {
  use rand::thread_rng;
  let mut rng = thread_rng();
  (0..length).map(|_| use_distr(&mut rng, 0.0, 1.0)).collect()
}

/// generate a random element following N(0, 1)
fn use_distr<R: rand::Rng>(rng: &mut R, mu: f64, sigma2: f64) -> f64 {
  use rand_distr::{Normal, Distribution};
  let normal = Normal::new(mu, sigma2.sqrt()).unwrap();
  normal.sample(rng)
}

/// generate arrays which length is "length" and the each elements has signal of psk(= 2 or 4.pow(n))
/// "length" does NOT means sum of bits length, this means the length of vector simply.
/// 2021-05-01 15:00 note: type changed grnqs(usize -> usize -> Vec<(f64, f64)>) into grnqs(usize -> usize -> Vec<Complex>)
use super::complex::Complex;
use super::cp;
pub fn generate_random_n_qam_signal(length: usize, psk: usize) -> Vec<Complex> {
  let mut checker = psk.clone();
  let mut n_psk = 0;
  while checker%2 == 0 && checker != 0 {
    checker >>= 1;
    n_psk += 1;
  }

  if checker != 1 || (n_psk != 1 && n_psk % 2 != 0) {
    panic!("psk size must be 2 or 4.pow(n)")
  }else{
    if n_psk == 1 {
      generate_random_bits(length).iter()
        .map(|&l| cp!(if l {1.0} else {-1.0}))
        .collect::<Vec<Complex>>()
    }else{
      n_psk >>= 1;
      let inphase = (0..length).map(|_| generate_random_n_bits(n_psk));
      let quadrature = (0..length).map(|_| generate_random_n_bits(n_psk));
      inphase.zip(quadrature)
        .map(|(l, r)| cp!((convert_random_n_bits_into_power(l, n_psk)) + (convert_random_n_bits_into_power(r, n_psk)) i))
        .collect::<Vec<Complex>>()
    }
  }
}

pub fn qam_mapping(vec: Vec<bool>, psk: usize) -> Vec<Complex> {
  let mut checker = psk.clone();
  let mut n_psk = 0;
  while checker%2 == 0 && checker != 0 {
    checker >>= 1;
    n_psk += 1;
  }

  if checker != 1 || (n_psk != 1 && n_psk % 2 != 0) {
    panic!("psk size must be 2 or 4.pow(n)")
  }else{
    if n_psk == 1 {
      vec.iter()
        .map(|&l| cp!(if l {1.0} else {-1.0}))
        .collect::<Vec<Complex>>()
    }else{
      n_psk >>= 1;
      let vec = convert_code_gray2binary(vec, n_psk);
      let vec: Vec<f64> = vec.iter()
        .map(|&c| convert_random_n_bits_into_power(c, n_psk)).collect();
      let mut ret = Vec::with_capacity(vec.len()/2);
      for i in 0..vec.len()/2{ 
        ret.push(cp!((vec[2*i]) + (vec[2*i+1])i));
      }
      ret
    }
  }
}

/// generate a number which has n_bits
fn generate_random_n_bits(n_bits: usize) -> u128 {
  if n_bits > 128 {
    panic!("tooooooooo looooooooooooong!!! use less than 4^128-qam!!!!!")
  }
  rand::random::<u128>() % (1 << n_bits)
}

/// convert a number which has n_bits into power
fn convert_random_n_bits_into_power(number: u128, bits_length: usize) -> f64 {
  if bits_length > 128 {
    panic!("tooooooooo looooooooooooong!!! use less than 4^128-qam!!!!!")
  }else{
    let bits_length = (1 << bits_length) as f64;
    let number = number as f64;
    (3.0 / (2.0 * (bits_length.powf(2.0) - 1.0) as f64)).sqrt() * (bits_length - 1.0 - 2.0 * number)
  }
}

fn convert_code_gray2binary(vec: Vec<bool>, bits_length: usize) -> Vec<u128> {
  if bits_length > 128{
    panic!("tooooooooo looooooooooooong!!! use less than 4^128-qam!!!!!")
  }else if bits_length == 0 {
    panic!("why do you do that?  thers's meaningless.")
  }else{
    // let modulo = 1 << bits_length;
    if vec.len() % bits_length != 0 {
      panic!("vec length '{}' must be by factor of bits_length '{}'", vec.len(), bits_length);
    }else{
      let mut v = vec.iter();
      let mut ret = Vec::with_capacity(vec.len()/bits_length);
      for _ in 0..vec.len()/bits_length {
        let mut num = 0;
        let mut c = 0;
        for i in 0..bits_length {
          num |= (if *v.next().unwrap() {1}else{0} * (1<<i)) ^ (c << 1);
          c = num & (1<<i);
        }
        ret.push(num);
      }
      ret
    }
  }
}

pub fn cpa(mut vec: Vec<Complex>, copylength: usize) -> Vec<Complex> {
  if vec.len() <= copylength {
    panic!("copylength must be under the vector's length: {}", vec.len());
  }else{
    let mut ret = vec.iter().take(copylength).map(|&c| c).collect::<Vec<Complex>>();
    ret.append(&mut vec);
    ret
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn generate_random_bits_test() {
    let bits = super::generate_random_bits(10);
    assert!(bits.len() == 10);
    let check_number = bits.iter().map(|&e| e == true || e == false).fold(true, |l, r| l & r);
    assert!(check_number);
  }

  #[test]
  fn generate_noises_test() {
    let noises = super::generate_noises(10);
    assert!(noises.len() == 10);
  }

  #[test]
  fn generate_random_bpsk() {
    use super::Complex;
    let bits = super::generate_random_n_qam_signal(10, 2);
    assert!(bits.len() == 10);
    let check_number = bits.iter().map(|&cpx| cpx == super::cp!((1.0) + (0.0)i) || cpx == super::cp!((-1.0) + (0.0)i)).fold(true, |l, r| l & r);
    assert!(check_number);
  }

  #[test]
  fn generate_random_n_qsk() {
    let bits = super::generate_random_n_qam_signal(10, 4);
    assert!(bits.len() == 10);
  }

  #[test]
  fn generate_random_n_bits() {
    for _ in 0..1000 {
      let n_bits = 1;
      let number = super::generate_random_n_bits(n_bits);
      assert!(number < (1 << n_bits))
    }
  }

  #[test]
  #[should_panic]
  fn generate_random_n_bits_should_panic() {
    super::generate_random_n_bits(129);
  }

  #[test]
  fn convert_bits_into_power() {
    let bits_length: usize = 4;
    let m: u64 = 1 << bits_length;
    let max_number = (3.0 / (2.0 * (m.pow(2) - 1) as f64)).sqrt() * (m - 1) as f64;
    for number in 0..(1<<bits_length) {
      let power = super::convert_random_n_bits_into_power(number, bits_length);
      assert!(max_number - power.abs() >= 0.0, "max: {}, power: {}", max_number, power)
    }
  }

  #[test]
  fn cpa_test() {
    let bits = super::generate_random_n_qam_signal(10, 2);
    let vec = super::cpa(bits, 5);
    assert_eq!(vec.len(), 10 + 5);
  }

  #[test]
  fn convert_code_gray2binary_test() {
    let length = 16;
    let bit_pair = 4;
    let gray = super::generate_random_bits(length);
    let bin = super::convert_code_gray2binary(gray, bit_pair);
    assert_eq!(bin.len(), length/bit_pair);
  }
}