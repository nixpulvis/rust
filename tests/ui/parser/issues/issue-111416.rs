fn main() {
    let my = monad_bind(mx, T: Try);
    //~^ ERROR named arguments are experimental
    //~| ERROR cannot find function `monad_bind` in this scope
    //~| ERROR cannot find value `mx` in this scope
    //~| ERROR cannot find value `Try` in this scope
}
