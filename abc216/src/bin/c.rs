use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n_tmp: usize,
    }
    let mut res:VecDeque<char> = VecDeque::new();
    let mut n = n_tmp;
    while n != 0 {
        if n % 2 == 0 {
            res.push_front('B');
            n /= 2;
        } else {
            res.push_front('A');
            n -= 1;
        }
    }
    let res_str:String = res.iter().collect();
    println!("{}", res_str);
}
