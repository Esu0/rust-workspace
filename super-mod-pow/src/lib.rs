use std::collections::HashMap;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn prime_factors<M: Extend<(u64, u32)>>(mut n: u64, map: &mut M) {
    let mut count2 = 0;
    while n % 2 == 0 {
        n /= 2;
        count2 += 1;
    }
    if count2 > 0 {
        map.extend(std::iter::once((2, count2)));
    }
    let mut i = 3;
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count > 0 {
            map.extend(std::iter::once((i, count)));
        }
        i += 2;
    }
    if n > 1 {
        map.extend(std::iter::once((n, 1)));
    }
}

pub fn mpow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 != 0 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result
}

pub fn solve(a: u64, b: u64, c: u64, n: u64) -> u64 {
    let g = gcd(n, a);
    // println!("{n} = {} * {}", g, n / g);
    // println!("{a} = {} * {}", g, a / g);
    // let p = eratosthenes(n);
    // println!("{:?}", p);
    let mut n_prime_factors = HashMap::new();
    let mut p = HashMap::new();
    prime_factors(g, &mut p);
    prime_factors(n, &mut n_prime_factors);
    let mut ans = 1u64;

    let mut phi = 1;
    for (&p, &k) in &n_prime_factors {
        phi *= p.pow(k - 1) * (p - 1);
    }

    for (&p, &k) in &p {
        let count = n_prime_factors[&p];
        
        // compare k * b^c and count
        {
            let mut tmp = k as u64;
            for _ in 0..c {
                if tmp > count as u64 {
                    break;
                }
                tmp *= b;
            }
            if tmp < count as u64 {
                ans = ans * mpow(p, tmp, n) % n;
                continue;
            }
        }
        let ppcount = p.pow(count);
        let modulo = n / ppcount;

        // calculate p^(k * b^c - count) (mod modulo)
        let phi = phi / (ppcount / p * (p - 1));
        let exp = (mpow(b, c, phi) + phi - count as u64) % phi;
        ans = ans * (mpow(p, exp, modulo) * ppcount) % n;
    }
    let a = a / g;
    ans = ans * mpow(a, mpow(b, c, phi), n) % n;
    ans
}
