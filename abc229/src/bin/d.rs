use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize,
    }
    let mut v:Vec<usize> = vec![0; s.len()+1];
    let sc:Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        if sc[i] == '.' {
            v[i+1] = v[i] + 1;
        } else {
            v[i+1] = v[i];
        }
    }

    let mut j = 0;
    let mut res = 0;
    for i in 0..s.len() {
        while j < s.len() && v[j+1] - v[i] <= k {
            j += 1;
        }
        res = res.max(j-i);
    }
    println!("{:?}", res);
}
