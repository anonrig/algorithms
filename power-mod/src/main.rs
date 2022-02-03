fn powerMod(b: i64, e: i64, modulus: i64) -> i64 {
  if modulus == 1 {
    return 0;
  }

  let mut result = 1;
  let mut exponent = e;
  let mut base = b % modulus;

  while exponent > 0 {
    if exponent % 2 == 1 {
      result = (result * base) % modulus;
    }
    exponent = exponent >> 1;
    base = (base * base) % modulus;
  }

  return result;
}

fn main() {
  println!("answer for mod 35 is {:?}", powerMod(4, 1536, 35) - powerMod(9, 4824, 35));
  println!("answer for mod 37 is {:?}", powerMod(4, 1536, 37) - powerMod(9, 4824, 37));
}
