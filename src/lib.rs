use crate::backend::xsim::Xsim;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::path::Path;

pub mod backend;

#[no_mangle]
pub extern "C" fn run_add() {
    println!("Running add...");
    let design_lib_path = Path::new("designs/add/xsim.dir/work.testbench/xsimk.so");
    let mut sim = Xsim::new(&design_lib_path);
    // reset for 10 cycles
    sim.poke("mask", 0);
    for _ in 0..10 {
        sim.poke("reset", 1);
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    sim.poke("reset", 0);
    sim.eval();
    // write a
    sim.poke("opcode", 1);
    sim.poke("id", 0);
    sim.poke("in", 3);
    sim.eval();
    // write mem
    sim.poke("opcode", 3);
    sim.poke("id", 0);
    sim.poke("in", 9);
    sim.poke("addr", 4);
    sim.eval();
    // read mem
    sim.poke("opcode", 4);
    sim.poke("id", 0);
    sim.poke("addr", 4);
    sim.eval();
    println!("read-back from mem:{}", sim.peek("out"));
    // run for 1 cycle
    for _ in 0..1 {
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    // read y
    sim.poke("opcode", 2);
    sim.poke("id", 1);
    sim.eval();
    println!("adder result:{}", sim.peek("out"));
    sim.free();
    println!("Finishing add...");
}

#[no_mangle]
pub extern "C" fn run_vadd() {
    println!("Running vadd...");
    let design_lib_path = Path::new("designs/vadd/xsim.dir/work.testbench/xsimk.so");
    let mut sim = Xsim::new(&design_lib_path);
    // reset for 10 cycles
    sim.poke("mask", 0);
    for _ in 0..10 {
        sim.poke("reset", 1);
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    sim.poke("reset", 0);
    sim.eval();
    // write mem
    for i in 0..8 {
        sim.poke("opcode", 3);
        sim.poke("id", 0);
        sim.poke("in", i);
        sim.poke("addr", i);
        sim.eval();
    }
    // write a
    sim.poke("opcode", 1);
    sim.poke("id", 2);
    sim.poke("in", 0); // first vector starts at 0 address nothing to do here
    sim.eval();
    // write b
    sim.poke("opcode", 1);
    sim.poke("id", 3);
    sim.poke("in", 16); // address in bytes, this vector starts at index*num_bytes_per_word, 4x4
    sim.eval();
    // write c
    sim.poke("opcode", 1);
    sim.poke("id", 4);
    sim.poke("in", 32); // result vector starts at 8x4
    sim.eval();
    // write length
    sim.poke("opcode", 1);
    sim.poke("id", 5);
    sim.poke("in", 4);
    sim.eval();
    // write start
    sim.poke("opcode", 1);
    sim.poke("id", 0);
    sim.poke("in", 1);
    sim.eval();
    // run for 1000 cycle
    for _ in 0..1000 {
        sim.poke("clock", 1);
        sim.eval();
        sim.poke("clock", 0);
        sim.eval();
    }
    // read done
    sim.poke("opcode", 2);
    sim.poke("id", 1);
    sim.eval();
    println!("done:{}", sim.peek("out"));
    // read mem
    for i in 0..12 {
        sim.poke("opcode", 4);
        sim.poke("id", 0);
        sim.poke("addr", i);
        sim.eval();
        println!("mem[{}]:{}", i, sim.peek("out"));
    }
    sim.free();
    println!("Finishing vadd...");
}

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
