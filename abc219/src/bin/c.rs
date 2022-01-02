use proconio::input;

fn main() {
    input! {
        x: String,
        n: usize,
        ss: [String;n]
    }
    let mut map = [0_usize; 26];
    let m:Vec<char> = x.chars().collect();
    for i in 0..26 {
        map[m[i] as usize- 'a' as usize] = i;
    }
    let mut ss_sorted = ss.clone();
    ss_sorted.sort_by(|x, y| {
        let x_c:Vec<char> = x.chars().collect();
        let y_c:Vec<char> = y.chars().collect();
        for i in 0..x_c.len().min(y_c.len()) {
            let x_idx = x_c[i] as usize - 'a' as usize;
            let y_idx = y_c[i] as usize - 'a' as usize;
            match map[x_idx].cmp(&map[y_idx]) {
                std::cmp::Ordering::Equal => continue,
                v => return v,
            };
        }
        return x_c.len().cmp(&y_c.len());
    });
    for s in ss_sorted {
        println!("{}", s);
    }
}
