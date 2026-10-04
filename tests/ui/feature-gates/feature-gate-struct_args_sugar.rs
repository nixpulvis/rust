// The sugar needs `struct_args_sugar` on top of `struct_args`.
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

fn f(_ { x: u32 = 0 }) -> u32 {
    x
}

fn g(a: u32; y: u32) -> u32 { //~ ERROR named arguments are experimental
    a + y
}

fn main() {
    f(x: 1); //~ ERROR named arguments are experimental
    f(); //~ ERROR leaving out a record argument is experimental
    g(1, _ { y: 2 });
}
