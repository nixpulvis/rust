// Leaving out the pattern of a parameter that is just an anonymous struct needs
// `struct_args_pattern_elision`, on top of `struct_args`.
#![feature(struct_args)]
#![allow(incomplete_features)]

// The anonymous struct implies the pattern `_ { x }`.
fn f(struct { x: u32 }) -> u32 {
    //~^ ERROR leaving out an anonymous struct parameter's pattern is experimental
    x
}

// Spelling out the pattern doesn't need it, and calls are the same.
fn h(_ { x }: struct { x: u32 }) -> u32 {
    x
}

fn main() {
    f(_ { x: 1 });
    h(_ { x: 1 });
}
