use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut tbl = vec![[0usize; 10]; n];
    tbl[1][(a[0] + a[1]) % 10] += 1;
    tbl[1][(a[0] * a[1]) % 10] += 1;
    for i in 2..n {
        for a_p in 0..10 {
            if tbl[i - 1][a_p] == 0 {
                continue;
            }
            tbl[i][(a_p + a[i]) % 10] += tbl[i - 1][a_p];
            tbl[i][(a_p * a[i]) % 10] += tbl[i - 1][a_p];
            tbl[i][(a_p + a[i]) % 10] %= 998244353;
            tbl[i][(a_p * a[i]) % 10] %= 998244353;
        }
    }
    for i in 0..10 {
        println!("{:?}", tbl[n - 1][i]);
    }
}
