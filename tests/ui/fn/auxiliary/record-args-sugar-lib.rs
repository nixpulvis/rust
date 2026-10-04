//@ edition: 2024
#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

pub fn text(text: &str; size: u32 = 16, bold: bool = false) -> String {
    format!("{text}@{size}{}", if bold { "!" } else { "" })
}

pub trait Greet {
    fn greet(&self; name: &'static str, times: usize = 1) -> String;
}

pub struct Hello;
impl Greet for Hello {
    fn greet(&self; name: &'static str, times: usize) -> String {
        format!("hello {name}").repeat(times)
    }
}
