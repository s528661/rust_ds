use std::io::stdin;

fn main() {
  let NM = input_X();
  let mut resV = Vec::<usize>::new();
  let A = input_X();
  let B = input_X();
  
  resV.extend_from_slice(&A);
  resV.extend_from_slice(&B);

  resV.sort();
  resV.dedup();
  
  for v in resV {
    println!("{:?}", v);
  }
}

fn input_X()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
