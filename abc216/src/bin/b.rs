use proconio::input;

fn main() {
    input! {
        n: usize,
        sts_tmp: [[String; 2]; n]
    }
    let mut sts:Vec<(String, String)> = Vec::new();
    for st in sts_tmp {
        sts.push((st[0].clone(), st[1].clone()));
    }
    sts.sort_by(|x, y| x.0.cmp(&y.0));
    let mut s_idx = 0;
    loop {
        if s_idx >= n - 1 {
            break;
        }
        if sts[s_idx].0 == sts[s_idx+1].0 {
            if sts[s_idx].1 == sts[s_idx+1].1 {
                println!("Yes");
                return;
            }
        }
        s_idx += 1;
    }
    println!("No");
}
