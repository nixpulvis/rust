// Without `struct_args_pattern_elision`, an anonymous struct parameter spells out its pattern.
fn f(_ { x }: struct { x: u32 }) -> u32 {
    //~^ ERROR record parameters and arguments are experimental
    //~| ERROR record parameters and arguments are experimental
    x
}

struct Args {
    y: u32,
}

fn main() {
    f(_ { x: 1 }); //~ ERROR record parameters and arguments are experimental
    let _ { y } = Args { y: 1 }; //~ ERROR record parameters and arguments are experimental
}
