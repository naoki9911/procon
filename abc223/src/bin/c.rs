use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut v:Vec<(f64, f64)> = Vec::new();
    let mut total_sec:f64 = 0.0;
    for _ in 0..n {
        input! {
            a: f64,
            b: f64
        }
        v.push((a, b));
        total_sec += a  / b;
    }
    let elapsed = total_sec / 2.0;
    let mut cur_sec:f64 = 0.0;
    let mut cur_cm:f64 = 0.0;
    for i in 0..n {
        let next_sec = v[i].0 / v[i].1;
        let next_elapsed = cur_sec + next_sec;
        if next_elapsed > elapsed {
            let rest_sec = elapsed - cur_sec;
            println!("{:?}",cur_cm + v[i].1 * rest_sec);
            return;
        }
        cur_sec += next_sec;
        cur_cm += v[i].0;
    }
}
