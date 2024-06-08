use std::ops::Deref;

#[allow(unused_imports)]
use proconio::input;
use rand::Rng;



fn eratosthenes(n: u64) -> Vec<u64> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p <= n {
        if is_prime[p as usize] {
            let mut i = p * p;
            while i <= n {
                is_prime[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }
    is_prime.into_iter().enumerate().filter_map(|(i, b)| if b { Some(i as u64) } else { None }).collect()
}


fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => return false,
        2 => return true,
        _ if n % 2 == 0 => return false,
        _ => (),
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn check1() {
    let mut rng = rand::thread_rng();
    let cases = 1000;
    let a = 1;
    let b = std::iter::repeat_with(|| rng.gen_range(1..=1_000_000_000)).take(cases).collect::<Vec<_>>();
    let c = std::iter::repeat_with(|| rng.gen_range(1..=1_000_000_000_000_000_000)).take(cases).collect::<Vec<_>>();
    let n = std::iter::repeat_with(|| rng.gen_range(2..=1_000_000_000)).take(cases).collect::<Vec<_>>();
    for i in 0..cases {
        assert_eq!(super_mod_pow::solve(a, b[i], c[i], n[i]), 1);
    }
}

fn main() {
    check1();
}
