use std::io::stdin;

fn main(){
  let NXr = input();
  
  let N: u64 = NXr[0] as u64;
  let X: u64 = NXr[1] as u64;
  let r: u64 = NXr[2] as u64;

  let MOD: u64 = 1e9 as u64;

  let mut r_n: u64 = 1;
  for _i in 0..N {
    r_n = (r_n * r) % MOD;
  }
  println!("{:?}", X * (r_n - 1 + MOD) % MOD); 
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
