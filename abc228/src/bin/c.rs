use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut scores:Vec<usize> = Vec::new();
    for _ in 0..n {
        input! {
            p: [usize; 3],
        }
        let sum = p.iter().sum();
        scores.push(sum);
    }

    let mut scores_sorted = scores.clone();
    scores_sorted.sort_by(|x, y| y.cmp(&x));
    for i in 0..n {
        let tgt_score = scores[i] + 300;
        if tgt_score >= scores_sorted[k-1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }

}
