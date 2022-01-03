use proconio::input;

fn main() {
    input! {
        ps: [usize; 26]
    }
    let mut res:Vec<char> = Vec::new();
    for p in ps {
        let p_conv = p as u8+ 'a' as u8 - 1;
        res.push(p_conv as char);
    }
    let res_s: String = res.iter().collect();
    println!("{}", res_s);

}
