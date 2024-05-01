use std::io::stdin;

fn main(){
  let ABXY = input();
  println!("{:?}", ABXY[3]-(ABXY[3]-ABXY[2])/(ABXY[1]-ABXY[0])*ABXY[1]); 
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

