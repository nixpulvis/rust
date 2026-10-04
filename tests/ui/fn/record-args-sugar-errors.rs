//@ edition: 2024
// Errors for the sugar of record parameters and arguments (`#![feature(struct_args_sugar)]`).
#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

fn positional(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(_ { name: &str, times: usize = 1 }) -> usize {
    name.len() * times
}

struct Options {
    verbose: bool = false,
    quiet: bool = false,
}
fn run(options: Options) -> bool {
    options.verbose
}

fn first(_ { x: u32 = 0 }, y: u32) -> u32 {
    x + y
}

fn named_args() {
    let times = 2;
    positional(1, b: 2);
    //~^ ERROR named arguments need a record parameter, but this parameter has type `i32`
    greet(name: "x", times);
    //~^ ERROR positional arguments must come before named arguments
    greet(name: "x", 3);
    //~^ ERROR positional arguments must come before named arguments
    greet(name: "x", .., times: 2);
    //~^ ERROR `..` must come after the other named arguments
    // Named arguments take the defaults they leave out, but not fields without one.
    greet(times: 2);
    //~^ ERROR missing named argument `name`
}

fn left_out() {
    // Only a record whose fields all have defaults can be left out.
    greet();
    //~^ ERROR this function takes 1 argument but 0 arguments were supplied
    // A named struct can't be, even with all fields defaulted.
    run();
    //~^ ERROR this function takes 1 argument but 0 arguments were supplied
    // Only a last record parameter can be left out.
    first(1);
    //~^ ERROR this function takes 2 arguments but 1 argument was supplied
}

// At most one `;`, which starts the last parameter.
fn two_semis(a: u32; b: u32; c: u32) {}
//~^ ERROR record parameter fields are separated by `,`

fn main() {}
