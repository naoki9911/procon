use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[usize; n],
        b:[usize; n]
    }
    let mut c = vec![[0usize; 3001]; n];
    for j in a[n-1]..=b[n-1] {
        c[n-1][j] = 1;
    }
    for i in (0..n-1).rev() {
        for j in a[i]..=b[i] {
            c[i][j] = c[i+1][a[i+1].max(j)..=b[i+1]].iter().sum();
            c[i][j] = c[i][j] % 998244353;
        }
    }
    let mut cnt:usize = c[0][a[0]..=b[0]].iter().sum();
    cnt = cnt % 998244353;
    println!("{:?}", cnt);
}
