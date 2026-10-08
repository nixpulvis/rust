//@ run-pass
//@ edition: 2024
// Record parameters and record arguments (`#![feature(struct_args)]`).
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

#[derive(Debug, PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn crop(
    _ { x, y, width, height }: struct { x: u32, y: u32, width: u32, height: u32 = 100 },
) -> Rect {
    Rect { x, y, width, height }
}

// A named argument struct, taken with a struct pattern, is called the same way.
pub struct CropArgs {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32 = 100,
}

fn crop_named(CropArgs { x, y, width, height }: CropArgs) -> Rect {
    Rect { x, y, width, height }
}

// A trait's record parameter is shared by its impls, which repeat its fields but not its
// defaults, and by calls through generics and trait objects.
trait Writer {
    fn write(&mut self, _: struct { data: &'static str, times: usize = 1 }) -> String;
    fn describe(&self, _ { prefix }: struct { prefix: &'static str = "> " }) -> String {
        format!("{prefix}writer")
    }
}

struct Buffer(Vec<String>);
impl Writer for Buffer {
    fn write(&mut self, _ { data, times }: struct { data: &'static str, times: usize }) -> String {
        let s = data.repeat(times);
        self.0.push(s.clone());
        s
    }
}

struct Loud;
impl Writer for Loud {
    fn write(&mut self, _ { times, data }: struct { times: usize, data: &'static str }) -> String {
        data.to_uppercase().repeat(times)
    }
    fn describe(&self, _ { prefix }: struct { prefix: &'static str }) -> String {
        format!("{prefix}LOUD")
    }
}

fn write_twice<W: Writer>(w: &mut W) -> String {
    w.write(_ { data: "g", times: 2 })
}

// Record fields can use what's in scope in the signature: generic parameters, elided
// lifetimes, `impl Trait`, and `Self`. Fields with defaults can use type and const parameters,
// and lifetimes: elided ones are elided in the signature too, named ones are the function's.
fn map<T, U>(
    v: Vec<T>,
    _ { f, label, limit }: struct { f: impl Fn(&T) -> U, label: &str, limit: usize = 10 },
) -> (String, Vec<U>) {
    (label.to_string(), v.iter().take(limit).map(f).collect())
}

fn collect<T, const N: usize>(
    _ { item, extra, arr }: struct { item: T, extra: Vec<T> = Vec::new(), arr: [u8; N] = [0; N] },
) -> (Vec<T>, usize) {
    let mut v = extra;
    v.push(item);
    (v, arr.len())
}

fn say(
    from: &str,
    _ { device, tag }: struct { device: &str = "carrier pigeon", tag: Option<&'_ str> = None },
) -> String {
    format!("{from} with a {device}{}", tag.unwrap_or(""))
}

// A lone elided input lifetime, here in the record, is the output's.
fn prefix(_ { s, n }: struct { s: &str = "abc", n: usize = 1 }) -> &str {
    &s[..n]
}

fn pick<'a>(x: &'a str, _ { fallback }: struct { fallback: &'a str = "none" }) -> &'a str {
    if x.is_empty() { fallback } else { x }
}

// Lifetimes of `fn` pointers and `Fn` bounds in a field's type are their own.
fn apply(
    _ { f, g }: struct {
        f: fn(&str) -> usize = str::len,
        g: &dyn Fn(&str) -> usize = &|s: &str| s.len(),
    },
) -> usize {
    f("ab") + g("abc")
}

// In a trait, impls repeat a field with an elided lifetime like any other.
trait Label {
    fn label(&self, _: struct { pre: &str = "<", post: &str = ">" }) -> String;
}
impl Label for u8 {
    fn label(&self, _ { post, pre }: struct { post: &str, pre: &str }) -> String {
        format!("{pre}{self}{post}")
    }
}

trait Store<K> {
    type Item;
    fn put(
        &mut self,
        _: struct { key: K, value: Self::Item, note: &str, times: usize = 1 },
    ) -> usize;
}

struct Log(Vec<String>);
impl<K: std::fmt::Display> Store<K> for Log {
    type Item = u32;
    fn put(
        &mut self,
        _ { value, note, key, times }: struct { value: u32, note: &str, key: K, times: usize },
    ) -> usize {
        for _ in 0..times {
            self.0.push(format!("{key}={value} ({note})"));
        }
        self.0.len()
    }
}

#[derive(Debug, PartialEq)]
struct Point(i32, i32);
impl Point {
    fn with(&self, _ { other, scale }: struct { other: Self, scale: i32 = 1 }) -> Self {
        Point(self.0 + other.0 * scale, self.1 + other.1 * scale)
    }
}

struct Sink(Vec<u8>);
impl Sink {
    fn write(&mut self, _ { byte, times }: struct { byte: u8, times: usize = 1 }) {
        for _ in 0..times {
            self.0.push(byte);
        }
    }
}

// A method with the same name in the same module gets its own argument struct.
struct Echo;
impl Echo {
    fn write(&self, _ { byte }: struct { byte: u8 }) -> u8 {
        byte
    }
    fn write_twice(&self, _ { byte }: struct { byte: u8 }) -> [u8; 2] {
        [byte; 2]
    }
}

fn pair(_ { a, b }: struct { a: u32, b: u32 }) -> (u32, u32) {
    (a, b)
}

struct Counter(u32);
impl Counter {
    const fn new(_ { start }: struct { start: u32 = 0 }) -> Self {
        Counter(start)
    }
    fn bump(&mut self, _ { by }: struct { by: u32 = 1 }) {
        self.0 += by;
    }
}
const ZERO: Counter = Counter::new(_ { .. });

// A field can be bound mutably, as a positional parameter can. Impls choose their own binding
// modes.
fn countdown(_ { mut n, step }: struct { n: u32, step: u32 = 1 }) -> Vec<u32> {
    let mut v = vec![];
    while n > 0 {
        v.push(n);
        n = n.saturating_sub(step);
    }
    v
}

// And by reference, as with `fn f(ref x: String)`, to borrow one field while moving another.
fn split(
    _ { ref label, ref mut log, owned }: struct { label: String, log: Vec<String>, owned: String },
) -> usize {
    log.push(owned);
    label.len() + log.len()
}

trait Grow {
    fn grow(&self, _: struct { by: u32 }) -> u32;
    fn grow_by(&self, args: struct { by: u32, times: u32 = 1 }) -> u32;
}
impl Grow for u32 {
    fn grow(&self, _ { mut by }: struct { by: u32 }) -> u32 {
        by += *self;
        by
    }
    fn grow_by(&self, args: struct { times: u32, by: u32 }) -> u32 {
        *self + args.by * args.times
    }
}

// A parameter whose type is an anonymous struct is an ordinary `pat: ty`, so `r` binds the whole
// struct and nothing else, even when its fields don't implement `Copy`.
fn area(r: struct { w: u32, h: u32, label: String }) -> (u32, String) {
    let r2 = r;
    (r2.w * r2.h, r2.label)
}

fn named_mut(mut p: struct { n: u32 }) -> u32 {
    p.n += 4;
    p.n
}

// `_ { .. }` is a struct pattern whose struct is the type it matches, so a parameter's anonymous
// struct can be written `pat: ty` with any pattern for its fields.
fn renamed(_ { x: y }: struct { x: u32 }) -> u32 {
    y
}

fn by_ref(_ { ref s }: struct { s: String }) -> usize {
    s.len()
}

fn some_fields(_ { x, .. }: struct { x: u32, y: u32 = 2 }) -> u32 {
    x
}

fn nested_pat(_ { inner: Rect { width, .. } }: struct { inner: Rect }) -> u32 {
    width
}

// A field of an anonymous struct may be one too, with defaults of its own. Its pattern only
// binds the outer fields, so `at` is the whole inner struct, which `_ { .. }` can take apart.
fn place(
    _ { at, scale }: struct { at: struct { x: u32, y: u32 = 0 }, scale: u32 = 1 },
) -> (u32, u32) {
    let _ { x, y } = at;
    (x * scale, y * scale)
}

fn place_label(
    _ { at: _ { ref label, .. } }: struct { at: struct { label: String, n: u8 = 0 } },
) -> usize {
    label.len()
}

fn nested_generic<T: Copy, U>(
    _ { inner }: struct { inner: struct { v: T, f: impl Fn(T) -> U } },
) -> U {
    (inner.f)(inner.v)
}

trait Place {
    fn place(&self, _: struct { at: struct { x: u32, y: u32 = 7 } }) -> u32;
}
impl Place for u32 {
    fn place(&self, _ { at }: struct { at: struct { y: u32, x: u32 } }) -> u32 {
        self + at.x + at.y
    }
}

// Positional parameters keep their patterns.
fn offset((x, y): (i32, i32), _ { scale }: struct { scale: i32 = 1 }) -> i32 {
    (x + y) * scale
}

fn scale(_ { by }: struct { by: u32 = 2 }, n: u32) -> u32 {
    n * by
}

fn blend(
    _ { r, g }: struct { r: u8, g: u8 },
    _ { alpha }: struct { alpha: u8 = 255 },
) -> (u8, u8, u8) {
    (r, g, alpha)
}

trait Mix {
    fn mix(&self, _: struct { a: u32 }, _: struct { b: u32 = 1 }) -> u32;
}
impl Mix for u32 {
    fn mix(&self, _ { a }: struct { a: u32 }, _ { b }: struct { b: u32 }) -> u32 {
        self + a * b
    }
}

macro_rules! make_double {
    ($name:ident) => {
        fn $name(_ { v }: struct { v: u32 }) -> u32 {
            v * 2
        }
    };
}
make_double!(double);

fn main() {
    let expected = Rect { x: 10, y: 20, width: 200, height: 100 };

    // Any order, `..` for defaults, field-init shorthand.
    assert_eq!(crop(_ { x: 10, y: 20, width: 200, .. }), expected);
    assert_eq!(crop(_ { width: 200, height: 5, x: 10, y: 20 }).height, 5);
    let (x, y, width) = (10, 20, 200);
    assert_eq!(crop(_ { x, y, width, .. }), expected);

    // Named argument struct: inferred or written out, and forwarding.
    assert_eq!(crop_named(_ { x: 10, y: 20, width: 200, .. }), expected);
    assert_eq!(crop_named(CropArgs { x: 10, y: 20, width: 200, .. }), expected);
    let opts = CropArgs { x: 10, y: 20, width: 1, .. };
    assert_eq!(crop_named(_ { width: 200, ..opts }), expected);

    // Methods.
    let mut sink = Sink(vec![]);
    sink.write(_ { byte: 7, times: 2 });
    sink.write(_ { byte: 9, .. });
    sink.write(_ { byte: 9, times: 1 });
    assert_eq!(sink.0, [7, 7, 9, 9]);
    assert_eq!(Echo.write(_ { byte: 3 }), 3);
    assert_eq!(Echo.write_twice(_ { byte: 4 }), [4, 4]);

    // Record arguments are evaluated in the order they are written.
    let mut n = 0;
    let mut next = || {
        n += 1;
        n
    };
    assert_eq!(pair(_ { b: next(), a: next() }), (2, 1));

    // A `const fn` with a record parameter.
    let mut counter = Counter::new(_ { .. });
    counter.bump(_ { .. });
    counter.bump(_ { by: 2 });
    assert_eq!((counter.0, ZERO.0), (3, 0));

    // Mutable bindings, and bindings of the whole record.
    assert_eq!(countdown(_ { n: 5, step: 2 }), [5, 3, 1]);
    assert_eq!(countdown(_ { n: 2, .. }), [2, 1]);
    assert_eq!(split(_ { label: "ab".into(), log: vec![], owned: "c".into() }), 3);
    assert_eq!(3.grow(_ { by: 4 }), 7);
    assert_eq!(3.grow_by(_ { by: 4, times: 2 }), 11);
    assert_eq!(3.grow_by(_ { by: 4, .. }), 7);
    let grow: &dyn Grow = &1u32;
    assert_eq!(grow.grow_by(_ { by: 2, .. }), 3);
    assert_eq!(area(_ { w: 2, h: 3, label: String::from("a") }), (6, String::from("a")));
    assert_eq!(named_mut(_ { n: 4 }), 8);
    assert_eq!(offset((1, 2), _ { scale: 2 }), 6);
    assert_eq!(offset((1, 2), _ { .. }), 3);

    // `_ { .. }` patterns, in parameters and anywhere else a struct is matched.
    assert_eq!(renamed(_ { x: 3 }), 3);
    assert_eq!(by_ref(_ { s: String::from("ab") }), 2);
    assert_eq!(some_fields(_ { x: 1, .. }), 1);
    assert_eq!(nested_pat(_ { inner: crop(_ { x: 0, y: 0, width: 7, .. }) }), 7);
    let _ { width, height: h, .. } = crop(_ { x: 0, y: 0, width: 4, .. });
    assert_eq!((width, h), (4, 100));
    match Some(&expected) {
        Some(_ { x: 10, y, .. }) => assert_eq!(*y, 20),
        _ => unreachable!(),
    }

    // Anonymous structs in the fields of others.
    assert_eq!(place(_ { at: _ { x: 2, .. }, .. }), (2, 0));
    assert_eq!(place(_ { at: _ { x: 2, y: 3 }, scale: 2 }), (4, 6));
    assert_eq!(place_label(_ { at: _ { label: String::from("abc"), .. } }), 3);
    assert_eq!(nested_generic(_ { inner: _ { v: 2, f: |v| v * 10 } }), 20);
    assert_eq!(1.place(_ { at: _ { x: 2, .. } }), 10);
    let p: &dyn Place = &1u32;
    assert_eq!(p.place(_ { at: _ { x: 2, y: 0 } }), 3);

    // Function pointers keep the argument struct as their parameter, and with it its names and
    // defaults.
    let f = crop;
    assert_eq!(f(_ { x: 10, y: 20, width: 200, .. }), expected);
    let g: fn(_) -> Rect = crop;
    assert_eq!(g(_ { width: 200, y: 20, x: 10, .. }), expected);
    let p: fn(&str, _) -> String = say;
    assert_eq!(p("Al", _ { .. }), "Al with a carrier pigeon");

    // In a condition, where struct literals need parentheses elsewhere.
    if pair(_ { a: 1, b: 2 }).0 == 1 {
    } else {
        unreachable!();
    }

    // Functions from macros and in blocks.
    assert_eq!(double(_ { v: 21 }), 42);
    fn len(_ { s }: struct { s: &'static str }) -> usize {
        s.len()
    }
    assert_eq!(len(_ { s: "abc" }), 3);

    // With no struct to infer, a record argument needs a type annotation (see the errors test).
    let rect: Rect = _ { x: 10, y: 20, width: 200, height: 100 };
    assert_eq!(rect, expected);

    // Traits.
    let mut buffer = Buffer(vec![]);
    assert_eq!(buffer.write(_ { data: "ab", times: 2 }), "abab");
    assert_eq!(buffer.write(_ { data: "c", .. }), "c");
    assert_eq!(buffer.describe(_ { .. }), "> writer");
    assert_eq!(write_twice(&mut buffer), "gg");
    let writer: &mut dyn Writer = &mut Loud;
    assert_eq!(writer.write(_ { data: "x", times: 3 }), "XXX");
    assert_eq!(writer.describe(_ { prefix: "* " }), "* LOUD");

    // Generics.
    let (label, doubled) = map(vec![1, 2, 3], _ { f: |x| x * 2, label: "doubled", .. });
    assert_eq!((label.as_str(), doubled), ("doubled", vec![2, 4, 6]));
    assert_eq!(collect::<_, 3>(_ { item: 'x', .. }), (vec!['x'], 3));
    assert_eq!(collect(_ { item: 1, extra: vec![0], arr: [0; 2] }), (vec![0, 1], 2));
    let mut log = Log(vec![]);
    let note = String::from("borrowed");
    assert_eq!(log.put(_ { key: "a", value: 1, note: &note, .. }), 1);
    let store: &mut dyn Store<i32, Item = u32> = &mut log;
    assert_eq!(store.put(_ { key: 7, value: 2, note: "x", times: 2 }), 3);
    assert_eq!(Point(1, 1).with(_ { other: Point(1, 2), scale: 2 }), Point(3, 5));
    let device = String::from("smoke signal");
    assert_eq!(say("Bob", _ { .. }), "Bob with a carrier pigeon");
    let tagged = say("Bob", _ { device: &device, tag: Some(&device[..1]) });
    assert_eq!(tagged, "Bob with a smoke signals");
    assert_eq!(prefix(_ { .. }), "a");
    assert_eq!(pick("", _ { fallback: &device }), "smoke signal");
    assert_eq!(apply(_ { .. }), 5);
    assert_eq!(1u8.label(_ { pre: &device[..1], .. }), "s1>");
    let label: &dyn Label = &2u8;
    assert_eq!(label.label(_ { .. }), "<2>");

    // Record parameters in any position, and more than one.
    assert_eq!(scale(_ { .. }, 3), 6);
    assert_eq!(scale(_ { by: 3 }, 3), 9);
    assert_eq!(blend(_ { r: 1, g: 2 }, _ { alpha: 3 }), (1, 2, 3));
    assert_eq!(blend(_ { r: 1, g: 2 }, _ { .. }), (1, 2, 255));
    assert_eq!(5u32.mix(_ { a: 2 }, _ { b: 3 }), 11);
    let mix: &dyn Mix = &5u32;
    assert_eq!(mix.mix(_ { a: 1 }, _ { .. }), 6);
}
