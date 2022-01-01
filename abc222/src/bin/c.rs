use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize
    }
    let mut a:Vec<Vec<char>> = Vec::new();
    for _ in 0..2*n {
        input! {
            a_s:String,
        }
        a.push(a_s.chars().collect());
    }

    let mut res_v:Vec<(usize, usize)> = Vec::new();
    for i in 0.. 2*n {
        res_v.push((i+1, 0));
    }
    for i in 0..m {
        for k in 0..n {
            let mut a_p = res_v[2*k];
            let mut b_p = res_v[2*k+1];
            let a_h = &a[a_p.0-1][i].to_string();
            let b_h = &a[b_p.0-1][i].to_string();
            if a_h == "G" {
                if b_h == "G" {
                } else if b_h == "C" {
                    a_p.1 += 1;
                } else {
                    b_p.1 += 1;
                }
            } else if a_h == "C" {
                if b_h == "G" {
                    b_p.1 += 1;
                } else if b_h == "C" {
                } else {
                    a_p.1 += 1;
                }
            } else {
                if b_h == "G" {
                    a_p.1 += 1;
                } else if b_h == "C" {
                    b_p.1 += 1;
                } else {
                }
            }
            res_v[2*k] = a_p;
            res_v[2*k+1] = b_p;
        }
        res_v.sort_by(|x, y| 
            match y.1.cmp(&x.1) {
                std::cmp::Ordering::Equal => x.0.cmp(&y.0),
                tmp => tmp,
            }
        );
    }

    for r in res_v {
        println!("{:?}", r.0);
    }
}
