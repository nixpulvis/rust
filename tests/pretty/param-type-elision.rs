//@ pp-exact
// Parameters that leave out the type their pattern spells out
// (`#![feature(parameter_type_elision)]`) print as written.

#![feature(parameter_type_elision)]
#![allow(incomplete_features)]

struct P {
    x: u32,
}

fn elided(P { x }, all @ P { x: y }) -> u32 { x + y + all.x }

struct T(u32);

fn spelled(T(a), &P { x }, &mut T(b), (T(c), P { x: d })) -> u32 {
    a + x + b + c + d
}

fn more([T(a), T(b)], &[T(c), T(d)]) -> u32 { a + b + c + d }

fn main() {}
