use std::io::stdin;
use std::collections::HashMap;

fn main(){
    let _N = input_n();
    let A = input();

    let mut hist: HashMap<usize, i32> = HashMap::new();

    for &a in &A {
        let count = hist.entry(a).or_insert(0);
        *count += 1;
    }

    let mut max_k = 0;
    let mut max_v = 0;
    for (&k, &v) in &hist {
        if max_v < v {
            max_k = k;
            max_v = v;
        }
    }

    let mut max_hist_v: Vec<usize> = Vec::<usize>::new();
    for (&k, &v) in &hist {
        if &max_v == &v {
            max_hist_v.push(k);
        }
    }
    
    // max_hist_v.retain(|&x| x != 0);
    max_hist_v.sort();

    for h in &max_hist_v {
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
