// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Permission;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct SimplePermission(Object<ffi::GSimplePermission>) @extends Permission;

    match fn {
        get_type => || ffi::g_simple_permission_get_type(),
    }
}

impl SimplePermission {
    pub fn new(allowed: bool) -> SimplePermission {
        unsafe {
            Permission::from_glib_full(ffi::g_simple_permission_new(allowed.to_glib()))
                .unsafe_cast()
        }
    }
}

impl fmt::Display for SimplePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SimplePermission")
    }
}
