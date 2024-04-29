use std::io::stdin;

fn main() {
    let NXY = input_xy();
    let mut cnt = 0;

    for i in 1..NXY[0]+1 {
        if (i % NXY[1] == 0) && (i % NXY[2] == 0) {
            cnt += 1; 
        }
    }

    println!("{:?}", cnt);
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
