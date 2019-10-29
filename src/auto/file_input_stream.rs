// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "futures", feature = "dox"))]
use futures::future;
use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;
use Cancellable;
use FileInfo;
use InputStream;
use Seekable;

glib_wrapper! {
    pub struct FileInputStream(Object<gio_sys::GFileInputStream, gio_sys::GFileInputStreamClass, FileInputStreamClass>) @extends InputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_file_input_stream_get_type(),
    }
}

pub const NONE_FILE_INPUT_STREAM: Option<&FileInputStream> = None;

pub trait FileInputStreamExt: 'static {
    fn query_info<P: IsA<Cancellable>>(
        &self,
        attributes: &str,
        cancellable: Option<&P>,
    ) -> Result<FileInfo, glib::Error>;

    fn query_info_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn query_info_async_future(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<FileInfo, glib::Error>> + std::marker::Unpin>;
}

impl<O: IsA<FileInputStream>> FileInputStreamExt for O {
    fn query_info<P: IsA<Cancellable>>(
        &self,
        attributes: &str,
        cancellable: Option<&P>,
    ) -> Result<FileInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_input_stream_query_info(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn query_info_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn query_info_async_trampoline<
            Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_input_stream_query_info_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_file_input_stream_query_info_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn query_info_async_future(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<FileInfo, glib::Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        let attributes = String::from(attributes);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.query_info_async(&attributes, io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }
}

impl fmt::Display for FileInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileInputStream")
    }
}
