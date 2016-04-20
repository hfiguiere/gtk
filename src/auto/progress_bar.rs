// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ProgressBar(Object<ffi::GtkProgressBar>): Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_progress_bar_get_type(),
    }
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_progress_bar_new()).downcast_unchecked()
        }
    }

    //pub fn get_ellipsize(&self) -> /*Ignored*/pango::EllipsizeMode {
    //    unsafe { TODO: call ffi::gtk_progress_bar_get_ellipsize() }
    //}

    pub fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(self.to_glib_none().0)
        }
    }

    pub fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_inverted(self.to_glib_none().0))
        }
    }

    pub fn get_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_pulse_step(self.to_glib_none().0)
        }
    }

    pub fn get_show_text(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_show_text(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_progress_bar_get_text(self.to_glib_none().0))
        }
    }

    pub fn pulse(&self) {
        unsafe {
            ffi::gtk_progress_bar_pulse(self.to_glib_none().0);
        }
    }

    //pub fn set_ellipsize(&self, mode: /*Ignored*/pango::EllipsizeMode) {
    //    unsafe { TODO: call ffi::gtk_progress_bar_set_ellipsize() }
    //}

    pub fn set_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(self.to_glib_none().0, fraction);
        }
    }

    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    pub fn set_pulse_step(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(self.to_glib_none().0, fraction);
        }
    }

    pub fn set_show_text(&self, show_text: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_show_text(self.to_glib_none().0, show_text.to_glib());
        }
    }

    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_progress_bar_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }
}
