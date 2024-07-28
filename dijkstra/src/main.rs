use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvb: [(usize, usize, u64); m],
    }

    let mut adj_list = vec![vec![]; n];
    for &(u, v, b) in &uvb {
        adj_list[u - 1].push((v - 1, b));
    }

    let mut heap = BinaryHeap::from([Reverse((0, 0))]);
    let mut dist = vec![u64::MAX; n];
    dist[0] = 0;
    while let Some(Reverse((w, node))) = heap.pop() {
        if w > dist[node] {
            continue;
        }
        for &(next, b) in &adj_list[node] {
            let next_w = w + b;
            if next_w < dist[next] {
                dist[next] = next_w;
                heap.push(Reverse((next_w, next)));
            }
        }
    }
    for &d in &dist[1..] {
        print!("{} ", d);
    }
    println!();
}
