// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;
use aravis_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GcBoolean(Object<aravis_sys::ArvGcBoolean, aravis_sys::ArvGcBooleanClass, GcBooleanClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

    match fn {
        get_type => || aravis_sys::arv_gc_boolean_get_type(),
    }
}

impl GcBoolean {
    pub fn new() -> GcBoolean {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_boolean_new()).unsafe_cast()
        }
    }
}

impl Default for GcBoolean {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_BOOLEAN: Option<&GcBoolean> = None;

pub trait GcBooleanExt: 'static {
    fn get_value(&self) -> Result<bool, glib::Error>;

    fn set_value(&self, v_boolean: bool) -> Result<(), glib::Error>;
}

impl<O: IsA<GcBoolean>> GcBooleanExt for O {
    fn get_value(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_boolean_get_value_gi(self.as_ref().to_glib_none().0, value.as_mut_ptr(), &mut error);
            let value = value.assume_init();
            if error.is_null() { Ok(from_glib(value)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_value(&self, v_boolean: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_boolean_set_value(self.as_ref().to_glib_none().0, v_boolean.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for GcBoolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcBoolean")
    }
}
