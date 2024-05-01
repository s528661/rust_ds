use std::io::stdin;

fn main(){
  let N = input();
  let a0: usize = 1 as usize;
  let aN: usize = 2 as usize;

  println!("{:?}", aN.pow(N as u32)-a0);
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
