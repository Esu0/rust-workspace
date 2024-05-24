
use std::{collections::HashMap, fmt::Debug, num::NonZeroU8, ops::{Add, Div, Mul, Sub}};

use num_traits::{Signed, Zero};
use tinyvec::ArrayVec;

struct GroupIter<const BUF_SIZE: usize> {
    n: Option<NonZeroU8>,
    next: [u8; BUF_SIZE],
    buf: [u8; BUF_SIZE],
}

impl<const BUF_SIZE: usize> Iterator for GroupIter<BUF_SIZE> {
    type Item = [u8; BUF_SIZE];
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n?;
        let ret = self.next;
        let r = (n.get() - 1) as usize;
        if r == 0 {
            self.n = None;
            return Some(ret);
        }
        if self.next[r] < self.buf[r] {
            self.next[r] += 1;
        } else {
            let mut r = r;
            loop {
                r -= 1;
                if r == 0 {
                    self.n = None;
                    return Some(ret);
                }
                if self.next[r] < self.buf[r] {
                    self.next[r] += 1;
                    break;
                }
            }
            while r < (n.get() - 1) as usize {
                let next_r = r + 1;
                self.next[next_r] = 0;
                self.buf[next_r] = self.buf[r].max(self.next[r] + 1);
                r = next_r;
            }
        }
        Some(ret)
    }
}

#[allow(dead_code)]
fn group_iter<const BUF_SIZE: usize>(n: u8) -> GroupIter<BUF_SIZE> {
    assert!(1 <= n && n as usize <= BUF_SIZE);
    GroupIter {
        n: NonZeroU8::new(n),
        next: [0; BUF_SIZE],
        buf: [1; BUF_SIZE],
    }
}

pub mod input {
    use std::{
        cell::RefCell,
        fmt::Debug,
        io::Read,
        str::{FromStr, SplitWhitespace},
    };

    fn tokens_init() -> RefCell<SplitWhitespace<'static>> {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        RefCell::new(String::leak(buf).split_whitespace())
    }

    fn next_token() -> Option<&'static str> {
        thread_local! {
            static TOKENS: RefCell<SplitWhitespace<'static>> = tokens_init();
        }
        TOKENS.with_borrow_mut(|tokens| tokens.next())
    }

    #[allow(dead_code)]
    pub fn scan<T: FromStr>() -> Option<T>
    where
        T::Err: Debug,
    {
        next_token().map(|s| s.parse().unwrap())
    }

    #[macro_export]
    macro_rules! scan {
        ($t:ty $(,)?) => {
            $crate::input::scan::<$t>().unwrap()
        };
        ($($t:ty),+ $(,)?) => {
            ($($crate::input::scan::<$t>().unwrap()),*)
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct Fraction<T> {
    numerator: T,
    denominator: T,
}

impl<T: Signed + Copy> Default for Fraction<T> {
    fn default() -> Self {
        Self {
            numerator: T::zero(),
            denominator: T::one(),
        }
    }
}

impl<T: Signed + Copy> Fraction<T> {
    fn new(mut numerator: T, mut denominator: T) -> Self {
        if denominator.is_negative() {
            numerator = -numerator;
            denominator = -denominator;
        }
        let mut ret = Self {
            numerator,
            denominator,
        };
        ret.reduce();
        ret
    }

    fn reduce(&mut self) {
        let gcd = gcd(self.numerator.abs(), self.denominator);
        self.numerator = self.numerator / gcd;
        self.denominator = self.denominator / gcd;
    }

    // 整数ならSome
    fn integer(&self) -> Option<T> {
        if self.denominator.is_one() {
            Some(self.numerator)
        } else {
            None
        }
    }
}

impl<T: Signed + Copy> Add for Fraction<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<T: Signed + Copy> Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<T: Signed + Copy> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<T: Signed + Copy> Div for Fraction<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self::new(numerator, denominator)
    }
}

impl<T: Signed + Copy> From<T> for Fraction<T> {
    fn from(numerator: T) -> Self {
        Self {
            numerator,
            denominator: T::one(),
        }
    }
}

fn gcd<T: Signed + Copy>(a: T, b: T) -> T {
    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Clone, Copy)]
enum Node {
    Num(i32),
    Op(u8),
}

impl Default for Node {
    fn default() -> Self {
        Node::Num(0)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Num(num) => write!(f, "{}", num),
            Node::Op(op) => write!(f, "{}", *op as char),
        }
    }
}

fn search(n: usize, mut num_count: HashMap<i32, u32>, mut f: impl FnMut(&[Node])) {
    let mut stack = ArrayVec::new();
    search_rec(n, &mut num_count, &mut stack, 0, &mut f);
}

const MAX_N: usize = 20;

fn search_rec(n: usize, num_count: &mut HashMap<i32, u32>, stack: &mut ArrayVec<[Node; MAX_N]>, m: usize, f: &mut impl FnMut(&[Node])) {
    if stack.len() + 1 - m < m {
        for op in [b'+', b'-', b'*', b'/'] {
            stack.push(Node::Op(op));
            search_rec(n, num_count, stack, m, f);
            stack.pop();
        }
    } else if n == 0 {
        f(stack);
    }
    let keys = num_count.keys().copied().collect::<ArrayVec<[_; MAX_N]>>();

    for num in keys {
        let count = num_count.get_mut(&num).unwrap();
        *count -= 1;
        if *count == 0 {
            num_count.remove(&num);
        }
        stack.push(Node::Num(num));
        search_rec(n - 1, num_count, stack, m + 1, f);
        stack.pop();
        num_count.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
}

fn calculate(expr: &[Node]) -> Option<Fraction<i64>> {
    let mut stack = ArrayVec::<[Fraction<i64>; MAX_N]>::new();
    for &node in expr {
        match node {
            Node::Num(num) => stack.push(Fraction::from(num as i64)),
            Node::Op(op) => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let result = match op {
                    b'+' => lhs + rhs,
                    b'-' => lhs - rhs,
                    b'*' => lhs * rhs,
                    b'/' => {
                        if rhs.numerator.is_zero() {
                            return None;
                        }
                        lhs / rhs
                    },
                    _ => unreachable!(),
                };
                stack.push(result);
            }
        }
    }
    stack.pop()
}

fn main() {
    let n = scan!(u8);
    assert!(1 <= n && n as usize <= MAX_N);
    let numbers = (0..n).map(|_| scan!(i32)).collect::<Vec<_>>();
    let mut num_count = HashMap::new();
    for &num in &numbers {
        num_count.entry(num).and_modify(|x| *x += 1u32).or_insert(1);
    }
    let mut answer_count = HashMap::new();
    search(n as usize, num_count, |expr| {
        if let Some(result) = calculate(expr) {
            if let Some(integer) = result.integer() {
                *answer_count.entry(integer).or_insert(0u32) += 1;
            }
        }
    });
    for (i, c) in answer_count {
        if c <= 2u32.pow(n as _) && (0..=30).contains(&i) {
            println!("{} {}", i, c);
        }
    }
}
