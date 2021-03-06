// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomDocument;
use DomNodeList;
use DomNodeType;
use aravis_sys;
use gio;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct DomNode(Object<aravis_sys::ArvDomNode, aravis_sys::ArvDomNodeClass, DomNodeClass>);

    match fn {
        get_type => || aravis_sys::arv_dom_node_get_type(),
    }
}

pub const NONE_DOM_NODE: Option<&DomNode> = None;

pub trait DomNodeExt: 'static {
    fn append_child<P: IsA<DomNode>>(&self, new_child: &P) -> Option<DomNode>;

    fn changed(&self);

    fn get_child_nodes(&self) -> Option<DomNodeList>;

    fn get_first_child(&self) -> Option<DomNode>;

    fn get_last_child(&self) -> Option<DomNode>;

    fn get_next_sibling(&self) -> Option<DomNode>;

    fn get_node_name(&self) -> Option<GString>;

    fn get_node_type(&self) -> DomNodeType;

    fn get_node_value(&self) -> Option<GString>;

    fn get_owner_document(&self) -> Option<DomDocument>;

    fn get_parent_node(&self) -> Option<DomNode>;

    fn get_previous_sibling(&self) -> Option<DomNode>;

    fn has_child_nodes(&self) -> bool;

    fn insert_before<P: IsA<DomNode>, Q: IsA<DomNode>>(&self, new_child: &P, ref_child: &Q) -> Option<DomNode>;

    fn remove_child<P: IsA<DomNode>>(&self, old_child: &P) -> Option<DomNode>;

    fn replace_child<P: IsA<DomNode>, Q: IsA<DomNode>>(&self, new_child: &P, old_child: &Q) -> Option<DomNode>;

    fn set_node_value(&self, new_value: &str);

    fn write_to_stream<P: IsA<gio::OutputStream>>(&self, stream: &P) -> Result<(), glib::Error>;
}

impl<O: IsA<DomNode>> DomNodeExt for O {
    fn append_child<P: IsA<DomNode>>(&self, new_child: &P) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_append_child(self.as_ref().to_glib_none().0, new_child.as_ref().to_glib_full()))
        }
    }

    fn changed(&self) {
        unsafe {
            aravis_sys::arv_dom_node_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn get_child_nodes(&self) -> Option<DomNodeList> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_child_nodes(self.as_ref().to_glib_none().0))
        }
    }

    fn get_first_child(&self) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_first_child(self.as_ref().to_glib_none().0))
        }
    }

    fn get_last_child(&self) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_last_child(self.as_ref().to_glib_none().0))
        }
    }

    fn get_next_sibling(&self) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_next_sibling(self.as_ref().to_glib_none().0))
        }
    }

    fn get_node_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_node_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_node_type(&self) -> DomNodeType {
        unsafe {
            from_glib(aravis_sys::arv_dom_node_get_node_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_node_value(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_node_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_owner_document(&self) -> Option<DomDocument> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_owner_document(self.as_ref().to_glib_none().0))
        }
    }

    fn get_parent_node(&self) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_parent_node(self.as_ref().to_glib_none().0))
        }
    }

    fn get_previous_sibling(&self) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_get_previous_sibling(self.as_ref().to_glib_none().0))
        }
    }

    fn has_child_nodes(&self) -> bool {
        unsafe {
            from_glib(aravis_sys::arv_dom_node_has_child_nodes(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_before<P: IsA<DomNode>, Q: IsA<DomNode>>(&self, new_child: &P, ref_child: &Q) -> Option<DomNode> {
        unsafe {
            from_glib_none(aravis_sys::arv_dom_node_insert_before(self.as_ref().to_glib_none().0, new_child.as_ref().to_glib_full(), ref_child.as_ref().to_glib_none().0))
        }
    }

    fn remove_child<P: IsA<DomNode>>(&self, old_child: &P) -> Option<DomNode> {
        unsafe {
            from_glib_full(aravis_sys::arv_dom_node_remove_child(self.as_ref().to_glib_none().0, old_child.as_ref().to_glib_none().0))
        }
    }

    fn replace_child<P: IsA<DomNode>, Q: IsA<DomNode>>(&self, new_child: &P, old_child: &Q) -> Option<DomNode> {
        unsafe {
            from_glib_full(aravis_sys::arv_dom_node_replace_child(self.as_ref().to_glib_none().0, new_child.as_ref().to_glib_full(), old_child.as_ref().to_glib_none().0))
        }
    }

    fn set_node_value(&self, new_value: &str) {
        unsafe {
            aravis_sys::arv_dom_node_set_node_value(self.as_ref().to_glib_none().0, new_value.to_glib_none().0);
        }
    }

    fn write_to_stream<P: IsA<gio::OutputStream>>(&self, stream: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_dom_node_write_to_stream(self.as_ref().to_glib_none().0, stream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for DomNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DomNode")
    }
}
