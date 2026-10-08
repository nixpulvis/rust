fn main() {
    let _ = {foo: (), bar: {} }; //~ ERROR struct literal body without path
    //~| NOTE struct name missing for struct literal
    //~| HELP add the correct type
    let _ = _ {foo: (), bar: {} }; //~ ERROR record parameters and arguments are experimental
    //~| HELP add `#![feature(struct_args)]` to the crate attributes to enable
    //~| NOTE this compiler was built on YYYY-MM-DD; consider upgrading it if it is out of date
    let _ = {foo: ()}; //~ ERROR struct literal body without path
    //~| NOTE struct name missing for struct literal
    //~| HELP add the correct type
    let _ = _ {foo: ()}; //~ ERROR record parameters and arguments are experimental
    //~| HELP add `#![feature(struct_args)]` to the crate attributes to enable
    //~| NOTE this compiler was built on YYYY-MM-DD; consider upgrading it if it is out of date
}
