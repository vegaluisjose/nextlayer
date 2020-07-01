use std::ffi::CString;
use std::os::raw::{c_void, c_char};
use libloading::{Library, Symbol};

#[repr(C)]
pub struct XsimInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

type XsimOpen = fn(*const XsimInfo) -> *mut c_void;

pub struct Xsim {
    tb_handle: *mut c_void,
}

impl Xsim {
    pub fn new() -> Xsim {
        let tb_lib = Library::new("xsim/xsim.dir/work.testbench/xsimk.so").expect("Error: could not load testbench lib");
        let path = CString::new("").expect("Error: specifying path"); // empty for now
        unsafe {
            let xsim_info = XsimInfo{ log_file: path.as_ptr(), wdb_file: path.as_ptr() };
            let xsim_open: Symbol<XsimOpen> = tb_lib.get(b"xsi_open").expect("Error: could not find xsi_open");
            Xsim { tb_handle: xsim_open(&xsim_info) }
        }
    }
}
