#![feature(panic_always_abort)]

use ctor::ctor;

// No need to be public.
#[ctor]
fn invoke_always_abort() {
    std::panic::always_abort();
}
