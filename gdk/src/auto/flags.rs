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

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
bitflags! {
    pub struct AnchorHints: u32 {
        const FLIP_X = 1;
        const FLIP_Y = 2;
        const SLIDE_X = 4;
        const SLIDE_Y = 8;
        const RESIZE_X = 16;
        const RESIZE_Y = 32;
        const FLIP = 3;
        const SLIDE = 12;
        const RESIZE = 48;
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
#[doc(hidden)]
impl ToGlib for AnchorHints {
    type GlibType = ffi::GdkAnchorHints;

    fn to_glib(&self) -> ffi::GdkAnchorHints {
        self.bits()
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
#[doc(hidden)]
impl FromGlib<ffi::GdkAnchorHints> for AnchorHints {
    fn from_glib(value: ffi::GdkAnchorHints) -> AnchorHints {
        skip_assert_initialized!();
        AnchorHints::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl StaticType for AnchorHints {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_anchor_hints_get_type()) }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl<'a> FromValueOptional<'a> for AnchorHints {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl<'a> FromValue<'a> for AnchorHints {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl SetValue for AnchorHints {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
bitflags! {
    pub struct AxisFlags: u32 {
        const X = 2;
        const Y = 4;
        const PRESSURE = 8;
        const XTILT = 16;
        const YTILT = 32;
        const WHEEL = 64;
        const DISTANCE = 128;
        const ROTATION = 256;
        const SLIDER = 512;
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
#[doc(hidden)]
impl ToGlib for AxisFlags {
    type GlibType = ffi::GdkAxisFlags;

    fn to_glib(&self) -> ffi::GdkAxisFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
#[doc(hidden)]
impl FromGlib<ffi::GdkAxisFlags> for AxisFlags {
    fn from_glib(value: ffi::GdkAxisFlags) -> AxisFlags {
        skip_assert_initialized!();
        AxisFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl StaticType for AxisFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_axis_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl<'a> FromValueOptional<'a> for AxisFlags {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl<'a> FromValue<'a> for AxisFlags {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
impl SetValue for AxisFlags {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DragAction: u32 {
        const DEFAULT = 1;
        const COPY = 2;
        const MOVE = 4;
        const LINK = 8;
        const PRIVATE = 16;
        const ASK = 32;
    }
}

#[doc(hidden)]
impl ToGlib for DragAction {
    type GlibType = ffi::GdkDragAction;

    fn to_glib(&self) -> ffi::GdkDragAction {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkDragAction> for DragAction {
    fn from_glib(value: ffi::GdkDragAction) -> DragAction {
        skip_assert_initialized!();
        DragAction::from_bits_truncate(value)
    }
}

impl StaticType for DragAction {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_drag_action_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DragAction {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DragAction {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DragAction {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct EventMask: u32 {
        const EXPOSURE_MASK = 2;
        const POINTER_MOTION_MASK = 4;
        const POINTER_MOTION_HINT_MASK = 8;
        const BUTTON_MOTION_MASK = 16;
        const BUTTON1_MOTION_MASK = 32;
        const BUTTON2_MOTION_MASK = 64;
        const BUTTON3_MOTION_MASK = 128;
        const BUTTON_PRESS_MASK = 256;
        const BUTTON_RELEASE_MASK = 512;
        const KEY_PRESS_MASK = 1024;
        const KEY_RELEASE_MASK = 2048;
        const ENTER_NOTIFY_MASK = 4096;
        const LEAVE_NOTIFY_MASK = 8192;
        const FOCUS_CHANGE_MASK = 16384;
        const STRUCTURE_MASK = 32768;
        const PROPERTY_CHANGE_MASK = 65536;
        const VISIBILITY_NOTIFY_MASK = 131072;
        const PROXIMITY_IN_MASK = 262144;
        const PROXIMITY_OUT_MASK = 524288;
        const SUBSTRUCTURE_MASK = 1048576;
        const SCROLL_MASK = 2097152;
        const TOUCH_MASK = 4194304;
        const SMOOTH_SCROLL_MASK = 8388608;
        const TOUCHPAD_GESTURE_MASK = 16777216;
        const TABLET_PAD_MASK = 33554432;
        const ALL_EVENTS_MASK = 67108862;
    }
}

#[doc(hidden)]
impl ToGlib for EventMask {
    type GlibType = ffi::GdkEventMask;

    fn to_glib(&self) -> ffi::GdkEventMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkEventMask> for EventMask {
    fn from_glib(value: ffi::GdkEventMask) -> EventMask {
        skip_assert_initialized!();
        EventMask::from_bits_truncate(value)
    }
}

impl StaticType for EventMask {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_event_mask_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EventMask {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EventMask {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for EventMask {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct FrameClockPhase: u32 {
        const NONE = 0;
        const FLUSH_EVENTS = 1;
        const BEFORE_PAINT = 2;
        const UPDATE = 4;
        const LAYOUT = 8;
        const PAINT = 16;
        const RESUME_EVENTS = 32;
        const AFTER_PAINT = 64;
    }
}

#[doc(hidden)]
impl ToGlib for FrameClockPhase {
    type GlibType = ffi::GdkFrameClockPhase;

    fn to_glib(&self) -> ffi::GdkFrameClockPhase {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkFrameClockPhase> for FrameClockPhase {
    fn from_glib(value: ffi::GdkFrameClockPhase) -> FrameClockPhase {
        skip_assert_initialized!();
        FrameClockPhase::from_bits_truncate(value)
    }
}

impl StaticType for FrameClockPhase {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_frame_clock_phase_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FrameClockPhase {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FrameClockPhase {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FrameClockPhase {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ModifierType: u32 {
        const SHIFT_MASK = 1;
        const LOCK_MASK = 2;
        const CONTROL_MASK = 4;
        const MOD1_MASK = 8;
        const MOD2_MASK = 16;
        const MOD3_MASK = 32;
        const MOD4_MASK = 64;
        const MOD5_MASK = 128;
        const BUTTON1_MASK = 256;
        const BUTTON2_MASK = 512;
        const BUTTON3_MASK = 1024;
        const BUTTON4_MASK = 2048;
        const BUTTON5_MASK = 4096;
        const MODIFIER_RESERVED_13_MASK = 8192;
        const MODIFIER_RESERVED_14_MASK = 16384;
        const MODIFIER_RESERVED_15_MASK = 32768;
        const MODIFIER_RESERVED_16_MASK = 65536;
        const MODIFIER_RESERVED_17_MASK = 131072;
        const MODIFIER_RESERVED_18_MASK = 262144;
        const MODIFIER_RESERVED_19_MASK = 524288;
        const MODIFIER_RESERVED_20_MASK = 1048576;
        const MODIFIER_RESERVED_21_MASK = 2097152;
        const MODIFIER_RESERVED_22_MASK = 4194304;
        const MODIFIER_RESERVED_23_MASK = 8388608;
        const MODIFIER_RESERVED_24_MASK = 16777216;
        const MODIFIER_RESERVED_25_MASK = 33554432;
        const SUPER_MASK = 67108864;
        const HYPER_MASK = 134217728;
        const META_MASK = 268435456;
        const MODIFIER_RESERVED_29_MASK = 536870912;
        const RELEASE_MASK = 1073741824;
        const MODIFIER_MASK = 1543512063;
    }
}

#[doc(hidden)]
impl ToGlib for ModifierType {
    type GlibType = ffi::GdkModifierType;

    fn to_glib(&self) -> ffi::GdkModifierType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkModifierType> for ModifierType {
    fn from_glib(value: ffi::GdkModifierType) -> ModifierType {
        skip_assert_initialized!();
        ModifierType::from_bits_truncate(value)
    }
}

impl StaticType for ModifierType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_modifier_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ModifierType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ModifierType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ModifierType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
bitflags! {
    pub struct SeatCapabilities: u32 {
        const NONE = 0;
        const POINTER = 1;
        const TOUCH = 2;
        const TABLET_STYLUS = 4;
        const KEYBOARD = 8;
        const ALL_POINTING = 7;
        const ALL = 15;
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
#[doc(hidden)]
impl ToGlib for SeatCapabilities {
    type GlibType = ffi::GdkSeatCapabilities;

    fn to_glib(&self) -> ffi::GdkSeatCapabilities {
        self.bits()
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GdkSeatCapabilities> for SeatCapabilities {
    fn from_glib(value: ffi::GdkSeatCapabilities) -> SeatCapabilities {
        skip_assert_initialized!();
        SeatCapabilities::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl StaticType for SeatCapabilities {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_seat_capabilities_get_type()) }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl<'a> FromValueOptional<'a> for SeatCapabilities {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl<'a> FromValue<'a> for SeatCapabilities {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl SetValue for SeatCapabilities {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct WMDecoration: u32 {
        const ALL = 1;
        const BORDER = 2;
        const RESIZEH = 4;
        const TITLE = 8;
        const MENU = 16;
        const MINIMIZE = 32;
        const MAXIMIZE = 64;
    }
}

#[doc(hidden)]
impl ToGlib for WMDecoration {
    type GlibType = ffi::GdkWMDecoration;

    fn to_glib(&self) -> ffi::GdkWMDecoration {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWMDecoration> for WMDecoration {
    fn from_glib(value: ffi::GdkWMDecoration) -> WMDecoration {
        skip_assert_initialized!();
        WMDecoration::from_bits_truncate(value)
    }
}

impl StaticType for WMDecoration {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_wm_decoration_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WMDecoration {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WMDecoration {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for WMDecoration {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct WMFunction: u32 {
        const ALL = 1;
        const RESIZE = 2;
        const MOVE = 4;
        const MINIMIZE = 8;
        const MAXIMIZE = 16;
        const CLOSE = 32;
    }
}

#[doc(hidden)]
impl ToGlib for WMFunction {
    type GlibType = ffi::GdkWMFunction;

    fn to_glib(&self) -> ffi::GdkWMFunction {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWMFunction> for WMFunction {
    fn from_glib(value: ffi::GdkWMFunction) -> WMFunction {
        skip_assert_initialized!();
        WMFunction::from_bits_truncate(value)
    }
}

impl StaticType for WMFunction {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_wm_function_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WMFunction {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WMFunction {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for WMFunction {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct WindowHints: u32 {
        const POS = 1;
        const MIN_SIZE = 2;
        const MAX_SIZE = 4;
        const BASE_SIZE = 8;
        const ASPECT = 16;
        const RESIZE_INC = 32;
        const WIN_GRAVITY = 64;
        const USER_POS = 128;
        const USER_SIZE = 256;
    }
}

#[doc(hidden)]
impl ToGlib for WindowHints {
    type GlibType = ffi::GdkWindowHints;

    fn to_glib(&self) -> ffi::GdkWindowHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWindowHints> for WindowHints {
    fn from_glib(value: ffi::GdkWindowHints) -> WindowHints {
        skip_assert_initialized!();
        WindowHints::from_bits_truncate(value)
    }
}

impl StaticType for WindowHints {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_window_hints_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WindowHints {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WindowHints {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for WindowHints {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct WindowState: u32 {
        const WITHDRAWN = 1;
        const ICONIFIED = 2;
        const MAXIMIZED = 4;
        const STICKY = 8;
        const FULLSCREEN = 16;
        const ABOVE = 32;
        const BELOW = 64;
        const FOCUSED = 128;
        const TILED = 256;
        const TOP_TILED = 512;
        const TOP_RESIZABLE = 1024;
        const RIGHT_TILED = 2048;
        const RIGHT_RESIZABLE = 4096;
        const BOTTOM_TILED = 8192;
        const BOTTOM_RESIZABLE = 16384;
        const LEFT_TILED = 32768;
        const LEFT_RESIZABLE = 65536;
    }
}

#[doc(hidden)]
impl ToGlib for WindowState {
    type GlibType = ffi::GdkWindowState;

    fn to_glib(&self) -> ffi::GdkWindowState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWindowState> for WindowState {
    fn from_glib(value: ffi::GdkWindowState) -> WindowState {
        skip_assert_initialized!();
        WindowState::from_bits_truncate(value)
    }
}

impl StaticType for WindowState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_window_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WindowState {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WindowState {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for WindowState {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
