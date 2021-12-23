use itertools::Itertools;
use proconio::input;

#[derive(Clone, Debug)]
struct Edge {
    a: i32,
    b: i32,
}

impl Edge {
    pub fn new(a: i32, b: i32) -> Edge {
        if a > b {
            return Edge { a: b, b: a };
        } else {
            return Edge { a: a, b: b };
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.b == other.a && self.a == other.b)
    }
}

impl Eq for Edge {}

fn judge(g1: &Vec<Edge>, g2: &Vec<Edge>) -> bool {
    for (e1, e2) in (g1.iter()).zip(g2.iter()) {
        if e1 != e2 {
            return false;
        }
    }
    return true;
}

fn convert(g: &Vec<Edge>, p: &Vec<i32>) -> Vec<Edge> {
    let mut g_new: Vec<Edge> = Vec::new();

    for e in g {
        let e_new = Edge::new(p[(e.a - 1) as usize], p[(e.b - 1) as usize]);
        g_new.push(e_new);
    }

    g_new.sort_by(|x, y| match y.a.cmp(&x.a) {
        std::cmp::Ordering::Equal => y.b.cmp(&x.b),
        other => other,
    });

    return g_new;
}

fn main() {
    input! {
        n: i32,
        m: i32,
    }

    let mut g1: Vec<Edge> = Vec::new();
    let mut g2: Vec<Edge> = Vec::new();
    for _ in 0..m {
        input! {
            a: i32,
            b: i32,
        }
        let edge = Edge::new(a, b);
        g1.push(edge);
    }
    g1.sort_by(|x, y| match y.a.cmp(&x.a) {
        std::cmp::Ordering::Equal => y.b.cmp(&x.b),
        other => other,
    });

    for _ in 0..m {
        input! {
            a: i32,
            b: i32,
        }
        let edge = Edge::new(a, b);
        g2.push(edge);
    }

    if m == 0 {
        println!("Yes");
        return;
    }

    for perm in (1..=n).permutations(n as usize) {
        let g_new = convert(&g2, &perm);
        if judge(&g1, &g_new) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
