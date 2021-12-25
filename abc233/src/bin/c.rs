use proconio::input;

fn solve(tbl: &Vec<Vec<usize>>, bag_i: usize, bag_num: usize, x:usize) -> usize {
    if bag_i == bag_num-1 {
        let mut cnt:usize = 0;
        for i in &tbl[bag_i] {
            if x == *i {
                cnt += 1;
            }
        }

        return cnt;
    }

    let mut cnt:usize = 0;
    for i in &tbl[bag_i] {
        if x % *i == 0 {
            cnt += solve(tbl, bag_i+1, bag_num, x / i);
        }
    }

    return cnt;
}
fn main() {
    input! {
        n: usize,
        x: usize
    }
    let mut bag:Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }
        bag[i] = a;
    }

    println!("{:?}", solve(&bag, 0, n, x));
}
