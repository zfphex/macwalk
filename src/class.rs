use std::os::raw::c_char;
use std::mem;
use crate::{objc_msgSend, sel_registerName, Selector, Id};

/// import_class!
///
/// Given a struct name and a list of instance‐method signatures + selectors,
/// this will:
///  1. define `pub struct $N(pub Id);`  
///  2. impl one `pub unsafe fn` per `fn … = "sel:";` for you  
///
/// Example:
/// ```rust
/// import_class! {
///     struct NSFileManager {
///         fn contentsOfDirectory(
///             &self,
///             path: Id,
///             error: *mut Id
///         ) -> Id = "contentsOfDirectoryAtPath:error:";
///     }
/// }
/// // expands to:
/// // pub struct NSFileManager(pub Id);
/// // impl NSFileManager {
/// //   pub unsafe fn contentsOfDirectory(&self, path: Id, error: *mut Id) -> Id { … }
/// // }
/// ```
#[macro_export]
macro_rules! import_class {
    (
        struct $struct_name:ident {
            $(
                fn $method:ident(
                    &self $(, $arg:ident : $arg_ty:ty )*
                ) -> $ret:ty = $sel_name:literal ;
            )*
        }
    ) => {
        /// A thin newtype wrapper around an Objective-C Id.
        #[derive(Debug, Clone, Copy)]
        pub struct $struct_name(pub Id);

        impl $struct_name {
            $(
                #[allow(non_snake_case)]
                pub unsafe fn $method(&self, $($arg: $arg_ty),* ) -> $ret {
                    // build selector + register it
                    let sel_cstr = concat!($sel_name, "\0");
                    let sel = sel_registerName(sel_cstr.as_ptr() as *const c_char);
                    // cast objc_msgSend into the right fn‐type
                    let msg_send: unsafe extern "C" fn(
                        Id,
                        Selector
                        $(, $arg_ty)*
                    ) -> $ret = mem::transmute(objc_msgSend as *const ());
                    // call it on our wrapped `Id`
                    msg_send(self.0, sel $(, $arg)*)
                }
            )*
        }
    };
}

import_class! {
    struct NSFileManager {
        fn contentsOfDirectory(
            &self,
            path: Id,
            error: *mut Id
        ) -> Id = "contentsOfDirectoryAtPath:error:";
    }
}