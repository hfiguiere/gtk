// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use Widget;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextChildAnchor(Object<ffi::GtkTextChildAnchor>);

    match fn {
        get_type => || ffi::gtk_text_child_anchor_get_type(),
    }
}

impl TextChildAnchor {
    pub fn new() -> TextChildAnchor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_child_anchor_new())
        }
    }

    pub fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_child_anchor_get_deleted(self.to_glib_none().0))
        }
    }

    pub fn get_widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_child_anchor_get_widgets(self.to_glib_none().0))
        }
    }
}
