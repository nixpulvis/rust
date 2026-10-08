macro_rules! foo {
    ($rest: tt) => {
        bar(baz: $rest)
        //~^ ERROR named arguments are experimental
        //~| ERROR cannot find function `bar` in this scope
    }
}

fn main() {
    foo!(true);
}
