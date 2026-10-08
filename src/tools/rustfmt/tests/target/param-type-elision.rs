#![feature(parameter_type_elision)]

fn elided(Args { x, .. }, all @ Args { x: y }, /* kept */ Pair::<u8> { a, b }) -> u32 {
    x
}
fn spelled(T(a), &P { x }, &mut T(b), (T(c), P { x: d })) -> u32 {
    a
}
fn more([T(a), T(b)], &[T(c), T(d)], Pair<u8> { a, b }, Wrap<u8>(x)) -> u32 {
    a
}
