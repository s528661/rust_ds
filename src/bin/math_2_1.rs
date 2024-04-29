use std::io::stdin;

fn main() {
  let NM = input_X();
  let A = input_X();
  let B = input_X();
  
  for i in 0..NM[0] {
    for j in 0..NM[1] {
        if A[i] == B[j] {
            println!("{:?}", A[i]);
        }
    }
  }
}

fn input_X()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
