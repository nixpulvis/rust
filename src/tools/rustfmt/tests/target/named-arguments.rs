#![feature(struct_args, named_arguments, default_field_values)]

fn text(text: &str; size: u32 = 16, bold: bool = false) -> String {
    format!("{text}{size}{bold}")
}

fn copy<P: AsRef<Path>, Q: AsRef<Path>>(; from: P, to: Q) -> io::Result<u64> {}

fn nothing(;) {}

// Too long for one line: one parameter per line, with the `;` on its own.
fn crop(
    img: &Image,
    ;
    width: u32,
    height: Option<u32> = None,
    x: u32 = 0,
    mut y: u32 = 0,
    z: u8 = 0,
) -> Image {
}

fn request(
    url: impl Into<Url>,
    client: &Client,
    ;
    timeout: Option<Duration> = None,
    redirects: bool = true,
) {
}

fn crop_only(
    ;
    width: u32,
    height: Option<u32> = None,
    x: u32 = 0,
    y: u32 = 0,
    label: &str = "crop",
) {
}

// Comments are kept as written.
fn commented(
    a: u32; // the positional one
    b: u32,
) {
}

fn main() {
    text("hi", size: 2, bold: true);
    text("hi");
    text("hi", _ { size: 1, .. });
}
