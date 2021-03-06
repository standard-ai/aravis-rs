// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;
use aravis_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GcRegisterDescriptionNode(Object<aravis_sys::ArvGcRegisterDescriptionNode, aravis_sys::ArvGcRegisterDescriptionNodeClass, GcRegisterDescriptionNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

    match fn {
        get_type => || aravis_sys::arv_gc_register_description_node_get_type(),
    }
}

impl GcRegisterDescriptionNode {
    pub fn new() -> GcRegisterDescriptionNode {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_register_description_node_new()).unsafe_cast()
        }
    }
}

impl Default for GcRegisterDescriptionNode {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_REGISTER_DESCRIPTION_NODE: Option<&GcRegisterDescriptionNode> = None;

pub trait GcRegisterDescriptionNodeExt: 'static {
    fn check_schema_version(&self, required_major: u32, required_minor: u32, required_subminor: u32) -> bool;

    fn compare_schema_version(&self, major: u32, minor: u32, subminor: u32) -> i32;
}

impl<O: IsA<GcRegisterDescriptionNode>> GcRegisterDescriptionNodeExt for O {
    fn check_schema_version(&self, required_major: u32, required_minor: u32, required_subminor: u32) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_gc_register_description_node_check_schema_version(self.as_ref().to_glib_none().0, required_major, required_minor, required_subminor))
        }
    }

    fn compare_schema_version(&self, major: u32, minor: u32, subminor: u32) -> i32 {
        unsafe {
            aravis_sys::arv_gc_register_description_node_compare_schema_version(self.as_ref().to_glib_none().0, major, minor, subminor)
        }
    }
}

impl fmt::Display for GcRegisterDescriptionNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcRegisterDescriptionNode")
    }
}
