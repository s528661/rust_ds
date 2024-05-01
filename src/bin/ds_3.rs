use std::io::stdin;

fn main(){
    let N = input_n();
    let mut A = input();

    A.sort();
    
    if A.len() % 2 == 0 {
      println!("{:?}", (A[A.len()/2-1] as f32 + A[A.len()/2] as f32)/2.0);
    } else {
      println!("{:?}", A[A.len()/2]);
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
