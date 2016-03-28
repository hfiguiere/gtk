// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}
