use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();

    let mut hist = vec![0; 5];

    for a in A {
        if a <= 20 {
            hist[0] += 1
        } else if a <= 40 {
            hist[1] += 1
        } else if a <= 60 {
            hist[2] += 1
        } else if a <= 80 {
            hist[3] += 1
        } else {
            hist[4] += 1
        }
    }

    for h in hist {
        println!("{:?}", h);
    }
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


