use std::io::stdin;

fn main(){
  let mut total = 1;

  for i in 0..21 {
    println!("{:?}", total);
    total *= 2;
  }
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
