// Patterns that spell out no type still need one written (`#![feature(parameter_type_elision)]`).
#![feature(parameter_type_elision)]
#![allow(incomplete_features)]

struct Px(u32);

// An array pattern with a rest pattern spells nothing: its length is unknown, and as a slice it
// would be refutable, since it doesn't match an empty one.
fn rest([Px(a), ..]) {}
//~^ ERROR expected one of `:` or `|`, found `)`
fn rest_ref(&[Px(a), ..]) {}
//~^ ERROR expected one of `:` or `|`, found `)`

// An empty array pattern has no element to spell its type.
fn empty_array([]) {}
//~^ ERROR expected one of `:` or `|`, found `)`

// Patterns that can't bind anything spell no type either, so such a parameter writes `_: ()`.
mod m {
    pub struct Unit;
}
fn unit(()) {}
//~^ ERROR expected one of `:` or `|`, found `)`
fn unit_struct(m::Unit) {}
//~^ ERROR expected one of

fn main() {}
