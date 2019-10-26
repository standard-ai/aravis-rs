// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcInteger;
use GcNode;
use GcSelector;
use aravis_sys;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GcIntegerNode(Object<aravis_sys::ArvGcIntegerNode, aravis_sys::ArvGcIntegerNodeClass, GcIntegerNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode, @implements GcInteger, GcSelector;

    match fn {
        get_type => || aravis_sys::arv_gc_integer_node_get_type(),
    }
}

impl GcIntegerNode {
    pub fn new() -> GcIntegerNode {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_integer_node_new()).unsafe_cast()
        }
    }
}

impl Default for GcIntegerNode {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_INTEGER_NODE: Option<&GcIntegerNode> = None;

impl fmt::Display for GcIntegerNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcIntegerNode")
    }
}