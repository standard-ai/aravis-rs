#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

extern crate aravis_sys;
extern crate gio;
extern crate gio_sys;
#[macro_use]
pub extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate image;

macro_rules! assert_initialized_main_thread {
    () => {};
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use auto::*;

mod manual;
pub use manual::*;

pub mod prelude {
    pub use auto::traits::*;
    pub use manual::traits::*;
}
