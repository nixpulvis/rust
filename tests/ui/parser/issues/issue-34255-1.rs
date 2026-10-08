enum Test {
    Drill {
        field: i32,
    }
}

fn main() {
    Test::Drill(field: 42);
    //~^ ERROR named arguments are experimental
    //~| ERROR expected value, found struct variant `Test::Drill`
}
