// A record parameter (`#![feature(struct_args)]`) is only allowed in a function's parameters.
#![feature(struct_args)]
#![allow(incomplete_features)]

type Ptr = fn(_ { x: u32 });
//~^ ERROR unexpected token: `{`

fn main() {}
