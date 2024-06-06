use rand::seq::SliceRandom;
use proconio::input;

fn main() {
    let mut rng = rand::thread_rng();
    input! {
        n: usize,
        mut names: [String; n],
        m: usize,
        k: usize,
    }

    let mut s = names.iter().flat_map(|name| name.chars()).collect::<Vec<_>>();
    for _ in 0..m {
        s.shuffle(&mut rng);
        for c in &s[..k] {
            print!("{}", *c);
        }
        println!();
    }
}
