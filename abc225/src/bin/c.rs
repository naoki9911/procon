use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut start_i: usize = 0;
    let mut start_j: usize = 0;
    {
        input! {
            bs: [usize; m],
        }
        start_i = (bs[0]-1)/7;
        start_j = (bs[0]-1)% 7;
        for idx in 1..m {
            if (bs[idx] - 1) % 7 !=  start_j + idx {
                println!("No");
                return;
            }
            if (bs[idx] - 1) / 7 != start_i {
                println!("No");
                return;
            }
        }
    }
    for i in 1..n {
        input! {
            bs: [usize; m]
        }
        for idx in 0..m {
            if (bs[idx] - 1)% 7 !=  start_j + idx {
                println!("No");
                return;
            }
            if (bs[idx] - 1)/ 7 != start_i + i {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
