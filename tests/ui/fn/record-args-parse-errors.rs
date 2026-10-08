// An anonymous struct (`#![feature(struct_args)]`) is only allowed as a function's parameter type.
#![feature(struct_args, struct_args_pattern_elision)]
#![allow(incomplete_features)]

type Ptr = fn(struct { x: u32 });
//~^ ERROR anonymous structs are only allowed as parameter types

// Only another anonymous struct's field may be one.
struct Named {
    a: struct { x: u32 },
    //~^ ERROR anonymous structs are only allowed as parameter types
}

// `_ { .. }` is a struct pattern, which needs a type, not an anonymous struct.
fn inferred_pattern(_ { x: u32 }) {}
//~^ ERROR expected one of `:` or `|`, found `)`

// Binding modes go on the fields only where the struct's type implies its pattern.
fn bound(p: struct { ref s: String, mut n: u32 }) {}
//~^ ERROR `ref` can't be used here
//~| ERROR `mut` can't be used here
fn nested(struct { at: struct { ref mut x: String } }) {}
//~^ ERROR `ref mut` can't be used here

fn main() {}
