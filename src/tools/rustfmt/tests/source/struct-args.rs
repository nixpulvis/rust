#![feature(struct_args, struct_args_pattern_elision, default_field_values)]

fn crop(img: &u8,   struct { width: u32, height: Option<u32> = None }) -> u32 {
    width + height.unwrap_or(0) + crop2(_ {width:1, ..})
}
fn crop2(p: struct { width: u32, x: u32 = 0 }) -> u32 { p.width + p.x }
fn crop3(_ {width:w,  ..}: struct { width: u32, x: u32 = 0 }) -> u32 {
    let _ {x,y:  z,..} = point();
    w
}
