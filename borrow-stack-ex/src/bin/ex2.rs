fn main() {
    let mut local = 42; // Location: l, Tag: 0
    // Pointer(l, 0)
    // stack: [Unuque(0)]
    let tmp_ref = &mut local; // Tag: 1
    // Pointer(l, 1)
    // stack: [Unique(0), Unique(1)]
    let raw_ptr = tmp_ref as *mut i32; // Tag: ⊥
    // Pointer(l, ⊥)
    // stack: [Unique(0), Unique(1), SharedRW]
    let result = unsafe {
        example(
            &mut *raw_ptr,
            // 生ポインタを使用して可変参照を作成
            // Pointer(l, 2)を作成
            // stack: [Unique(0), Unique(1), SharedRW, Unique(2)]
            &mut *raw_ptr,
            // Unique(2)をpopして先頭にSharedRWが来るようにする
            // stack: [Unique(0), Unique(1), SharedRW]
            // Pointer(l, 3)を作成
            // stack: [Unique(0), Unique(1), SharedRW, Unique(3)]
        )
    };
    println!("{result}");
}

fn example(x: &mut i32, y: &mut i32) -> i32 {
    // x = Pointer(l, 2), y = Pointer(l, 3)
    // stack: [Unique(0), Unique(1), SharedRW, Unique(3)]
    *x = 42;
    // Unique(2)が存在しないのでundefined behavior
    *y = 13;
    *x
}
