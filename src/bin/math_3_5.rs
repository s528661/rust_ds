use std::io::stdin;

fn main(){
  let N = input();
  println!("{:?}", N*(N+1)*(2*N+1)/6); 
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
