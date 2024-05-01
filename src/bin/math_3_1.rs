use std::io::stdin;

fn main(){
    let NX = input();
    let mut total = NX[1];

    for i in 0..NX[0] {
      println!("{:?}", total);
      total += 500;
    }
    println!("{:?}", total);
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

