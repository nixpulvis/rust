// Leaving out a parameter's type when its pattern spells it out needs `parameter_type_elision`.

struct Args {
    y: u32,
}

struct Pair(u32, u32);

// The pattern implies the type `Args`.
fn f(Args { y }) -> u32 {
    //~^ ERROR parameter type elision is experimental
    y
}

// The pattern implies the type `(&Args, Pair)`.
fn g((&Args { y }, Pair(a, b))) -> u32 {
    //~^ ERROR parameter type elision is experimental
    y + a + b
}

// Spelling out the type doesn't need it, and calls are the same.
fn h(Args { y }: Args) -> u32 {
    y
}

// Generic arguments without a turbofish in a parameter's pattern spell its type, so they need it
// even when the type is written.
struct Wrap<T>(T);
fn k(Wrap<u32>(x): Wrap<u32>) -> u32 {
    //~^ ERROR parameter type elision is experimental
    x
}

fn main() {
    f(Args { y: 1 });
    g((&Args { y: 1 }, Pair(2, 3)));
    h(Args { y: 1 });
    k(Wrap(1));
}
