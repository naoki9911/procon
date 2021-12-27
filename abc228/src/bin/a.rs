use proconio::input;

fn main() {
    input! {
        s: usize,
        t_tmp: usize,
        x_tmp: usize
    }
    let mut t = t_tmp;
    let mut x = x_tmp;
    if t_tmp < s {
       t = t_tmp + 24; 
       if x_tmp < s {
           x = x_tmp + 24;
       }
    }
    if s <= x && x < t {
       println!("Yes");
    } else {
        println!("No");
    }
}
