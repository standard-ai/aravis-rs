// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Device;
use aravis_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Interface(Object<aravis_sys::ArvInterface, aravis_sys::ArvInterfaceClass, InterfaceClass>);

    match fn {
        get_type => || aravis_sys::arv_interface_get_type(),
    }
}

pub const NONE_INTERFACE: Option<&Interface> = None;

pub trait InterfaceExt: 'static {
    fn get_device_address(&self, index: u32) -> Option<GString>;

    fn get_device_id(&self, index: u32) -> Option<GString>;

    fn get_device_model(&self, index: u32) -> Option<GString>;

    fn get_device_physical_id(&self, index: u32) -> Option<GString>;

    fn get_device_protocol(&self, index: u32) -> Option<GString>;

    fn get_device_serial_nbr(&self, index: u32) -> Option<GString>;

    fn get_device_vendor(&self, index: u32) -> Option<GString>;

    fn get_n_devices(&self) -> u32;

    fn open_device(&self, device_id: Option<&str>) -> Option<Device>;

    fn update_device_list(&self);
}

impl<O: IsA<Interface>> InterfaceExt for O {
    fn get_device_address(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_address(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_id(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_id(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_model(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_model(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_physical_id(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_physical_id(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_protocol(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_protocol(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_serial_nbr(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_serial_nbr(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_device_vendor(&self, index: u32) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_interface_get_device_vendor(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_n_devices(&self) -> u32 {
        unsafe {
            aravis_sys::arv_interface_get_n_devices(self.as_ref().to_glib_none().0)
        }
    }

    fn open_device(&self, device_id: Option<&str>) -> Option<Device> {
        unsafe {
            from_glib_full(aravis_sys::arv_interface_open_device(self.as_ref().to_glib_none().0, device_id.to_glib_none().0))
        }
    }

    fn update_device_list(&self) {
        unsafe {
            aravis_sys::arv_interface_update_device_list(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interface")
    }
}