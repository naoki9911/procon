use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }
    input! {
        a: [usize; n],
    }
    let mut a_tbl:Vec<bool> = vec![false; n];
    let mut cur_friend = x-1;
    let mut cnt = 0;
    loop {
        if a_tbl[cur_friend] {
            break;
        }
        a_tbl[cur_friend] = true;
        cnt += 1;
        cur_friend = a[cur_friend] - 1;
    }
    println!("{:?}", cnt);
}
