//@ run-pass
//@ edition: 2024
// A parameter whose pattern spells out its type can leave the type out
// (`#![feature(parameter_type_elision)]`).
#![feature(parameter_type_elision)]
#![allow(incomplete_features)]

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

// A struct pattern names its type, so the parameter's type can be left out.
fn area(scale: u32, Rect { width, height, .. }) -> u32 {
    scale * width * height
}

struct Wrapper<'a, T> {
    s: &'a str,
    v: T,
}

// The path is the type as written, so generic arguments go in the pattern, with or without a
// turbofish.
fn unwrap<T>(Wrapper::<'_, T> { s, v }) -> (usize, T) {
    (s.len(), v)
}

fn unwrap_type_like<T>(Wrapper<'_, T> { s, v }) -> (usize, T) {
    (s.len(), v)
}

// Also under a binding, which names the whole value.
fn bound(all @ Rect { x, .. }) -> (u32, u32) {
    (x, all.y)
}

// Tuple struct, reference, and tuple patterns spell out their types too.
struct Px(u32);

fn tuple_struct(Px(n)) -> u32 {
    n
}

fn by_shared_ref(&Rect { width, .. }) -> u32 {
    width
}

fn by_mut(&mut Px(n)) -> u32 {
    n + 1
}

// The elided `&` takes part in lifetime elision like a written one.
fn by_ref_out(px @ &Px(_)) -> &u32 {
    &px.0
}

fn tuple((Px(a), Rect { x, .. })) -> u32 {
    a + x
}

fn nested_ref(&(Px(a), Wrapper::<'_, u8> { v, .. })) -> u32 {
    a + u32::from(v)
}

fn nested_type_like(&(Px(a), Wrapper<'_, u8> { v, .. })) -> u32 {
    a + u32::from(v)
}

// Generic arguments without a turbofish after a multi-segment path.
mod shapes {
    pub struct Tagged<T>(pub T);
}

fn tagged<T>(shapes::Tagged<T>(v)) -> T {
    v
}

// An array pattern spells an array of the type its elements spell, as many as it has.
fn array([Px(a), Px(b), c]) -> u32 {
    a + b + c.0
}

// Behind a reference too.
fn array_ref(&[Px(a), Px(b)]) -> u32 {
    a + b
}

fn main() {
    let rect = Rect { x: 1, y: 2, width: 3, height: 4 };
    assert_eq!(area(2, Rect { ..rect }), 24);
    assert_eq!(unwrap(Wrapper { s: "abc", v: 'x' }), (3, 'x'));
    assert_eq!(unwrap_type_like(Wrapper { s: "ab", v: 'y' }), (2, 'y'));
    assert_eq!(bound(Rect { ..rect }), (1, 2));
    assert_eq!(tuple_struct(Px(2)), 2);
    assert_eq!(by_shared_ref(&rect), 3);
    let mut px = Px(1);
    assert_eq!(by_mut(&mut px), 2);
    assert_eq!(*by_ref_out(&px), 1);
    assert_eq!(tuple((Px(1), rect)), 2);
    assert_eq!(nested_ref(&(Px(1), Wrapper { s: "", v: 2 })), 3);
    assert_eq!(nested_type_like(&(Px(1), Wrapper { s: "", v: 3 })), 4);
    assert_eq!(tagged(shapes::Tagged(2)), 2);
    assert_eq!(array([Px(1), Px(2), Px(3)]), 6);
    assert_eq!(array_ref(&[Px(7), Px(8)]), 15);
}
