#![feature(panic_always_abort)]

use ctor::ctor;

#[ctor]
fn invoke_always_abort() {
    std::panic::always_abort();
}
