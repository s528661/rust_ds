use std::io::stdin;

fn main() {
    let x = input_x();
    // let S = input_str();
    // let s2i: i32 = S[0].parse().unwrap();
}

fn input_x()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input_str()->Vec<String>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

fn input_X()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
