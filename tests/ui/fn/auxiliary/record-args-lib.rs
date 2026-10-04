//@ edition: 2024
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

pub fn crop<'a>(img: &'a str, _ { width: u32, x: u32 = 0, label: &'a str = "crop" }) -> String {
    format!("{label}: {img} {width} at {x}")
}

pub fn map<T, U>(v: Vec<T>, _ { f: impl Fn(&T) -> U, limit: usize = 10 }) -> Vec<U> {
    v.iter().take(limit).map(f).collect()
}

pub trait Writer {
    fn write(&mut self, _ { data: &'static str, times: usize = 1 }) -> String;
}

pub struct Buffer;
impl Writer for Buffer {
    fn write(&mut self, _ { times: usize, data: &'static str }) -> String {
        data.repeat(times)
    }
}

pub struct Options {
    pub verbose: bool = false,
}

pub fn run(Options { verbose }) -> bool {
    verbose
}
