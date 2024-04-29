use std::io::stdin;

fn main() {
    let LR = input();
    let L: i32 = LR[0].parse().unwrap();
    let R: i32 = LR[1].parse().unwrap();
    let mut sum = 0;
    for i in L..R+1 {
        let element: i32 = pow(2*i-1);
        sum += element;
    }
    println!("{:?}", sum);
}

fn input()->Vec<String>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

fn pow(x: i32) -> i32 {
    return x*x;
}
