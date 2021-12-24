use proconio::input;
fn solve(arr:&Vec<usize>, start_idx:usize, end_idx:usize, tgt:usize) -> usize {
    if end_idx == start_idx {
        if arr[end_idx] >= tgt {
            return end_idx + 1;
        }
        return end_idx;
    }

    let pivot_idx = (end_idx + start_idx)/2;
    if arr[pivot_idx] >= tgt {
        return solve(arr, pivot_idx+1, end_idx, tgt);
    } else {
        return solve(arr, start_idx, pivot_idx, tgt);
    }
}

fn main() {
    input! {
	n: usize,
    q: usize,
    }
        input! {
            x: [usize; n],
        }
    let mut students = x.to_vec();
    students.sort_by(|x, y| y.cmp(x));
    for _ in 0..q {
        input! {
            x: usize,
        }
        let res = solve(&students, 0, n-1, x);
        println!("{:?}", res);
    }
}
