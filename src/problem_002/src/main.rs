fn main() {
  let mut fibs = fib(4_000_000);

  fibs.retain(|x| x % 2 == 0);

  let sum: i64 = fibs.iter().sum();

  println!("{:?}", sum);
}

fn fib(max: i64) -> Vec<i64> {
  let mut vec: Vec<i64> = vec![1, 2];
  let mut sum: i64 = vec[0] + vec[1];
  let mut index = 1;

  while sum < max {
     sum = vec[index] + vec[index - 1];
     index = index + 1;

     vec.push(sum);
  }

  vec.pop();

  vec
}
