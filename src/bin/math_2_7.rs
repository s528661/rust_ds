use std::io::stdin;

fn main() {
  let NXY = input_xy();
  let A = input_xy();
  let B = input_xy();
  let mut resV = Vec::<usize>::new();
  
  resV.extend_from_slice(&A);
  resV.extend_from_slice(&B);

  resV.sort();
  resV.dedup();

  println!("{:?}", NXY[0] - resV.len())
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
