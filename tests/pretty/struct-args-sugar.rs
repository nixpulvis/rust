//@ pp-exact
// Named arguments (`#![feature(struct_args_sugar)]`) print as written.

#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

fn text(text: &str, _ { size: u32 = 16, bold: bool = false }) -> u32 { size }

fn main() {
    text("a", size: 2, bold: true);
    text("b", size: 3, ..);
    text("c");
}
