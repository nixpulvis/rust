//@ run-pass
//@ edition: 2024
//@ aux-build: record-args-sugar-lib.rs
// The sugar of records (`#![feature(struct_args_sugar)]`) for another crate's functions and traits.
#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

extern crate record_args_sugar_lib as lib;

use lib::Greet;

struct Shout;
impl Greet for Shout {
    fn greet(&self; times: usize, name: &'static str) -> String {
        name.to_uppercase().repeat(times)
    }
}

fn main() {
    assert_eq!(lib::text("hi"), "hi@16");
    assert_eq!(lib::text("hi", bold: true), "hi@16!");
    assert_eq!(lib::text("hi", _ { size: 8, .. }), "hi@8");
    assert_eq!(lib::Hello.greet(name: "you"), "hello you");
    let shout: &dyn Greet = &Shout;
    assert_eq!(shout.greet(name: "you", times: 2), "YOUYOU");
}
