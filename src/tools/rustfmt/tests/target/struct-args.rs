#![feature(struct_args, default_field_values)]

fn crop(img: &u8, _ { width: u32, height: Option<u32> = None }) -> u32 {
    width + height.unwrap_or(0) + crop2(_ { width: 1, .. })
}
fn crop2(p: _ { width: u32, x: u32 = 0 }) -> u32 {
    p.width + x
}
