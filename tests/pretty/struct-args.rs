//@ pp-exact
// Anonymous struct parameters and inferred struct literals and patterns
// (`#![feature(struct_args)]`), and parameters that leave out their pattern
// (`#![feature(struct_args_pattern_elision)]`), print as written.

#![feature(struct_args, struct_args_pattern_elision, default_field_values)]
#![allow(incomplete_features)]

struct P {
    x: u32,
}

fn crop(img: &str, struct { width: u32, x: u32 = 0 }) -> u32 { width + x }

fn countdown(mut p: struct { n: u32, step: u32 = 1 }) -> u32 {
    p.n -= p.step;
    p.n
}

fn renamed(_ { x: y, .. }: struct { x: u32, z: u32 = 0 }) -> u32 { y }

fn place(struct { at: struct { x: u32, y: u32 = 0 } }) -> u32 { at.x + at.y }

fn modes(struct { ref s: String, ref mut v: Vec<u8>, mut n: u32 }) -> usize {
    s.len()
}

fn main() {
    crop("img", _ { width: 1, .. });
    countdown(_ { n: 2, step: 1 });
    let _ { x, .. } = P { x: renamed(_ { x: 1, .. }) };
}
