use std::io::stdin;

fn main(){
  let NXd = input();
  println!("{:?}", NXd[0]*NXd[1] + (NXd[0]-1)*NXd[0]*NXd[2]/2); 
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

