use std::io::stdin;

fn main() {
    let NM = input_xy();
    let A = input_X();
    let B = input_X();

    let N = NM[0];
    let M = NM[1];

    let mut sum = 0;
    
    for i in 0..N {
        for j in 0..M {
            sum += A[i] + B[j];
        }
    }

    println!("{:?}", sum);
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
