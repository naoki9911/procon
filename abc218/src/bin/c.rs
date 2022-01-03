use proconio::input;

fn solve(mat:&Vec<Vec<bool>>, pat:&Vec<(i64, i64)>, n:usize) -> bool {
    let mut mat_new = mat.clone();
    for y in 0..n {
        for x in 0..n {
            if !mat[y][x] {
                continue;
            }
            //println!("{} {}", x, y);
            for p in pat {
                let x_m = (x as i64 + p.0) as usize;
                let y_m = (y as i64 + p.1) as usize;
                if y_m >= n || x_m >= n {
                    return false;
                }
                if !mat[y_m][x_m] {
                    return false;
                }
                mat_new[y_m][x_m] = false;
            }
            return mat_new.iter().all(|p| p.iter().all(|q| !q));
        }
    }

    return false;
}
fn main() {
    input! {
        n: usize,
        ss: [String; n],
        ts: [String; n]
    }
    let mut pat:Vec<(i64, i64)> = Vec::new();
    let mut init_x:i64 = 0;
    let mut init_y:i64 = 0;
    let mut t_mat:Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut t_mat_l:Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut t_mat_r:Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut t_mat_i:Vec<Vec<bool>> = vec![vec![false; n]; n];
    for y in 0..n {
        let s_c: Vec<char> = ss[y].chars().collect();
        for x in 0..n {
            if s_c[x] != '#' {
                continue;
            }
            if pat.len() == 0 {
                pat.push((0, 0));
                init_x = x as i64;
                init_y = y as i64;
            } else {
                pat.push((x as i64 - init_x, y as i64- init_y))
            }
        }
    }
    for y in 0..n {
        let t_c: Vec<char> = ts[y].chars().collect();
        for x in 0..n {
            let flag = t_c[x] == '#';
            t_mat[y][x] = flag;
            t_mat_l[n-x-1][y] = flag;
            t_mat_r[x][n-y-1] = flag;
            t_mat_i[n-y-1][n-x-1] = flag;
        }
    }
    //println!("{:?}", pat);
    //println!("{:?}", t_mat_i);
    if solve(&t_mat, &pat, n) {
        println!("Yes");
        return;
    }
    if solve(&t_mat_l, &pat, n) {
        println!("Yes");
        return;
    }
    if solve(&t_mat_r, &pat, n) {
        println!("Yes");
        return;
    }
    if solve(&t_mat_i, &pat, n) {
        println!("Yes");
        return;
    }
    println!("No");
}
