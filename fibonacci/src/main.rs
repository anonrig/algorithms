fn recursive_fibonacci(n: u32) -> u32 {
  return match n {
    0 => 0,
    1 => 1,
    _ => recursive_fibonacci(n-1) + recursive_fibonacci(n-2)
  };
}

fn iterative_fibonacci(n: u32) -> u32 {
  return match n {
    0 => 0,
    1 => 1,
    _ => {
      let mut lhs: u32 = 0;
      let mut rhs: u32 = 1;
      let mut sum: u32 = 1;

      for _i in 1..n {
        sum = lhs + rhs;
        lhs = rhs;
        rhs = sum;
      }

      return sum;
    }
  }
}

fn closed_form_fibonacci(n: f64) -> i32 {
  let lhs = ((1.0 + (5 as f64).sqrt()) / 2.0).powf(n);
  let rhs = ((1.0 - (5 as f64).sqrt()) / 2.0).powf(n);

  return ((1.0 / (5 as f64).sqrt()) * (lhs - rhs)) as i32;
}

fn main() {
  println!("Recursive fibonacci result {}", recursive_fibonacci(20));
  println!("Iterative fibonacci result {}", iterative_fibonacci(20));
  println!("Closed form fibonacci result {}", closed_form_fibonacci(20.0));
}
