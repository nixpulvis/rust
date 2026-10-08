// Leaving out the pattern of a parameter that is just an anonymous struct needs
// `struct_args_pattern_elision`, on top of `struct_args`.
#![feature(struct_args, named_arguments)]
#![allow(incomplete_features)]

// The anonymous struct implies the pattern `_ { x }`.
fn f(struct { x: u32 }) -> u32 {
    //~^ ERROR leaving out an anonymous struct parameter's pattern is experimental
    x
}

// The fields after `;` imply their pattern too, but they are `named_arguments`, which doesn't
// need it.
fn g(a: u32; z: u32) -> u32 {
    a + z
}

// Spelling out the pattern doesn't need it, and calls are the same.
fn h(_ { x }: struct { x: u32 }) -> u32 {
    x
}

fn main() {
    f(_ { x: 1 });
    g(1, z: 2);
    h(_ { x: 1 });
}
