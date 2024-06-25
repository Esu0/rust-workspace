pub trait FromStrFast: Sized {
    type Err;
    fn parse_fast(s: &str) -> Result<Self, Self::Err>;

    /// # Safety
    /// 変換に失敗しないこと
    unsafe fn parse_fast_unchecked(s: &str) -> Self {
        Self::parse_fast(s).unwrap_unchecked()
    }
}

fn slice_as_sized<T: Copy, const N: usize>(slice: &[T], default: T) -> [T; N] {
    if slice.len() < N {
        let mut ret = [default; N];
        ret[N - slice.len()..].copy_from_slice(slice);
        ret
    } else {
        unsafe {
            (slice.as_ptr() as *const [T; N]).read()
        }
    }
}

fn parse8(s: &[u8]) -> u64 {
    const MASK0: u64 = 0x0f0f_0f0f_0f0f_0f0f;
    const MUL1: u64 = (10 << 8) + 1;
    const MASK1: u64 = 0x00ff_00ff_00ff_00ff;
    const MUL2: u64 = (100 << 16) + 1;
    const MASK2: u64 = 0x0000_ffff_0000_ffff;
    const MUL3: u64 = (10000 << 32) + 1;
    const MASK3: u64 = 0x0000_0000_ffff_ffff;
    let s = slice_as_sized::<_, 8>(s, 0);
    let mut n = u64::from_le_bytes(s);
    n &= MASK0;
    n = (n.wrapping_mul(MUL1) >> 8) & MASK1;
    n = (n.wrapping_mul(MUL2) >> 16) & MASK2;
    n = (n.wrapping_mul(MUL3) >> 32) & MASK3;
    n
}

impl FromStrFast for u32 {
    type Err = ();
    fn parse_fast(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.as_bytes();
        if s.first().copied().ok_or(())? == b'+' {
            s = &s[1..];
        }
        if s.len() <= 8 {
            let n = parse8(s) as u32;
            return Ok(n);
        }
        let (s, rest) = s.split_at(s.len() - 8);
        let n = parse8(rest) as u32;
        if s.len() <= 8 {
            n.checked_add((parse8(s) as u32).checked_mul(100_000_000).ok_or(())?).ok_or(())
        } else {
            Err(())
        }
    }
}
