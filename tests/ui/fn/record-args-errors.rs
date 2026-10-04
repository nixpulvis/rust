//@ edition: 2024
// Errors for record parameters and record arguments (`#![feature(struct_args)]`).
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

fn crop(_ { x: u32, y: u32 = 1 }) -> u32 {
    x + y
}

trait Writer {
    fn write(&mut self, _ { data: u32, times: usize = 1 });
}

struct WrongType;
impl Writer for WrongType {
    fn write(&mut self, _ { data: u32, times: u32 }) {}
    //~^ ERROR field `times` of a record parameter has an incompatible type for trait
}

struct ExtraField;
impl Writer for ExtraField {
    fn write(&mut self, _ { data: u32, times: usize, extra: u8 }) {}
    //~^ ERROR field `extra` is not a member of the trait's record
}

struct MissingField;
impl Writer for MissingField {
    fn write(&mut self, _ { data: u32 }) {}
    //~^ ERROR missing field `times` in record parameter
}

// A renamed field is reported by name.
struct RenamedField;
impl Writer for RenamedField {
    fn write(&mut self, _ { data: u32, count: usize }) {}
    //~^ ERROR field `count` is not a member of the trait's record
    //~| ERROR missing field `times` in record parameter
}

struct MisspelledField;
impl Writer for MisspelledField {
    fn write(&mut self, _ { dta: u32, times: usize }) {}
    //~^ ERROR field `dta` is not a member of the trait's record
}

struct Default;
impl Writer for Default {
    fn write(&mut self, _ { data: u32, times: usize = 2 }) {
        //~^ ERROR default values are not allowed in trait impls
        let _ = (data, times);
    }
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

// A record argument takes its struct from the expected type.
fn no_expected_type() {
    let _ = _ { x: 1, .. };
    //~^ ERROR type annotations needed
}

// A record parameter binds the whole record only by name.
fn tuple_binding((a, b): _ { x: u32 }) {}
//~^ ERROR expected a binding for a record parameter

// A renamed field doesn't hide other differences from the trait's method.
struct RenamedAndMut;
impl Writer for RenamedAndMut {
    fn write(&self, _ { data: u32, count: usize }) {}
    //~^ ERROR method `write` has an incompatible type for trait
    //~| ERROR field `count` is not a member of the trait's record
    //~| ERROR missing field `times` in record parameter
}

// A record field using `Self` makes its method not dyn compatible, as a parameter would.
trait Merge {
    fn merge(&self, _ { other: Self, times: usize = 1 })
    where
        Self: Sized;
    fn merge_unsized(&self, _ { other: Box<Self> });
}
fn merge_dyn(m: &dyn Merge) {}
//~^ ERROR the trait `Merge` is not dyn compatible

fn main() {}
