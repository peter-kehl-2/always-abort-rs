#![feature(const_black_box)]
use std::panic;
// We must have `use import_transitive_lib;`. Otherwise it doesn't (transitively) invoke s `ctor`
// function of `always-abort`.
#[allow(unused_imports)]
use import_transitive_lib;

// @TODO PR to ctor that we do have to `use` the crate - even if we don't `use` (directly/explicitly) anything from within it.

fn main() {
    // From https://doc.rust-lang.org/nightly/std/panic/fn.always_abort.html
    let _ = panic::catch_unwind(|| {
        //always_abort::go_panic();
        panic!("inside the catch");
    });

    // We will have aborted already, due to the panic.
    unreachable!();
}
