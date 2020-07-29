use crate::backend::xsim::Xsim;
use std::ffi::CStr;
use std::os::raw::c_char;
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
/// This function deallocates xsim object, should be called after allocating
pub unsafe extern "C" fn dealloc(handle: *mut Xsim) {
    let xsim = handle.as_ref().unwrap();
    xsim.free();
    Box::from_raw(handle);
}
