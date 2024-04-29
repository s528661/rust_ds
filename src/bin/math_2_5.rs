use std::io::stdin;

fn main() {
    let NM = input_xy();
    let A = input_xy();
    let B = input_xy();
    let mut AB = Vec::<usize>::new();
    let mut cnt = 0;
    let mut pre_v = 0;

    AB.extend_from_slice(&A);
    AB.extend_from_slice(&B);

    AB.sort();
    AB.push(0);

    for v in AB {
        if pre_v != v {
            if cnt == 1 {
                println!("{:?}", pre_v);
            }
            cnt = 0;
        }
        pre_v = v;
        cnt += 1;
    }
}

fn input_xy()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
