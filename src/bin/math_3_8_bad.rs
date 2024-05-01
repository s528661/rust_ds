use std::io::stdin;

fn main(){
  let NXr = input();
  
  let N: u32 = NXr[0] as u32;
  let X: u32 = NXr[1] as u32;
  let r: u32 = NXr[2] as u32;

  let mut r_n: u32 = 1;
  for _i in 0..N {
    r_n = (r_n * r) % (1e9 as u32);
  }
  println!("{:?}", X * (r_n - 1 + (1e9 as u32)) % (1e9 as u32)); 
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
