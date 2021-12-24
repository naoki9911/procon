use proconio::input;

fn main() {
    input! {
	a: String,
    }

    // 0 is start
    // 1 means next is first x
    // 2 means next is second x
    // 3 means next is second x or o
    // 4 means next is o
    let mut state = 0;
    let v:Vec<_> = a.chars().collect();
    for c in v {
        if state == 0 {
            if c == 'o' {
                state = 1;
            } else {
                state = 3;
            }
        } else if state == 1 {
            if c == 'o' {
                println!("No");
                return;
            } else {
                state  = 2;
            }
        } else if state == 2 {
            if c == 'o' {
                println!("No");
                return;
            } else {
                state = 4;
            }
        } else if state == 3 {
            if c == 'x' {
                state = 4;
            } else {
                state = 1;
            }
        } else if state == 4 {
            if c == 'x' {
                println!("No");
                return;
            }
            state = 1;
        }
    }
    println!("Yes");
}
