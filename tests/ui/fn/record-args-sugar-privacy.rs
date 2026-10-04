//@ edition: 2024
// The `..` that named arguments imply checks private defaulted fields, as a written `..` does
// (`#![feature(struct_args_sugar)]`).
#![feature(struct_args, struct_args_sugar, default_field_values)]
#![allow(incomplete_features)]

mod style {
    pub struct Style {
        pub bold: bool = false,
        secret: u8 = 0,
    }
}

fn draw(style: style::Style) -> bool {
    style.bold
}

fn main() {
    draw(bold: true);
    //~^ ERROR field `secret` of struct `Style` is private
    let _ = style::Style { bold: true, .. };
    //~^ ERROR field `secret` of struct `Style` is private
}
