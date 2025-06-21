use crate::Id;
use crate::{objc_getClass, objc_msgSend, sel_registerName, Selector};
use std::ffi::c_char;
use std::mem;

/// import_class!
///
/// Generates:
///  • `pub struct Name(pub Id);`  
///  • for each `ctor foo(args) = "sel:";`
///      `pub unsafe fn foo(args) -> Self { /* msg_send, wrap in Self */ }`  
///  • for each `fn bar(&self, …) ->Ret = "sel2:";`
///      `pub unsafe fn bar(&self, …)->Ret { /* msg_send on self.0 */ }`
#[macro_export]
macro_rules! import_class {
    (
        struct $StructName:ident {
            // zero-arg or multi-arg ctors
            $(
                ctor $ctor:ident ( $( $carg:ident : $carg_ty:ty ),* ) = $ctor_sel:literal ;
            )*
            // instance methods
            $(
                fn $method:ident(
                    &self $(, $arg:ident : $arg_ty:ty )*
                ) -> $ret:ty = $sel_name:literal ;
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $StructName(pub Id);

        impl $StructName {
            $(
                #[allow(non_snake_case)]
                pub unsafe fn $ctor( $( $carg : $carg_ty ),* ) -> Self {
                    // register the selector
                    let sel = sel_registerName(
                        concat!($ctor_sel, "\0").as_ptr() as *const c_char
                    );
                    // get the `Class` object
                    let cls = objc_getClass(
                        concat!(stringify!($StructName), "\0")
                            .as_ptr() as *const c_char
                    );
                    // cast objc_msgSend -> fn(Id,Selector, …) -> Id
                    let msg_send: unsafe extern "C" fn(
                        Id,
                        Selector
                        $(, $carg_ty)*
                    ) -> Id =
                        mem::transmute(objc_msgSend as *const ());
                    // invoke + wrap
                    $StructName(msg_send(cls, sel $(, $carg)*))
                }
            )*

            $(
                #[allow(non_snake_case)]
                pub unsafe fn $method(
                    &self,
                    $( $arg : $arg_ty ),*
                ) -> $ret {
                    let sel = sel_registerName(
                        concat!($sel_name, "\0").as_ptr() as *const c_char
                    );
                    let msg_send: unsafe extern "C" fn(
                        Id,
                        Selector
                        $(, $arg_ty)*
                    ) -> $ret =
                        mem::transmute(objc_msgSend as *const ());
                    msg_send(self.0, sel $(, $arg)*)
                }
            )*
        }
    };
}

// NSString as a wrapper: its `new(...)` is the old
// `stringWithUTF8String:` class‐method.
import_class! {
    struct NSString {
        ctor new(utf8: *const c_char) = "stringWithUTF8String:";
    }
}

// NSFileManager with a zero‐arg `default()` and an instance method.
import_class! {
    struct NSFileManager {
        ctor default() = "defaultManager";
        fn contentsOfDirectory(
            &self,
            path: Id,
            error: *mut Id
        ) -> Id = "contentsOfDirectoryAtPath:error:";
    }
}