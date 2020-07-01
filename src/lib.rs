use std::os::raw::c_int;
use libloading::{Library, Symbol};

type CCAdd = fn(c_int, c_int) -> c_int;

#[no_mangle]
pub extern fn wrapper_add(a: i32, b: i32) -> i32 {
    let lib = Library::new("lib/libadd.so")
        .expect("Error: could not load shared lib");
    unsafe {
        let cc_add: Symbol<CCAdd> = lib
            .get(b"cc_add")
            .expect("Error: could not find cc_add function");
        cc_add(a, b)
    }
}
