use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::{c_char, c_void, c_int, c_longlong};

#[repr(C)]
pub struct XsiInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

type XsiHandle = *mut c_void;
type XsiOpen = fn(*const XsiInfo) -> XsiHandle;
type XsiGetPortNumber = fn(XsiHandle, *const c_char) -> c_int;
type XsiPutValue = fn(XsiHandle, c_int, *mut c_void) -> ();
type XsiGetValue = fn(XsiHandle, c_int, *mut c_void) -> c_int;
type XsiRun = fn(XsiHandle, c_longlong) -> ();
type XsiClose = fn(XsiHandle) -> ();

fn main() {
    println!("Running...");
    let design_lib = Library::new("/home/vega/nextlayer/xsim/xsim.dir/work.testbench/xsimk.so")
            .expect("Error: could not load design lib");
    let sim_lib = Library::new("/tools/Xilinx/Vivado/2020.1/lib/lnx64.o/librdi_simulator_kernel.so")
            .expect("Error: could not load sim lib");
    let log_file = CString::new("").expect("Error: specifying path"); // empty for now
    let wdb_file = CString::new("").expect("Error: specifying path"); // empty for now
    let clock_name = CString::new("clock").expect("Error: specifying clock name");
    let reset_name = CString::new("reset").expect("Error: specifying reset name");
    let count_name = CString::new("count").expect("Error: specifying count name");
    let info = XsiInfo {
        log_file: log_file.as_ptr(),
        wdb_file: wdb_file.as_ptr(),
    };
    unsafe {
        let xsi_open: Symbol<XsiOpen> = design_lib.get(b"xsi_open").expect("Error: could not find xsi_open");
        let design_handle: XsiHandle = xsi_open(&info);
        let xsi_get_port_number: Symbol<XsiGetPortNumber> = design_lib.get(b"xsi_get_port_number").expect("Error: could not find xsi_get_port_number");
        let xsi_put_value: Symbol<XsiPutValue> = design_lib.get(b"xsi_put_value").expect("Error: could not find xsi_put_value");
        let xsi_get_value: Symbol<XsiGetValue> = design_lib.get(b"xsi_get_value").expect("Error: could not find xsi_get_value");
        let xsi_run: Symbol<XsiRun> = design_lib.get(b"xsi_run").expect("Error: could not find xsi_run");
        let xsi_close: Symbol<XsiClose> = design_lib.get(b"xsi_close").expect("Error: could not find xsi_close");
        let clock = xsi_get_port_number(design_handle, clock_name.as_ptr());
        let reset = xsi_get_port_number(design_handle, reset_name.as_ptr());
        let count = xsi_get_port_number(design_handle, count_name.as_ptr());
        println!("clock id:{}", clock);
        println!("reset id:{}", reset);
        println!("count id:{}", count);
    }
}
