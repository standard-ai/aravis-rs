// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Interface;
use aravis_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct FakeInterface(Object<aravis_sys::ArvFakeInterface, aravis_sys::ArvFakeInterfaceClass, FakeInterfaceClass>) @extends Interface;

    match fn {
        get_type => || aravis_sys::arv_fake_interface_get_type(),
    }
}

impl FakeInterface {
    pub fn get_instance() -> Option<Interface> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(aravis_sys::arv_fake_interface_get_instance())
        }
    }
}

pub const NONE_FAKE_INTERFACE: Option<&FakeInterface> = None;

impl fmt::Display for FakeInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FakeInterface")
    }
}
