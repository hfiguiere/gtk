use glib::subclass::prelude::*;
use glib::ObjectClass;

use CellRendererClass;

pub trait CellRendererImpl: ObjectImpl + 'static {}

unsafe impl<T: ObjectSubclass + CellRendererImpl> IsSubclassable<T> for CellRendererClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
