// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ObjectFactory(Object<ffi::AtkObjectFactory, ffi::AtkObjectFactoryClass>);

    match fn {
        get_type => || ffi::atk_object_factory_get_type(),
    }
}

pub trait ObjectFactoryExt: 'static {
    fn create_accessible<P: IsA<glib::Object>>(&self, obj: &P) -> Option<Object>;

    fn get_accessible_type(&self) -> glib::types::Type;

    fn invalidate(&self);
}

impl<O: IsA<ObjectFactory>> ObjectFactoryExt for O {
    fn create_accessible<P: IsA<glib::Object>>(&self, obj: &P) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_object_factory_create_accessible(self.to_glib_none().0, obj.to_glib_none().0))
        }
    }

    fn get_accessible_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::atk_object_factory_get_accessible_type(self.to_glib_none().0))
        }
    }

    fn invalidate(&self) {
        unsafe {
            ffi::atk_object_factory_invalidate(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for ObjectFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObjectFactory")
    }
}
