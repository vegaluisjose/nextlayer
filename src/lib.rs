use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;

#[repr(C)]
pub struct XsimInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

type XsimOpen = fn(*const XsimInfo) -> *mut c_void;

pub struct Xsim {
    sim_lib: Library,
    design_handle: *mut c_void,
}

impl Xsim {
    pub fn new() -> Xsim {
        let design_handle = Library::new("xsim/xsim.dir/work.testbench/xsimk.so")
            .expect("Error: could not load design lib");
        let path = CString::new("").expect("Error: specifying path"); // empty for now
        let lnx64_path = Path::new("/tools/Xilinx/Vivado/2020.1/lib/lnx64.o");
        unsafe {
            let xsim_info = XsimInfo {
                log_file: path.as_ptr(),
                wdb_file: path.as_ptr(),
            };
            let xsim_open: Symbol<XsimOpen> = design_handle
                .get(b"xsi_open")
                .expect("Error: could not find xsi_open");
            Xsim {
                sim_lib: Library::new(lnx64_path.join("librdi_simulator_kernel.so"))
                    .expect("Error: could not load sim lib"),
                design_handle: xsim_open(&xsim_info),
            }
        }
    }
}
