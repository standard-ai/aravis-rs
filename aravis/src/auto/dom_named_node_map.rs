// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomNode;
use aravis_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct DomNamedNodeMap(Object<aravis_sys::ArvDomNamedNodeMap, aravis_sys::ArvDomNamedNodeMapClass, DomNamedNodeMapClass>);

    match fn {
        get_type => || aravis_sys::arv_dom_named_node_map_get_type(),
    }
}

pub const NONE_DOM_NAMED_NODE_MAP: Option<&DomNamedNodeMap> = None;

pub trait DomNamedNodeMapExt: 'static {
    fn get_item(&self, index: u32) -> Option<DomNode>;

    fn get_length(&self) -> u32;

    fn get_named_item(&self, name: &str) -> Option<DomNode>;

    fn remove_named_item(&self, name: &str) -> Option<DomNode>;

    fn set_named_item<P: IsA<DomNode>>(&self, item: &P) -> Option<DomNode>;
}

impl<O: IsA<DomNamedNodeMap>> DomNamedNodeMapExt for O {
    fn get_item(&self, index: u32) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_named_node_map_get_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_length(&self) -> u32 {
        unsafe {
            aravis_sys::arv_dom_named_node_map_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn get_named_item(&self, name: &str) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_named_node_map_get_named_item(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove_named_item(&self, name: &str) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_named_node_map_remove_named_item(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn set_named_item<P: IsA<DomNode>>(&self, item: &P) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_named_node_map_set_named_item(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for DomNamedNodeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DomNamedNodeMap")
    }
}
