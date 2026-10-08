//@ edition: 2024
// Errors for anonymous struct parameters and inferred struct literals (`#![feature(struct_args)]`).
#![feature(struct_args, struct_args_pattern_elision, default_field_values)]
#![allow(incomplete_features)]

fn crop(struct { x: u32, y: u32 = 1 }) -> u32 {
    x + y
}

trait Writer {
    fn write(&mut self, struct { data: u32, times: usize = 1 });
}

struct WrongType;
impl Writer for WrongType {
    fn write(&mut self, struct { data: u32, times: u32 }) {}
    //~^ ERROR field `times` of an anonymous struct parameter has an incompatible type for trait
}

struct ExtraField;
impl Writer for ExtraField {
    fn write(&mut self, struct { data: u32, times: usize, extra: u8 }) {}
    //~^ ERROR field `extra` is not a member of the trait's anonymous struct
}

struct MissingField;
impl Writer for MissingField {
    fn write(&mut self, struct { data: u32 }) {}
    //~^ ERROR missing field `times` in anonymous struct parameter
}

// A renamed field is reported by name.
struct RenamedField;
impl Writer for RenamedField {
    fn write(&mut self, struct { data: u32, count: usize }) {}
    //~^ ERROR field `count` is not a member of the trait's anonymous struct
    //~| ERROR missing field `times` in anonymous struct parameter
}

// The fields are checked when the parameter binds the struct by name, which has no pattern that
// lists them.
struct BoundRenamedField;
impl Writer for BoundRenamedField {
    fn write(&mut self, args: struct { data: u32, count: usize }) {}
    //~^ ERROR field `count` is not a member of the trait's anonymous struct
    //~| ERROR missing field `times` in anonymous struct parameter
}

struct MisspelledField;
impl Writer for MisspelledField {
    fn write(&mut self, struct { dta: u32, times: usize }) {}
    //~^ ERROR field `dta` is not a member of the trait's anonymous struct
}

struct Default;
impl Writer for Default {
    fn write(&mut self, struct { data: u32, times: usize = 2 }) {
        //~^ ERROR default values are not allowed in trait impls
        let _ = (data, times);
    }
}

// The fields of nested anonymous structs are checked against the trait's too.
trait Place {
    fn place(&self, struct { at: struct { x: u32, y: u32 = 0 } });
}
struct NestedMismatch;
impl Place for NestedMismatch {
    fn place(&self, struct { at: struct { x: u8, z: u32 = 1 } }) {}
    //~^ ERROR field `z` is not a member of the trait's anonymous struct
    //~| ERROR default values are not allowed in trait impls
    //~| ERROR field `x` of an anonymous struct parameter has an incompatible type for trait
}

fn missing_fields() {
    crop(_ { y: 2 });
    //~^ ERROR missing field `x`
    crop(_ { y: 2, .. });
    //~^ ERROR missing field `x` in initializer
}

fn no_struct_expected() {
    let _: u32 = _ { x: 1 };
    //~^ ERROR expected struct, variant or union type, found `u32`
}

// An inferred struct literal takes its struct from the expected type.
fn no_expected_type() {
    let _ = _ { x: 1, .. };
    //~^ ERROR type annotations needed
}

// `pat: struct { .. }` is an ordinary `pat: ty`, so the pattern has to match a struct.
fn tuple_binding((a, b): struct { x: u32 }) {}
//~^ ERROR mismatched types

struct Point {
    x: u32,
    y: u32,
}

// A `_ { .. }` pattern checks its fields against the struct it matches, like any struct pattern.
fn inferred_patterns(p: Point, q: Point) {
    let _ { z, .. } = p;
    //~^ ERROR struct `Point` does not have a field named `z`
    let _ { x } = q;
    //~^ ERROR pattern does not mention field `y`
    let _ { x } = 5u32;
    //~^ ERROR expected struct, found `u32`
}

// A renamed field doesn't hide other differences from the trait's method.
struct RenamedAndMut;
impl Writer for RenamedAndMut {
    fn write(&self, struct { data: u32, count: usize }) {}
    //~^ ERROR method `write` has an incompatible type for trait
    //~| ERROR field `count` is not a member of the trait's anonymous struct
    //~| ERROR missing field `times` in anonymous struct parameter
}

// A field of an anonymous struct using `Self` makes its method not dyn compatible, as a parameter
// would.
trait Merge {
    fn merge(&self, struct { other: Self, times: usize = 1 })
    where
        Self: Sized;
    fn merge_unsized(&self, struct { other: Box<Self> });
}
fn merge_dyn(m: &dyn Merge) {}
//~^ ERROR the trait `Merge` is not dyn compatible

fn main() {}
