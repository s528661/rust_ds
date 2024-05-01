use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();
    let sum: usize = A.iter().sum();
    let mut avg: f32 = sum as f32 / N as f32;
    println!("{:?}", &avg);
}

fn input_n()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
