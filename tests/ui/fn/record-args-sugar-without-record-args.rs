// The sugar of record parameters and arguments also needs `#![feature(struct_args)]`.
#![feature(struct_args_sugar)]
#![allow(incomplete_features)]

struct Point {
    x: u32,
}

fn f(p: Point) -> u32 {
    p.x
}

fn g(; y: u32) -> u32 { //~ ERROR record parameters and arguments are experimental
    y
}

fn main() {
    f(x: 1); //~ ERROR record parameters and arguments are experimental
    g(y: 2); //~ ERROR record parameters and arguments are experimental
}
