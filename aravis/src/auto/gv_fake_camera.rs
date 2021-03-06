// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use FakeCamera;
use aravis_sys;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct GvFakeCamera(Object<aravis_sys::ArvGvFakeCamera, aravis_sys::ArvGvFakeCameraClass, GvFakeCameraClass>);

    match fn {
        get_type => || aravis_sys::arv_gv_fake_camera_get_type(),
    }
}

impl GvFakeCamera {
    pub fn new(interface_name: Option<&str>, serial_number: Option<&str>) -> GvFakeCamera {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_gv_fake_camera_new(interface_name.to_glib_none().0, serial_number.to_glib_none().0))
        }
    }

    pub fn new_full(interface_name: Option<&str>, serial_number: Option<&str>, genicam_filename: Option<&str>) -> GvFakeCamera {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_gv_fake_camera_new_full(interface_name.to_glib_none().0, serial_number.to_glib_none().0, genicam_filename.to_glib_none().0))
        }
    }
}

pub const NONE_GV_FAKE_CAMERA: Option<&GvFakeCamera> = None;

pub trait GvFakeCameraExt: 'static {
    fn get_fake_camera(&self) -> Option<FakeCamera>;

    fn is_running(&self) -> bool;

    fn set_property_gvsp_lost_ratio(&self, gvsp_lost_ratio: f64);

    fn connect_property_gvsp_lost_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GvFakeCamera>> GvFakeCameraExt for O {
    fn get_fake_camera(&self) -> Option<FakeCamera> {
        unsafe {
            from_glib_none(aravis_sys::arv_gv_fake_camera_get_fake_camera(self.as_ref().to_glib_none().0))
        }
    }

    fn is_running(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_gv_fake_camera_is_running(self.as_ref().to_glib_none().0))
        }
    }

    fn set_property_gvsp_lost_ratio(&self, gvsp_lost_ratio: f64) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"gvsp-lost-ratio\0".as_ptr() as *const _, Value::from(&gvsp_lost_ratio).to_glib_none().0);
        }
    }

    fn connect_property_gvsp_lost_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gvsp_lost_ratio_trampoline<P, F: Fn(&P) + 'static>(this: *mut aravis_sys::ArvGvFakeCamera, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<GvFakeCamera>
        {
            let f: &F = &*(f as *const F);
            f(&GvFakeCamera::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gvsp-lost-ratio\0".as_ptr() as *const _,
                Some(transmute(notify_gvsp_lost_ratio_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for GvFakeCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GvFakeCamera")
    }
}
