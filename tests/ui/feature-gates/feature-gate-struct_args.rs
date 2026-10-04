fn f(_ { x: u32 }) -> u32 { x } //~ ERROR record parameters and arguments are experimental

struct Args {
    y: u32,
}
fn g(Args { y }) -> u32 { y } //~ ERROR record parameters and arguments are experimental

fn main() {
    f(_ { x: 1 }); //~ ERROR record parameters and arguments are experimental
}
