//@ run-pass
//@ edition: 2024
// Sugar for record parameters and arguments (`#![feature(struct_args_sugar)]`): named arguments,
// `f(a, x: 1)` for `f(a, _ { x: 1, .. })` whatever struct the last parameter is, leaving out a
// record whose fields all have defaults, `f(a)`, and `;` in a parameter list,
// `fn f(a: i32; x: i32)` for `fn f(a: i32, _ { x: i32 })`.
#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

fn greet(prefix: &str, _ { name: &str, times: usize = 1 }) -> String {
    format!("{prefix}{name}").repeat(times)
}

#[derive(Clone, Copy)]
struct Opts {
    verbose: bool = false,
    level: u8 = 1,
}

fn configure(o: Opts) -> u8 {
    if o.verbose { o.level * 10 } else { o.level }
}

fn configure_untyped(Opts { verbose, level }) -> u8 {
    if verbose { level * 10 } else { level }
}

struct Sink;

impl Sink {
    fn write(&self, _ { byte: u8, times: usize = 1 }) -> usize {
        byte as usize * times
    }
}

macro_rules! greet_all {
    ($($args:tt)*) => { greet("", $($args)*) };
}

// `;` starts the fields of a last record parameter.
fn crop(img: &str; width: u32, height: Option<u32> = None, x: u32 = 0, y: u32 = 0) -> String {
    format!("{img}: {width}x{} at {x},{y}", height.unwrap_or(width))
}

fn copy<P: AsRef<str>, Q: AsRef<str>>(; from: P, to: Q) -> String {
    format!("{} -> {}", from.as_ref(), to.as_ref())
}

fn countdown(; mut n: u32, step: u32 = 1) -> Vec<u32> {
    let mut v = vec![];
    while n > 0 {
        v.push(n);
        n = n.saturating_sub(step);
    }
    v
}

fn nothing(;) -> u8 {
    7
}

trait Writer {
    fn write(&mut self; data: &'static str, times: usize = 1) -> String;
}

struct Buffer;
impl Writer for Buffer {
    // Switching between `;` and `_ { .. }` changes nothing for callers.
    fn write(&mut self, _ { times: usize, data: &'static str }) -> String {
        data.repeat(times)
    }
}

struct Loud;
impl Writer for Loud {
    fn write(&mut self; times: usize, data: &'static str) -> String {
        data.to_uppercase().repeat(times)
    }
}

fn text(text: &str, _ { font_size: u32 = 16 }) -> String {
    format!("{text}@{font_size}")
}

struct Counter(u32);
impl Counter {
    const fn new(_ { start: u32 = 0 }) -> Self {
        Counter(start)
    }
    fn bump(&mut self; by: u32 = 1) {
        self.0 += by;
    }
}
const ZERO: Counter = Counter::new();

fn blend(_ { r: u8, g: u8 }, _ { alpha: u8 = 255 }) -> (u8, u8, u8) {
    (r, g, alpha)
}

fn main() {
    // Any order, `..` for defaults, and a trailing comma.
    assert_eq!(greet("> ", name: "a", ..), "> a");
    assert_eq!(greet("> ", times: 2, name: "b"), "> b> b");
    assert_eq!(greet("", name: "c", times: 3,), "ccc");
    // Named arguments imply `..`, for records and named structs alike.
    assert_eq!(greet("> ", name: "d"), "> d");
    assert_eq!(configure(verbose: true), 10);
    assert_eq!(configure_untyped(level: 4), 4);

    // Any struct-typed last parameter, with defaults or a base.
    let base = Opts { verbose: true, level: 2 };
    assert_eq!(configure(level: 5, ..base), 50);
    assert_eq!(configure(verbose: true, ..), 10);
    assert_eq!(configure_untyped(level: 3, ..), 3);

    // Methods, `fn` pointers, macros, and struct-literal-restricted positions.
    assert_eq!(Sink.write(byte: 3, ..), 3);
    assert_eq!(Sink.write(byte: 3, times: 2), 6);
    let f = greet;
    assert_eq!(f("", name: "d", ..), "d");
    assert_eq!(greet_all!(name: "e", times: 2), "ee");
    if greet("", name: "f", ..) != "f" {
        unreachable!();
    }

    // `;` parameters, called with named arguments or a record argument.
    assert_eq!(crop("cat", width: 200), "cat: 200x200 at 0,0");
    assert_eq!(crop("cat", width: 200, y: 25, x: 15, height: Some(100)), "cat: 200x100 at 15,25");
    assert_eq!(crop("cat", _ { width: 1, .. }), "cat: 1x1 at 0,0");
    assert_eq!(copy(from: "a", to: "b"), "a -> b");
    assert_eq!(copy(to: "b", from: "a"), "a -> b");
    assert_eq!(countdown(n: 5, step: 2), [5, 3, 1]);
    assert_eq!(nothing(), 7);
    assert_eq!(Buffer.write(data: "hi"), "hi");
    let w: &mut dyn Writer = &mut Loud;
    assert_eq!(w.write(data: "hi", times: 2), "HIHI");

    // Leaving out a last record whose fields all have defaults.
    assert_eq!(text("hi"), "hi@16");
    assert_eq!(text("hi", font_size: 24), "hi@24");
    let mut counter = Counter::new();
    counter.bump();
    counter.bump(by: 2);
    assert_eq!((counter.0, ZERO.0), (3, 0));
    let p: fn(&str, _) -> String = text;
    assert_eq!(p("ptr"), "ptr@16");
    assert_eq!(blend(_ { r: 1, g: 2 }), (1, 2, 255));
    // Named arguments are simply the last argument written: after a positional record they fill
    // the last parameter, and when the last record is left out they fill the one before it.
    assert_eq!(blend(_ { r: 1, g: 2 }, alpha: 3), (1, 2, 3));
    assert_eq!(blend(r: 1, g: 2), (1, 2, 255));

    // `..` alone is still a positional `RangeFull`.
    let mut v = vec![1, 2, 3];
    assert_eq!(v.drain(..).count(), 3);
}
