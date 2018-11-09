// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
use RelationType;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Relation(Object<ffi::AtkRelation, ffi::AtkRelationClass>);

    match fn {
        get_type => || ffi::atk_relation_get_type(),
    }
}

impl Relation {
    pub fn new(targets: &[Object], relationship: RelationType) -> Relation {
        assert_initialized_main_thread!();
        let n_targets = targets.len() as i32;
        unsafe {
            from_glib_full(ffi::atk_relation_new(targets.to_glib_none().0, n_targets, relationship.to_glib()))
        }
    }
}

pub trait RelationExt {
    fn add_target<P: IsA<Object>>(&self, target: &P);

    fn get_relation_type(&self) -> RelationType;

    //fn get_target(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 };

    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool;

    fn set_property_relation_type(&self, relation_type: RelationType);

    fn set_property_target(&self, target: Option<&glib::ValueArray>);

    fn connect_property_relation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Relation> + IsA<glib::object::Object>> RelationExt for O {
    fn add_target<P: IsA<Object>>(&self, target: &P) {
        unsafe {
            ffi::atk_relation_add_target(self.to_glib_none().0, target.to_glib_none().0);
        }
    }

    fn get_relation_type(&self) -> RelationType {
        unsafe {
            from_glib(ffi::atk_relation_get_relation_type(self.to_glib_none().0))
        }
    }

    //fn get_target(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 } {
    //    unsafe { TODO: call ffi::atk_relation_get_target() }
    //}

    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_relation_remove_target(self.to_glib_none().0, target.to_glib_none().0))
        }
    }

    fn set_property_relation_type(&self, relation_type: RelationType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "relation-type".to_glib_none().0, Value::from(&relation_type).to_glib_none().0);
        }
    }

    fn set_property_target(&self, target: Option<&glib::ValueArray>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "target".to_glib_none().0, Value::from(target).to_glib_none().0);
        }
    }

    fn connect_property_relation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::relation-type",
                transmute(notify_relation_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_relation_type_trampoline<P>(this: *mut ffi::AtkRelation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Relation> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Relation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::AtkRelation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Relation> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Relation::from_glib_borrow(this).downcast_unchecked())
}
