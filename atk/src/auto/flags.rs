// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    pub struct HyperlinkStateFlags: u32 {
        const INLINE = 1;
    }
}

#[doc(hidden)]
impl ToGlib for HyperlinkStateFlags {
    type GlibType = ffi::AtkHyperlinkStateFlags;

    fn to_glib(&self) -> ffi::AtkHyperlinkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::AtkHyperlinkStateFlags> for HyperlinkStateFlags {
    fn from_glib(value: ffi::AtkHyperlinkStateFlags) -> HyperlinkStateFlags {
        skip_assert_initialized!();
        HyperlinkStateFlags::from_bits_truncate(value)
    }
}

impl StaticType for HyperlinkStateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::atk_hyperlink_state_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for HyperlinkStateFlags {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for HyperlinkStateFlags {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for HyperlinkStateFlags {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
