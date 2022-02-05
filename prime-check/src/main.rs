use std::ops::Sub;
use std::str::FromStr;
use num_bigint::{BigUint, RandomBits, ToBigUint};
use num_traits::{Zero, One};
use rand::Rng;

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
  if a < b {
    gcd(b, a)
  } else if a % b == BigUint::zero() {
    b.to_biguint().unwrap()
  } else {
    gcd(b, &(a % b))
  }
}

fn is_prime(n: BigUint, k: u128) -> bool {
  let mut size = k;
  let mut rng = rand::thread_rng();

  while size > 0 {
    let random: BigUint = rng.sample(RandomBits::new(256));
    let a: BigUint = 2.to_biguint().unwrap() * random % n.clone();

    if gcd(&n , &a) != BigUint::one() {
      return false;
    }


    if a.modpow(&n.clone().sub(BigUint::one()), &n) != BigUint::one() {
      return false;
    }

    size -= 1;
  }

  true
}

fn main() {
  println!("10106665597294733930808268834911 {:?}", is_prime(BigUint::from_str("10106665597294733930808268834911").unwrap(), 128));
  println!("557940830126698960967415391 {:?}", is_prime(BigUint::from_str("557940830126698960967415391").unwrap(), 128));
  println!("2305567963945518424753102147331756071 {:?}", is_prime(BigUint::from_str("2305567963945518424753102147331756071").unwrap(), 128));
  println!("169665573205075467667167 {:?}", is_prime(BigUint::from_str("169665573205075467667167").unwrap(), 128));
}
