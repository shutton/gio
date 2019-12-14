// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use File;
use IOErrorEnum;
use IOStream;
use Icon;
use InputStream;
use Resource;
use ResourceLookupFlags;
use SettingsBackend;

//pub fn bus_get<P: IsA<Cancellable>, Q: FnOnce(Result</*Ignored*/DBusConnection, glib::Error>) + Send + 'static>(bus_type: /*Ignored*/BusType, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call gio_sys:g_bus_get() }
//}

//
//pub fn bus_get_future(bus_type: /*Ignored*/BusType) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/DBusConnection, glib::Error>> + 'static>> {

//Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
//    let cancellable = Cancellable::new();
//    bus_get(
//        bus_type,
//        Some(&cancellable),
//        move |res| {
//            send.resolve(res);
//        },
//    );

//    cancellable
//}))
//}

//pub fn bus_get_sync<P: IsA<Cancellable>>(bus_type: /*Ignored*/BusType, cancellable: Option<&P>) -> Result</*Ignored*/DBusConnection, glib::Error> {
//    unsafe { TODO: call gio_sys:g_bus_get_sync() }
//}

//pub fn bus_own_name(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameOwnerFlags, bus_acquired_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), name_acquired_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), name_lost_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_own_name() }
//}

//pub fn bus_own_name_on_connection(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameOwnerFlags, name_acquired_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), name_lost_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_own_name_on_connection() }
//}

//pub fn bus_own_name_on_connection_with_closures(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameOwnerFlags, name_acquired_closure: /*Ignored*/Option<&glib::Closure>, name_lost_closure: /*Ignored*/Option<&glib::Closure>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_own_name_on_connection_with_closures() }
//}

//pub fn bus_own_name_with_closures(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameOwnerFlags, bus_acquired_closure: /*Ignored*/Option<&glib::Closure>, name_acquired_closure: /*Ignored*/Option<&glib::Closure>, name_lost_closure: /*Ignored*/Option<&glib::Closure>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_own_name_with_closures() }
//}

pub fn bus_unown_name(owner_id: u32) {
    unsafe {
        gio_sys::g_bus_unown_name(owner_id);
    }
}

pub fn bus_unwatch_name(watcher_id: u32) {
    unsafe {
        gio_sys::g_bus_unwatch_name(watcher_id);
    }
}

//pub fn bus_watch_name(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str, &str), name_vanished_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_watch_name() }
//}

//pub fn bus_watch_name_on_connection(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str, &str), name_vanished_handler: /*Unimplemented*/Fn(/*Ignored*/DBusConnection, &str), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_watch_name_on_connection() }
//}

//pub fn bus_watch_name_on_connection_with_closures(connection: /*Ignored*/&DBusConnection, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_closure: /*Ignored*/Option<&glib::Closure>, name_vanished_closure: /*Ignored*/Option<&glib::Closure>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_watch_name_on_connection_with_closures() }
//}

//pub fn bus_watch_name_with_closures(bus_type: /*Ignored*/BusType, name: &str, flags: /*Ignored*/BusNameWatcherFlags, name_appeared_closure: /*Ignored*/Option<&glib::Closure>, name_vanished_closure: /*Ignored*/Option<&glib::Closure>) -> u32 {
//    unsafe { TODO: call gio_sys:g_bus_watch_name_with_closures() }
//}

pub fn content_type_can_be_executable(type_: &str) -> bool {
    unsafe {
        from_glib(gio_sys::g_content_type_can_be_executable(
            type_.to_glib_none().0,
        ))
    }
}

pub fn content_type_equals(type1: &str, type2: &str) -> bool {
    unsafe {
        from_glib(gio_sys::g_content_type_equals(
            type1.to_glib_none().0,
            type2.to_glib_none().0,
        ))
    }
}

pub fn content_type_from_mime_type(mime_type: &str) -> Option<GString> {
    unsafe {
        from_glib_full(gio_sys::g_content_type_from_mime_type(
            mime_type.to_glib_none().0,
        ))
    }
}

pub fn content_type_get_description(type_: &str) -> Option<GString> {
    unsafe {
        from_glib_full(gio_sys::g_content_type_get_description(
            type_.to_glib_none().0,
        ))
    }
}

pub fn content_type_get_generic_icon_name(type_: &str) -> Option<GString> {
    unsafe {
        from_glib_full(gio_sys::g_content_type_get_generic_icon_name(
            type_.to_glib_none().0,
        ))
    }
}

pub fn content_type_get_icon(type_: &str) -> Option<Icon> {
    unsafe { from_glib_full(gio_sys::g_content_type_get_icon(type_.to_glib_none().0)) }
}

#[cfg(any(feature = "v2_60", feature = "dox"))]
pub fn content_type_get_mime_dirs() -> Vec<GString> {
    unsafe { FromGlibPtrContainer::from_glib_none(gio_sys::g_content_type_get_mime_dirs()) }
}

pub fn content_type_get_mime_type(type_: &str) -> Option<GString> {
    unsafe {
        from_glib_full(gio_sys::g_content_type_get_mime_type(
            type_.to_glib_none().0,
        ))
    }
}

pub fn content_type_get_symbolic_icon(type_: &str) -> Option<Icon> {
    unsafe {
        from_glib_full(gio_sys::g_content_type_get_symbolic_icon(
            type_.to_glib_none().0,
        ))
    }
}

pub fn content_type_guess(filename: Option<&str>, data: &[u8]) -> (GString, bool) {
    let data_size = data.len() as usize;
    unsafe {
        let mut result_uncertain = mem::MaybeUninit::uninit();
        let ret = from_glib_full(gio_sys::g_content_type_guess(
            filename.to_glib_none().0,
            data.to_glib_none().0,
            data_size,
            result_uncertain.as_mut_ptr(),
        ));
        let result_uncertain = result_uncertain.assume_init();
        (ret, from_glib(result_uncertain))
    }
}

pub fn content_type_guess_for_tree<P: IsA<File>>(root: &P) -> Vec<GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(gio_sys::g_content_type_guess_for_tree(
            root.as_ref().to_glib_none().0,
        ))
    }
}

pub fn content_type_is_a(type_: &str, supertype: &str) -> bool {
    unsafe {
        from_glib(gio_sys::g_content_type_is_a(
            type_.to_glib_none().0,
            supertype.to_glib_none().0,
        ))
    }
}

#[cfg(any(feature = "v2_52", feature = "dox"))]
pub fn content_type_is_mime_type(type_: &str, mime_type: &str) -> bool {
    unsafe {
        from_glib(gio_sys::g_content_type_is_mime_type(
            type_.to_glib_none().0,
            mime_type.to_glib_none().0,
        ))
    }
}

pub fn content_type_is_unknown(type_: &str) -> bool {
    unsafe { from_glib(gio_sys::g_content_type_is_unknown(type_.to_glib_none().0)) }
}

#[cfg(any(feature = "v2_60", feature = "dox"))]
pub fn content_type_set_mime_dirs(dirs: &[&str]) {
    unsafe {
        gio_sys::g_content_type_set_mime_dirs(dirs.to_glib_none().0);
    }
}

pub fn content_types_get_registered() -> Vec<GString> {
    unsafe { FromGlibPtrContainer::from_glib_full(gio_sys::g_content_types_get_registered()) }
}

pub fn dbus_address_escape_value(string: &str) -> Option<GString> {
    unsafe {
        from_glib_full(gio_sys::g_dbus_address_escape_value(
            string.to_glib_none().0,
        ))
    }
}

//pub fn dbus_address_get_for_bus_sync<P: IsA<Cancellable>>(bus_type: /*Ignored*/BusType, cancellable: Option<&P>) -> Result<GString, glib::Error> {
//    unsafe { TODO: call gio_sys:g_dbus_address_get_for_bus_sync() }
//}

pub fn dbus_address_get_stream<
    P: IsA<Cancellable>,
    Q: FnOnce(Result<(IOStream, GString), glib::Error>) + Send + 'static,
>(
    address: &str,
    cancellable: Option<&P>,
    callback: Q,
) {
    let user_data: Box_<Q> = Box_::new(callback);
    unsafe extern "C" fn dbus_address_get_stream_trampoline<
        Q: FnOnce(Result<(IOStream, GString), glib::Error>) + Send + 'static,
    >(
        _source_object: *mut gobject_sys::GObject,
        res: *mut gio_sys::GAsyncResult,
        user_data: glib_sys::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let mut out_guid = ptr::null_mut();
        let ret = gio_sys::g_dbus_address_get_stream_finish(res, &mut out_guid, &mut error);
        let result = if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
        callback(result);
    }
    let callback = dbus_address_get_stream_trampoline::<Q>;
    unsafe {
        gio_sys::g_dbus_address_get_stream(
            address.to_glib_none().0,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

pub fn dbus_address_get_stream_future(
    address: &str,
) -> Pin<Box_<dyn std::future::Future<Output = Result<(IOStream, GString), glib::Error>> + 'static>>
{
    let address = String::from(address);
    Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
        let cancellable = Cancellable::new();
        dbus_address_get_stream(&address, Some(&cancellable), move |res| {
            send.resolve(res);
        });

        cancellable
    }))
}

pub fn dbus_address_get_stream_sync<P: IsA<Cancellable>>(
    address: &str,
    cancellable: Option<&P>,
) -> Result<(IOStream, GString), glib::Error> {
    unsafe {
        let mut out_guid = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_dbus_address_get_stream_sync(
            address.to_glib_none().0,
            &mut out_guid,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn dbus_generate_guid() -> Option<GString> {
    unsafe { from_glib_full(gio_sys::g_dbus_generate_guid()) }
}

pub fn dbus_gvalue_to_gvariant(
    gvalue: &glib::Value,
    type_: &glib::VariantTy,
) -> Option<glib::Variant> {
    unsafe {
        from_glib_full(gio_sys::g_dbus_gvalue_to_gvariant(
            gvalue.to_glib_none().0,
            type_.to_glib_none().0,
        ))
    }
}

pub fn dbus_gvariant_to_gvalue(value: &glib::Variant) -> glib::Value {
    unsafe {
        let mut out_gvalue = glib::Value::uninitialized();
        gio_sys::g_dbus_gvariant_to_gvalue(value.to_glib_none().0, out_gvalue.to_glib_none_mut().0);
        out_gvalue
    }
}

pub fn dbus_is_address(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_address(string.to_glib_none().0)) }
}

pub fn dbus_is_guid(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_guid(string.to_glib_none().0)) }
}

pub fn dbus_is_interface_name(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_interface_name(string.to_glib_none().0)) }
}

pub fn dbus_is_member_name(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_member_name(string.to_glib_none().0)) }
}

pub fn dbus_is_name(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_name(string.to_glib_none().0)) }
}

pub fn dbus_is_supported_address(string: &str) -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = gio_sys::g_dbus_is_supported_address(string.to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn dbus_is_unique_name(string: &str) -> bool {
    unsafe { from_glib(gio_sys::g_dbus_is_unique_name(string.to_glib_none().0)) }
}

pub fn io_error_from_errno(err_no: i32) -> IOErrorEnum {
    unsafe { from_glib(gio_sys::g_io_error_from_errno(err_no)) }
}

pub fn io_error_quark() -> glib::Quark {
    unsafe { from_glib(gio_sys::g_io_error_quark()) }
}

//pub fn io_modules_load_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call gio_sys:g_io_modules_load_all_in_directory() }
//}

//pub fn io_modules_load_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call gio_sys:g_io_modules_load_all_in_directory_with_scope() }
//}

pub fn io_modules_scan_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) {
    unsafe {
        gio_sys::g_io_modules_scan_all_in_directory(dirname.as_ref().to_glib_none().0);
    }
}

//pub fn io_modules_scan_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) {
//    unsafe { TODO: call gio_sys:g_io_modules_scan_all_in_directory_with_scope() }
//}

pub fn io_scheduler_cancel_all_jobs() {
    unsafe {
        gio_sys::g_io_scheduler_cancel_all_jobs();
    }
}

//pub fn io_scheduler_push_job<P: IsA<Cancellable>>(job_func: /*Unimplemented*/Fn(/*Ignored*/IOSchedulerJob, Option<&Cancellable>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, io_priority: i32, cancellable: Option<&P>) {
//    unsafe { TODO: call gio_sys:g_io_scheduler_push_job() }
//}

pub fn keyfile_settings_backend_new(
    filename: &str,
    root_path: &str,
    root_group: Option<&str>,
) -> Option<SettingsBackend> {
    unsafe {
        from_glib_full(gio_sys::g_keyfile_settings_backend_new(
            filename.to_glib_none().0,
            root_path.to_glib_none().0,
            root_group.to_glib_none().0,
        ))
    }
}

pub fn memory_settings_backend_new() -> Option<SettingsBackend> {
    unsafe { from_glib_full(gio_sys::g_memory_settings_backend_new()) }
}

pub fn networking_init() {
    unsafe {
        gio_sys::g_networking_init();
    }
}

pub fn null_settings_backend_new() -> Option<SettingsBackend> {
    unsafe { from_glib_full(gio_sys::g_null_settings_backend_new()) }
}

pub fn resources_enumerate_children(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<Vec<GString>, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_resources_enumerate_children(
            path.to_glib_none().0,
            lookup_flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn resources_get_info(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<(usize, u32), glib::Error> {
    unsafe {
        let mut size = mem::MaybeUninit::uninit();
        let mut flags = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = gio_sys::g_resources_get_info(
            path.to_glib_none().0,
            lookup_flags.to_glib(),
            size.as_mut_ptr(),
            flags.as_mut_ptr(),
            &mut error,
        );
        let size = size.assume_init();
        let flags = flags.assume_init();
        if error.is_null() {
            Ok((size, flags))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn resources_lookup_data(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<glib::Bytes, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_resources_lookup_data(
            path.to_glib_none().0,
            lookup_flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn resources_open_stream(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<InputStream, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gio_sys::g_resources_open_stream(
            path.to_glib_none().0,
            lookup_flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn resources_register(resource: &Resource) {
    unsafe {
        gio_sys::g_resources_register(resource.to_glib_none().0);
    }
}

pub fn resources_unregister(resource: &Resource) {
    unsafe {
        gio_sys::g_resources_unregister(resource.to_glib_none().0);
    }
}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_error_in_idle<P: IsA<glib::Object>, Q: FnOnce(Result<(), glib::Error>) + 'static>(object: Option<&P>, callback: Q, domain: glib::Quark, code: i32, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call gio_sys:g_simple_async_report_error_in_idle() }
//}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_gerror_in_idle<P: IsA<glib::Object>, Q: FnOnce(Result<(), glib::Error>) + 'static>(object: Option<&P>, callback: Q, error: &glib::Error) {
//    unsafe { TODO: call gio_sys:g_simple_async_report_gerror_in_idle() }
//}

//#[cfg_attr(feature = "v2_46", deprecated)]
//pub fn simple_async_report_take_gerror_in_idle<P: IsA<glib::Object>, Q: FnOnce(Result<(), glib::Error>) + 'static>(object: Option<&P>, callback: Q, error: &mut glib::Error) {
//    unsafe { TODO: call gio_sys:g_simple_async_report_take_gerror_in_idle() }
//}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_is_mount_path_system_internal<P: AsRef<std::path::Path>>(mount_path: P) -> bool {
    unsafe {
        from_glib(gio_sys::g_unix_is_mount_path_system_internal(
            mount_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
pub fn unix_is_system_device_path<P: AsRef<std::path::Path>>(device_path: P) -> bool {
    unsafe {
        from_glib(gio_sys::g_unix_is_system_device_path(
            device_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
pub fn unix_is_system_fs_type(fs_type: &str) -> bool {
    unsafe { from_glib(gio_sys::g_unix_is_system_fs_type(fs_type.to_glib_none().0)) }
}
