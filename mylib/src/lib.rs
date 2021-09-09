/// An innocent trait.
pub trait FooTrait {
    fn foo();
}

// A slightly less innocent re-export.
pub use mylib_sys::SysFoo;
