//@ pp-exact
// Record parameters and arguments (`#![feature(struct_args)]`) print as written.

#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

fn crop(img: &str, _ { width: u32, x: u32 = 0 }) -> u32 { width + x }

fn countdown(p: _ { mut n: u32, step: u32 = 1 }) -> u32 { n -= p.step; n }

fn main() { crop("img", _ { width: 1, .. }); countdown(_ { n: 2, step: 1 }); }
