//@ edition: 2024
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

pub fn crop<'a>(
    img: &'a str,
    _ { width, x, label }: struct { width: u32, x: u32 = 0, label: &'a str = "crop" },
) -> String {
    format!("{label}: {img} {width} at {x}")
}

pub fn map<T, U>(
    v: Vec<T>,
    _ { f, limit }: struct { f: impl Fn(&T) -> U, limit: usize = 10 },
) -> Vec<U> {
    v.iter().take(limit).map(f).collect()
}

pub trait Writer {
    fn write(&mut self, _: struct { data: &'static str, times: usize = 1 }) -> String;
}

pub struct Buffer;
impl Writer for Buffer {
    fn write(&mut self, _ { times, data }: struct { times: usize, data: &'static str }) -> String {
        data.repeat(times)
    }
}

pub struct Options {
    pub verbose: bool = false,
}

pub fn run(Options { verbose }: Options) -> bool {
    verbose
}

// The anonymous struct of a field of another, with the function's generics and lifetimes.
pub trait Draw {
    fn draw<'a>(
        &self,
        _: struct { at: struct { x: u32, y: u32 = 0 }, label: &'a str = "pt" },
    ) -> String;
}

pub struct Canvas;
impl Draw for Canvas {
    fn draw<'a>(
        &self,
        _ { label, at }: struct { label: &'a str, at: struct { y: u32, x: u32 } },
    ) -> String {
        format!("{label} {} {}", at.x, at.y)
    }
}
