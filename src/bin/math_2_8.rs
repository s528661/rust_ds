use std::io::stdin;

fn main() {
  let N = input();
  let mut cnt = 0;
  
  println!("{:?}", N - (N / 3) - (N / 5) + (N / 15));
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
