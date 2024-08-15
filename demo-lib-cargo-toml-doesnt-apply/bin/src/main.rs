use std::panic;

fn main() {
    // From https://doc.rust-lang.org/nightly/std/panic/fn.always_abort.html
    let _ = panic::catch_unwind(|| {
        //always_abort::go_panic();
        panic!("inside the catch");
    });

    // We will have aborted already, due to the panic.
    unreachable!();
}
