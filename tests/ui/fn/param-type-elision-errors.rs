// Errors for parameters that leave out their type (`#![feature(parameter_type_elision)]`).
#![feature(parameter_type_elision)]
#![allow(incomplete_features)]

// An elided parameter type is the pattern's path, so generic arguments go in the pattern.
struct Pair<T> {
    a: T,
    b: T,
}
fn generic_elided<T>(Pair { a, b }) {}
//~^ ERROR missing generics for struct `Pair`

// An array pattern spells an array of the type of its first element that spells one, so its
// other elements must match it.
struct Px(u32);
struct Py(u32);
fn mixed_array([Px(a), Py(b)]) {}
//~^ ERROR mismatched types

// A variant isn't a type, so a variant pattern doesn't spell one out. With a single variant the
// pattern is irrefutable, so the enum can be written as the type.
enum Shape {
    Square(u32),
}
fn variant_elided(Shape::Square(side)) {}
//~^ ERROR expected type, found variant `Shape::Square`
fn variant_elided_ref(&Shape::Square(side)) {}
//~^ ERROR expected type, found variant `Shape::Square`
mod imported {
    use super::Shape::Square;
    fn variant_elided_imported(Square(side)) {}
    //~^ ERROR expected type, found variant `Square`
}

// With more variants, the pattern is refutable, so writing the type wouldn't help.
enum Shape2 {
    Square(u32),
    Circle { radius: u32 },
}
fn variant_elided_refutable(Shape2::Square(side)) {}
//~^ ERROR expected type, found variant `Shape2::Square`

fn main() {}
