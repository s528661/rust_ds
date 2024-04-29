use std::io::stdin;

fn main() {
    let N = input_x();
    let mut sum = 0;

    for i in 1..N {
        for j in i+1..N+1 {
            sum += i * j;
        }
    }

    println!("{:?}", sum);
}

fn input_x()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
