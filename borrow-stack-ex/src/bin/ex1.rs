#![allow(unused_variables)]

fn main() {
    let mut local = 42;
    // Pointer(l, 0)
    // stack: [Unique(0)]
    let a = &mut local;
    // Pointer(l, 1)
    // stack: [Unique(0), Unique(1)]
    let b = &mut *a;
    // Pointer(l, 2)
    // stack: [Unique(0), Unique(1), Unique(2)]
    *a += 1;
    // Pop Unique(1)
    // stack: [Unique(0)]
    // *b = 43;
    // Pop Unique(2)
    // undefined behavior!
}
