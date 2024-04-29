use std::io::stdin;

fn main() {
    let N = input_x();
    let A = input_X();
    let mut sum = 1;

    for i in 0..N {
        sum = sum * A[i] % 10;
    }
    println!("{:?}", sum);
}

fn input_x()->usize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input_X()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
