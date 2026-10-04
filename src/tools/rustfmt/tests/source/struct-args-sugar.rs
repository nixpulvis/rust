#![feature(struct_args, struct_args_sugar, default_field_values)]

fn text(text: &str; size: u32 = 16, bold: bool = false) -> String {
    format!("{text}{size}{bold}")
}

fn main() {
    text("hi",   size: 2, bold: true);
    text("hi");
    text( "hi", _ { size: 1, .. });
}
