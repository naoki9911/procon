use proconio::input;
use std::cmp;

fn main() {
    input! {
	n: i64,
	a: i64,
	b: i64,
    }
    input! {
	p: i64,
	q: i64,
	r: i64,
	s: i64,
    }

    let mut table:Vec<Vec<bool>> = vec![vec![false; (s - r + 1) as usize]; (q - p + 1) as usize];
    let i1_start = cmp::max(1, 1 - b + a);
    let i1_end = cmp::min(n, n - b + a);
    for i in p..=q {
        if i < i1_start || i > i1_end {
            continue
        }
        let j = b - a + i;
        if j < r || j > s {
            continue
        }
        table[(i - p) as usize][(j - r) as usize] = true;
    }

    let i2_start = cmp::max(1, b - n + a);
    let i2_end = cmp::min(n, b + a - 1);
    for i in p..=q {
        if i < i2_start || i > i2_end {
            continue
        }
        let j = b + a - i;
        if j < r || j > s {
            continue
        }
        table[(i - p) as usize][(j - r) as usize] = true;
    }

    for v in table {
        let out:Vec<&str> = v.iter().map(|x| if *x { "#" } else {"."}).collect();
        println!("{}", out.join(""));
    }
}
