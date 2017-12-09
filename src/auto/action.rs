// This file was generated by gir (d8a605d) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v2_38", feature = "dox"))]
use Error;
use ffi;
use glib;
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
    pub struct Action(Object<ffi::GAction, ffi::GActionInterface>);

    match fn {
        get_type => || ffi::g_action_get_type(),
    }
}

impl Action {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn name_is_valid(action_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_action_name_is_valid(action_name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn parse_detailed_name(detailed_name: &str) -> Result<(String, glib::Variant), Error> {
        unsafe {
            let mut action_name = ptr::null_mut();
            let mut target_value = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_action_parse_detailed_name(detailed_name.to_glib_none().0, &mut action_name, &mut target_value, &mut error);
            if error.is_null() { Ok((from_glib_full(action_name), from_glib_full(target_value))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn print_detailed_name<'a, P: Into<Option<&'a glib::Variant>>>(action_name: &str, target_value: P) -> Option<String> {
        let target_value = target_value.into();
        let target_value = target_value.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_action_print_detailed_name(action_name.to_glib_none().0, target_value.0))
        }
    }
}

pub trait ActionExt {
    fn activate<'a, P: Into<Option<&'a glib::Variant>>>(&self, parameter: P);

    fn change_state(&self, value: &glib::Variant);

    fn get_enabled(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_parameter_type(&self) -> Option<glib::VariantType>;

    fn get_state(&self) -> Option<glib::Variant>;

    fn get_state_hint(&self) -> Option<glib::Variant>;

    fn get_state_type(&self) -> Option<glib::VariantType>;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Action> + IsA<glib::object::Object>> ActionExt for O {
    fn activate<'a, P: Into<Option<&'a glib::Variant>>>(&self, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::g_action_activate(self.to_glib_none().0, parameter.0);
        }
    }

    fn change_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_action_change_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_action_get_enabled(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_action_get_name(self.to_glib_none().0))
        }
    }

    fn get_parameter_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_get_parameter_type(self.to_glib_none().0))
        }
    }

    fn get_state(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_get_state(self.to_glib_none().0))
        }
    }

    fn get_state_hint(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_get_state_hint(self.to_glib_none().0))
        }
    }

    fn get_state_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_get_state_type(self.to_glib_none().0))
        }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enabled",
                transmute(notify_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parameter-type",
                transmute(notify_parameter_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::state",
                transmute(notify_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::state-type",
                transmute(notify_state_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_enabled_trampoline<P>(this: *mut ffi::GAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Action> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Action::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Action> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Action::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parameter_type_trampoline<P>(this: *mut ffi::GAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Action> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Action::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_state_trampoline<P>(this: *mut ffi::GAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Action> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Action::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_state_type_trampoline<P>(this: *mut ffi::GAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Action> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Action::from_glib_borrow(this).downcast_unchecked())
}
