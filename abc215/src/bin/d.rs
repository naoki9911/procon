use proconio::input;

fn fact(x_tmp:usize) -> Vec<usize> {
    let mut i = 2;
    let mut x = x_tmp;
    let mut v:Vec<usize> = Vec::new();
    while i * i <= x {
        let mut pushed = false;
        while x%i == 0 {
            x /= i;
            if !pushed {
                v.push(i);
            }
            pushed = true;
        }
        i+= 1;
    }

    if x != 1 {
        v.push(x);
    }

    return v;
}
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }
    let mut tbl:Vec<bool> = vec![true; m+1];
    tbl[0] = false;
    for a_e in &a {
        let ps = fact(*a_e);
        for p in ps {
            if p <= m && !tbl[p] {
                continue;
            }
            let mut i_idx = p;
            while i_idx <= m {
                tbl[i_idx] = false;
                i_idx += p;
            }
        }
    }

    let cnt = tbl.iter().filter(|&x| *x).count();
    println!("{}", cnt);
    for i in 1..=m {
        if !tbl[i] {
            continue;
        }
        println!("{:?}", i);
    }
}
