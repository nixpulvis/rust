//@ check-pass
// The anonymous struct of a record parameter (`#![feature(struct_args)]`) is documented, named and
// used with its function, so lints on items don't apply to it on its own.
#![feature(struct_args, default_field_values)]
#![allow(incomplete_features)]
#![deny(missing_docs, dead_code, non_camel_case_types, unreachable_pub)]

//! Records and lints.

/// A documented function whose record has fields the body doesn't use.
pub fn documented(_ { used: u32, unused_field: u32 = 0 }) -> u32 {
    used
}

/// A documented trait whose method's record no impl reads.
pub trait Documented {
    /// A method with a record.
    fn method(&self, _ { a: u32, b_c: u32 = 1 });
}

impl Documented for () {
    fn method(&self, _ { a: u32, b_c: u32 }) {
        let _ = (a, b_c);
    }
}

mod private {
    pub(crate) fn crate_visible(_ { x: u32 }) -> u32 {
        x
    }
}

/// Uses the private function.
pub fn uses() -> u32 {
    private::crate_visible(_ { x: 1 })
}

fn main() {}
