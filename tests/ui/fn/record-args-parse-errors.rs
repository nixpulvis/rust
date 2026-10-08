// An anonymous struct (`#![feature(struct_args)]`) is only allowed as a function's parameter type.
#![feature(struct_args)]
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

fn main() {}
