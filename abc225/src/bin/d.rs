use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut trains_front:Vec<usize> = vec![0; n+1];
    let mut trains_back:Vec<usize> = vec![0; n+1];
    for _ in 0..q {
        input! {
            q_cmd: String
        }
        if q_cmd == "1" {
            input! {
                x_s: String,
                y_s: String,
            }
            let x = x_s.parse::<usize>().unwrap();
            let y = y_s.parse::<usize>().unwrap();
            trains_back[x] = y;
            trains_front[y] = x;
        } else if q_cmd == "2" {
            input! {
                x_s: String,
                y_s: String,
            }
            let x = x_s.parse::<usize>().unwrap();
            let y = y_s.parse::<usize>().unwrap();
            trains_back[x] = 0;
            trains_front[y] = 0;
        } else {
            input! {
                x_s: String,
            }
            let mut x = x_s.parse::<usize>().unwrap();
            loop {
                if trains_front[x] == 0 {
                    break;
                }
                x = trains_front[x];
            }
            let mut out_str: String = "".to_string();
            let mut cnt: usize = 0;
            loop {
                out_str += &x.to_string();
                cnt += 1;
                x = trains_back[x];
                if x == 0 {
                    break;
                }
                out_str += " ";
            }
            println!("{:?} {}", cnt, out_str);
        }
    }
}
