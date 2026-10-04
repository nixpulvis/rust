//@ run-pass
//@ edition: 2024
// A record (`#![feature(struct_args)]`) is a child of its function and shares its generics, so its
// fields, and their defaults, can use anything the function's signature can.
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

use std::borrow::Cow;
use std::fmt::Debug;

// A lifetime elided in a path, in a field with a default.
fn label(_ { s: Cow<str> = Cow::Borrowed("none"), n: usize = 1 }) -> String {
    s.repeat(n)
}

// Defaults that use `Self`, and an associated type through the function's bounds.
#[derive(Debug, PartialEq)]
struct Point(i32, i32);
impl Point {
    fn with(&self, _ { other: Option<Self> = None, scale: i32 = 1 }) -> Self {
        let other = other.unwrap_or(Point(0, 0));
        Point(self.0 + other.0 * scale, self.1 + other.1 * scale)
    }
}

fn first<I: Iterator<Item: Debug>>(_ { iter: I, fallback: Option<I::Item> = None }) -> String {
    let mut iter = iter;
    format!("{:?}", iter.next().or(fallback))
}

// In traits, defaults can use the trait's and the method's generic parameters and lifetimes.
trait Collect<T> {
    fn collect<'a>(&self, _ { items: Vec<T> = Vec::new(), label: &'a str = "items" }) -> String;
}
impl<T: Debug> Collect<T> for () {
    fn collect<'a>(&self, _ { label: &'a str, items: Vec<T> }) -> String {
        format!("{label}: {items:?}")
    }
}

// The impl's generic parameters are the record's too.
struct Wrapper<T>(Vec<T>);
impl<T: Clone> Wrapper<T> {
    fn push(&mut self, _ { value: T, times: usize = 1 }) {
        for _ in 0..times {
            self.0.push(value.clone());
        }
    }
}

// Functions with records from macros, as items, in impls, and in traits.
macro_rules! double {
    ($name:ident) => {
        fn $name(_ { v: u32 }) -> u32 {
            v * 2
        }
    };
}
double!(double_free);

macro_rules! method {
    () => {
        fn scaled(&self, _ { by: u32 = 2 }) -> u32 {
            self.0 * by
        }
    };
}
struct Num(u32);
impl Num {
    method!();
}

macro_rules! trait_method {
    () => {
        fn shout(&self, _ { times: usize = 1 }) -> String;
    };
}
trait Shout {
    trait_method!();
}
impl Shout for &str {
    fn shout(&self, _ { times: usize }) -> String {
        self.to_uppercase().repeat(times)
    }
}

// Functions with records under `#[cfg]`.
#[cfg(false)]
fn configured(_ { a: u32 }) -> u32 {
    a
}
#[cfg(not(false))]
fn configured(_ { b: u32 = 3 }) -> u32 {
    b
}

// Async functions.
async fn delayed(_ { v: u32, extra: u32 = 1 }) -> u32 {
    v + extra
}

fn block_on<F: Future>(f: F) -> F::Output {
    let mut f = std::pin::pin!(f);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

fn main() {
    assert_eq!(label(_ { .. }), "none");
    let owned = String::from("ab");
    assert_eq!(label(_ { s: Cow::Borrowed(&owned), n: 2 }), "abab");

    let p = Point(1, 1);
    assert_eq!(p.with(_ { .. }), Point(1, 1));
    assert_eq!(p.with(_ { other: Some(Point(1, 2)), scale: 2 }), Point(3, 5));
    assert_eq!(first(_ { iter: [1u8, 2].into_iter(), .. }), "Some(1)");
    assert_eq!(first(_ { iter: std::iter::empty(), fallback: Some('x') }), "Some('x')");

    assert_eq!(().collect(_ { items: vec![1, 2], .. }), "items: [1, 2]");
    assert_eq!(Collect::<u8>::collect(&(), _ { label: "none", .. }), "none: []");

    let mut w = Wrapper(vec![]);
    w.push(_ { value: 'a', times: 2 });
    w.push(_ { value: 'b', .. });
    assert_eq!(w.0, ['a', 'a', 'b']);

    assert_eq!(double_free(_ { v: 21 }), 42);
    assert_eq!(Num(4).scaled(_ { .. }), 8);
    assert_eq!("hi".shout(_ { times: 2 }), "HIHI");
    let shout: &dyn Shout = &"yo";
    assert_eq!(shout.shout(_ { .. }), "YO");

    assert_eq!(configured(_ { .. }), 3);
    assert_eq!(block_on(delayed(_ { v: 1, .. })), 2);
}
