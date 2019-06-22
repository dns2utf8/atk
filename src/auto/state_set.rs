// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use StateType;

glib_wrapper! {
    pub struct StateSet(Object<atk_sys::AtkStateSet, atk_sys::AtkStateSetClass, StateSetClass>);

    match fn {
        get_type => || atk_sys::atk_state_set_get_type(),
    }
}

impl StateSet {
    pub fn new() -> StateSet {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(atk_sys::atk_state_set_new()) }
    }
}

impl Default for StateSet {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STATE_SET: Option<&StateSet> = None;

pub trait StateSetExt: 'static {
    fn add_state(&self, type_: StateType) -> bool;

    //fn add_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 });

    fn and_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet>;

    fn clear_states(&self);

    fn contains_state(&self, type_: StateType) -> bool;

    //fn contains_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 }) -> bool;

    fn is_empty(&self) -> bool;

    fn or_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet>;

    fn remove_state(&self, type_: StateType) -> bool;

    fn xor_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet>;
}

impl<O: IsA<StateSet>> StateSetExt for O {
    fn add_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(atk_sys::atk_state_set_add_state(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    //fn add_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 }) {
    //    unsafe { TODO: call atk_sys:atk_state_set_add_states() }
    //}

    fn and_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet> {
        unsafe {
            from_glib_full(atk_sys::atk_state_set_and_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }

    fn clear_states(&self) {
        unsafe {
            atk_sys::atk_state_set_clear_states(self.as_ref().to_glib_none().0);
        }
    }

    fn contains_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(atk_sys::atk_state_set_contains_state(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    //fn contains_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 }) -> bool {
    //    unsafe { TODO: call atk_sys:atk_state_set_contains_states() }
    //}

    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(atk_sys::atk_state_set_is_empty(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn or_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet> {
        unsafe {
            from_glib_full(atk_sys::atk_state_set_or_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(atk_sys::atk_state_set_remove_state(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    fn xor_sets<P: IsA<StateSet>>(&self, compare_set: &P) -> Option<StateSet> {
        unsafe {
            from_glib_full(atk_sys::atk_state_set_xor_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateSet")
    }
}
