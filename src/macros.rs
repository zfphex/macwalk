use crate::{objc_msgSend, sel_registerName, Id, Selector};
use std::mem;
use std::{ffi::c_ulong, os::raw::c_char};

/// import!
///
/// Generates an `unsafe fn $name(...) -> Ret` wrapper around
/// `sel_registerName` + `mem::transmute(objc_msgSend…)`.
///
/// Example:
/// ```ignore
/// import! {
///     fn contentsOfDirectory(
///         file_manager: Id,
///         path: Id,
///         error: *mut Id
///     ) -> Id = "contentsOfDirectoryAtPath:error:";
/// }
/// ```
#[macro_export]
macro_rules! import {
    (
        fn $fn_name:ident(
            $receiver:ident : $recv_ty:ty
            $(, $arg_name:ident : $arg_ty:ty)*
        ) -> $ret:ty = $sel_name:literal $(;)?
    ) => {
        pub unsafe fn $fn_name(
            $receiver: $recv_ty,
            $( $arg_name: $arg_ty ),*
        ) -> $ret {
            // build "selector\0" at compile time
            let sel_cstr = concat!($sel_name, "\0");
            // register it
            let sel = sel_registerName(sel_cstr.as_ptr() as *const c_char);
            // cast objc_msgSend to the proper fn signature
            let msg_send: unsafe extern "C" fn(
                $recv_ty,
                Selector
                $(, $arg_ty)*
            ) -> $ret = mem::transmute(objc_msgSend as *const ());
            // finally invoke
            msg_send($receiver, sel $(, $arg_name)*)
        }
    };
}

import! {
    fn contentsOfDirectory(
        file_manager: Id,
        path: Id,
        error: *mut Id
    ) -> Id = "contentsOfDirectoryAtPath:error:";
}

import! {
    fn stringWithUTF8String(
        nsstring_class: Id,
        utf8: *const c_char
    ) -> Id = "stringWithUTF8String:";
}

import! {
    fn defaultManager(
        nsfilemanager_class: Id
    ) -> Id = "defaultManager";
}

import! {
    fn count(
        array: Id
    ) -> c_ulong = "count";
}

import! {
    fn objectAtIndex(
        array: Id,
        idx: c_ulong
    ) -> Id = "objectAtIndex:";
}

import! {
    fn UTF8String(
        nsstring: Id
    ) -> *const c_char = "UTF8String";
}
