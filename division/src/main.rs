fn division(x: u32, y: u32) -> (u32, u32) {
  if x == 0 {
    return (0, 0);
  }

  let (mut q, mut r) = division(x >> 1, y);

  q *= 2;
  r *= 2;

  if x % 2 == 1 {
    r += 1;
  }

  if r >= y {
    r -= y;
    q += 1;
  }

  return (q, r);
}

fn main() {
  println!("Answer for 1/1 {:?}", division(1, 1));
  println!("Answer for 2/1 {:?}", division(2, 1));
  println!("Answer for 3/2 {:?}", division(3, 2));
}
