use std::io::stdin;

fn main(){
    let N = input();
    let mut sum = 0;
    for i in 1..N+1 {
        sum += i;
    }
    println!("{:?}", sum);
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
