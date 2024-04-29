use std::io::stdin;

fn main() {
  let NMK = input_xy();
  println!("{:?}", NMK[0]+NMK[1]-NMK[2]); 
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
