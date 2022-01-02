use itertools::Itertools;
use proconio::input;

fn v_to_usize(a: &Vec<usize>) -> usize {
    let mut res = 0;
    for ae in a {
        res *= 10;
        res += ae;
    }
    return res;
}
fn get_res(tgt: &Vec<usize>, perm: &Vec<usize>) -> usize {
    let mut a: Vec<usize> = Vec::new();
    for i in 0..tgt.len() {
        a.push(tgt[perm[i]]);
    }
    let a_usize = v_to_usize(&a);
    return a_usize;
}

fn main() {
    input! {
        n: usize
    }
    let tgt: Vec<usize> = n
        .to_string()
        .chars()
        .map(|x| x as usize - '0' as usize)
        .collect();
    let mut res = 0;
    for perm in (0..tgt.len()).permutations(tgt.len()) {
        let mut b: usize = 0;
        let mut a = get_res(&tgt, &perm);
        for _ in 0..tgt.len() {
            b = b * 10 + a % 10;
            a /= 10;
            res = res.max(a * b);
        }
    }
    println!("{:?}", res);
}
