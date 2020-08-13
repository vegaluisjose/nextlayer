use nextlayer::backend::xsim::Xsim;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::path::Path;

/// # Safety
///
/// This function allocates xsim object, should be called before anything else
#[no_mangle]
pub unsafe extern "C" fn alloc(lib: *const c_char) -> *mut Xsim {
    assert!(!lib.is_null());
    let path_str = CStr::from_ptr(lib);
    let path = Path::new(path_str.to_str().unwrap());
    let boxed = Box::new(Xsim::new(&path));
    Box::into_raw(boxed)
}

/// # Safety
///
/// This function writes to a register, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn write_reg(handle: *mut Xsim, value: c_int, id: c_int, mask: c_int) {
    let xsim = &mut handle.as_mut().unwrap();
    xsim.poke("opcode", 1);
    xsim.poke("mask", mask as i32);
    xsim.poke("id", id as i32);
    xsim.poke("in", value as i32);
    xsim.eval();
}

/// # Safety
///
/// This function reads from a register, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn read_reg(handle: *mut Xsim, id: c_int, mask: c_int) -> i32 {
    let xsim = &mut handle.as_mut().unwrap();
    xsim.poke("opcode", 2);
    xsim.poke("mask", mask as i32);
    xsim.poke("id", id as i32);
    xsim.eval();
    xsim.peek("out") as i32
}

/// # Safety
///
/// This function writes to memory, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn write_mem(
    handle: *mut Xsim,
    value: c_int,
    addr: c_int,
    id: c_int,
    mask: c_int,
) {
    let xsim = &mut handle.as_mut().unwrap();
    xsim.poke("opcode", 3);
    xsim.poke("mask", mask as i32);
    xsim.poke("id", id as i32);
    xsim.poke("in", value as i32);
    xsim.poke("addr", addr as i32);
    xsim.eval();
}

/// # Safety
///
/// This function reads from memory, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn read_mem(handle: *mut Xsim, addr: c_int, id: c_int, mask: c_int) -> i32 {
    let xsim = &mut handle.as_mut().unwrap();
    xsim.poke("opcode", 4);
    xsim.poke("mask", mask as i32);
    xsim.poke("id", id as i32);
    xsim.poke("addr", addr as i32);
    xsim.eval();
    xsim.peek("out") as i32
}

/// # Safety
///
/// This function asserts reset for n cycles, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn reset(handle: *mut Xsim, cycles: c_int) {
    let xsim = &mut handle.as_mut().unwrap();
    assert!(cycles > 0, "Error: cycles must be greater than zero");
    let n = cycles as i32;
    for _ in 0..n {
        xsim.poke("reset", 1);
        xsim.poke("clock", 1);
        xsim.eval();
        xsim.poke("clock", 0);
        xsim.eval();
    }
    xsim.poke("reset", 0);
    xsim.eval();
}

/// # Safety
///
/// This function runs for n cycles, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn run(handle: *mut Xsim, cycles: c_int) {
    let xsim = &mut handle.as_mut().unwrap();
    assert!(cycles > 0, "Error: cycles must be greater than zero");
    let n = cycles as i32;
    for _ in 0..n {
        xsim.poke("clock", 1);
        xsim.eval();
        xsim.poke("clock", 0);
        xsim.eval();
    }
}

/// # Safety
///
/// This function deallocates xsim object, should be called after allocating
#[no_mangle]
pub unsafe extern "C" fn dealloc(handle: *mut Xsim) {
    let xsim = handle.as_ref().unwrap();
    xsim.free();
    Box::from_raw(handle);
}
