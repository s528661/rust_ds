use std::io::stdin;

fn main() {
    let NX = input_xy();
    let mut cnt = 0;

    for i in 1..NX[0]+1 {
        if i % NX[1] == 0 { cnt += 1; }
    }

    println!("{:?}", cnt);
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
