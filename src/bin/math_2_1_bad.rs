use std::io::stdin;
use std::collections::HashSet;

fn main() {
  let NM = input_X();
  let A = input_X();
  let B = input_X();
  
  let result = common_elements(&A, &B);

  for rst in result {
    println!("{:?}", rst);
  }
}

fn input_X()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

fn common_elements(x: &[usize], y: &[usize]) -> Vec<usize> {
    let set_x: HashSet<_> = x.iter().cloned().collect();
    let set_y: HashSet<_> = y.iter().cloned().collect();
    let common_elements: HashSet<_> = set_x.intersection(&set_y).cloned().collect();
    let result: Vec<_> = common_elements.into_iter().collect();
    result
}
