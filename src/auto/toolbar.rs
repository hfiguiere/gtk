// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use Container;
use IconSize;
use Orientable;
use ReliefStyle;
use ToolItem;
use ToolShell;
use ToolbarStyle;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Toolbar(Object<ffi::GtkToolbar>): Container, Widget, Orientable, ToolShell;

    match fn {
        get_type => || ffi::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toolbar_new()).downcast_unchecked()
        }
    }

    pub fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(self.to_glib_none().0, x, y)
        }
    }

    pub fn get_icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_icon_size(self.to_glib_none().0))
        }
    }

    pub fn get_item_index<T: IsA<ToolItem>>(&self, item: &T) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(self.to_glib_none().0, item.to_glib_none().0)
        }
    }

    pub fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(self.to_glib_none().0)
        }
    }

    pub fn get_nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_toolbar_get_nth_item(self.to_glib_none().0, n))
        }
    }

    pub fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_relief_style(self.to_glib_none().0))
        }
    }

    pub fn get_show_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_show_arrow(self.to_glib_none().0))
        }
    }

    pub fn get_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_style(self.to_glib_none().0))
        }
    }

    pub fn insert<T: IsA<ToolItem>>(&self, item: &T, pos: i32) {
        unsafe {
            ffi::gtk_toolbar_insert(self.to_glib_none().0, item.to_glib_none().0, pos);
        }
    }

    pub fn set_drop_highlight_item<T: IsA<ToolItem>>(&self, tool_item: Option<&T>, index_: i32) {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(self.to_glib_none().0, tool_item.to_glib_none().0, index_);
        }
    }

    pub fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(self.to_glib_none().0, icon_size.to_glib());
        }
    }

    pub fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_toolbar_set_show_arrow(self.to_glib_none().0, show_arrow.to_glib());
        }
    }

    pub fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_toolbar_set_style(self.to_glib_none().0, style.to_glib());
        }
    }

    pub fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(self.to_glib_none().0);
        }
    }

    pub fn unset_style(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_style(self.to_glib_none().0);
        }
    }
}
