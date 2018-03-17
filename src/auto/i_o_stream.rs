// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use InputStream;
use OutputStream;
use ffi;
use glib;
use glib::StaticType;
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
    pub struct IOStream(Object<ffi::GIOStream, ffi::GIOStreamClass>);

    match fn {
        get_type => || ffi::g_io_stream_get_type(),
    }
}

impl IOStream {}

pub trait IOStreamExt {
    fn clear_pending(&self);

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, callback: Q);

    fn get_input_stream(&self) -> Option<InputStream>;

    fn get_output_stream(&self) -> Option<OutputStream>;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn set_pending(&self) -> Result<(), Error>;

    fn get_property_closed(&self) -> bool;

    fn connect_property_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IOStream> + IsA<glib::object::Object>> IOStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_io_stream_clear_pending(self.to_glib_none().0);
        }
    }

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn close_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_io_stream_close_async(self.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_input_stream(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_input_stream(self.to_glib_none().0))
        }
    }

    fn get_output_stream(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_output_stream(self.to_glib_none().0))
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_io_stream_has_pending(self.to_glib_none().0))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_io_stream_is_closed(self.to_glib_none().0))
        }
    }

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_set_pending(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_closed(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "closed".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::closed",
                transmute(notify_closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_closed_trampoline<P>(this: *mut ffi::GIOStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IOStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IOStream::from_glib_borrow(this).downcast_unchecked())
}