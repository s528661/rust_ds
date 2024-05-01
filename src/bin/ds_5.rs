use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();
    let B = input();

    let sum_A: isize = A.iter().map(|&x| x.abs()).sum();
    let sum_B: isize = B.iter().map(|&x| x.abs()).sum();

    let sigma_A: f32 = sum_A as f32 / N as f32;
    let sigma_B: f32 = sum_B as f32 / N as f32;

    if sigma_A < sigma_B { 
        println!("A"); 
    } else if sigma_A > sigma_B { 
        println!("B"); 
    } else {
        println!("same"); 
    }
}

fn input_n()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input()->Vec<isize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
