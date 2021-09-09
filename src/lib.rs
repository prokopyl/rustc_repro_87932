// SysFoo comes from the crate mylib_sys, and is re-exported.
use mylib::{FooTrait, SysFoo};

struct Foo;

impl FooTrait for Foo {
    fn foo() {
        // The main offender: using extern crate inside impl of a trait exposed by that same crate.
        extern crate mylib as _mylib;
    }
}

// An innocent bystander trait impl. It is not necessary to trigger the stack overflow, but having the same 'extern crate' declaration in it *prevents* it.
impl Clone for Foo {
    fn clone(&self) -> Self {
        // Uncomment the following, and it works again.
        // NOTE: This alone, without the 'extern crate' in the other impl, does not trigger the stack overflow.

        // extern crate mylib as _mylib;
        Foo
    }
}

// Either remove the pub, or change the return type, and it works again.
// Note: either having this as pub, or having a #[no_mangle] attribute triggers the stack overflow
// Note: changing `SysFoo` (reexported) to mylib_sys::SysFoo prevents the stack overflow
pub fn exposed_sys_foo() -> SysFoo {
    // NOTE: removing/changing this value to another type here causes the correct type error and does not trigger the stack overflow (yet?)
    SysFoo
}
