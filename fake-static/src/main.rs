fn h<'a, 'b, T, U>(x: &'a &'b (), y: &'a T, z: &'b U) -> &'a U {
    z
}

fn g<'a, 'b, T, U>(x: &'a T, y: &'b U) -> &'a U {
    let f: fn(&'static &'static (), &'a T, &'b U) -> &'a U = h;
    f(&&(), x, y)
}

fn fake_static<T>(x: &T) -> &'static T {
    g(Box::leak(Box::new(())), x)
}

fn main() {
    let a = 1;
    let b = fake_static(&a);
}
