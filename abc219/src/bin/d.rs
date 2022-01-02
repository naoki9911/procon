use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [[usize;2];n]
    }
    let mut tbl = vec![vec![vec![10000000i64; y+1]; x+1]; n+1];
    tbl[0][0][0] = 0;
    for i in 1..=n {
        for a_idx in 0..=x {
            for b_idx in 0..=y {
                let cur_a =  x.min(a_idx + a[i-1][0]);
                let cur_b =  y.min(b_idx + a[i-1][1]);
                let cost = tbl[i-1][a_idx][b_idx] + 1; 
                let prev = tbl[i][cur_a][cur_b];
                tbl[i][cur_a][cur_b] = cost.min(prev);
            }
        }
        for a_idx in 0..=x {
            for b_idx in 0..=y {
                tbl[i][a_idx][b_idx] = tbl[i-1][a_idx][b_idx].min(tbl[i][a_idx][b_idx]);
            }
        }
    }
    //for i in 0..=n {
    //for a_idx in 0..=x {
    //    for b_idx in 0..=y {
    //        if tbl[i][a_idx][b_idx] == 10000000 {
    //            continue;
    //        }
    //        println!("({}, {},{}) = {}",i, a_idx, b_idx, tbl[i][a_idx][b_idx]);
    //    }
    //}
    //}
    let cost = tbl[n][x][y];
    if cost == 10000000 {
        println!("-1");
    } else {
        println!("{:?}", cost);
    }
}
