use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize
    }
    let mut items:Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        input! {
            a: usize,
            b: usize
        }
        items.push((a, b));
    }
    items.sort_by(|x ,y| y.0.cmp(&x.0));
    let mut taste = 0;
    let mut cur_w = 0;
    for chees in items {
        if cur_w >= w {
            break;
        }
        for _ in 0..chees.1 {
            taste += chees.0;
            cur_w += 1;
            if cur_w >= w {
                break;
            }
        }
    }

    println!("{:?}", taste)
}
