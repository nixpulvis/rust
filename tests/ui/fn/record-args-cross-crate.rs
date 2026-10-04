//@ run-pass
//@ edition: 2024
//@ aux-build: record-args-lib.rs
// Records (`#![feature(struct_args)]`) of another crate's functions and traits.
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]

extern crate record_args_lib as lib;

use lib::Writer;

struct Loud;
impl Writer for Loud {
    fn write(&mut self, _ { data: &'static str, times: usize }) -> String {
        data.to_uppercase().repeat(times)
    }
}

fn main() {
    assert_eq!(lib::crop("cat", _ { width: 2, .. }), "crop: cat 2 at 0");
    assert_eq!(lib::crop("cat", _ { label: "c", x: 1, width: 2 }), "c: cat 2 at 1");
    assert_eq!(lib::map(vec![1, 2, 3], _ { f: |n| n * 2, limit: 2 }), [2, 4]);
    assert_eq!(lib::Buffer.write(_ { data: "ab", .. }), "ab");
    let writer: &mut dyn Writer = &mut Loud;
    assert_eq!(writer.write(_ { data: "x", times: 3 }), "XXX");
    assert!(lib::run(_ { verbose: true }));
    // `'a` is in the record, so it's early-bound, and the pointer can't be higher-ranked over it.
    let f: fn(&'static str, _) -> String = lib::crop;
    assert_eq!(f("dog", _ { width: 1, .. }), "crop: dog 1 at 0");
}
