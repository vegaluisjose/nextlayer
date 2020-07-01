use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;
use std::collections::HashMap;

#[repr(C)]
pub struct XsimInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

type XsimOpen = fn(*const XsimInfo) -> *mut c_void;
type XsimGetPortNumber = fn(*mut c_void, *const c_char) -> i32;

pub struct Xsim {
    sim_lib: Library,
    design_handle: *mut c_void,
    port_map: HashMap<String, i32>,
}

impl Xsim {
    pub fn new() -> Xsim {
        let design_lib = Library::new("xsim/xsim.dir/work.testbench/xsimk.so")
            .expect("Error: could not load design lib");
        let path = CString::new("").expect("Error: specifying path"); // empty for now
        let lnx64_path = Path::new("/tools/Xilinx/Vivado/2020.1/lib/lnx64.o");
        unsafe {
            let xsim_info = XsimInfo {
                log_file: path.as_ptr(),
                wdb_file: path.as_ptr(),
            };
            let xsim_open: Symbol<XsimOpen> = design_lib
                .get(b"xsi_open")
                .expect("Error: could not find xsi_open");
            Xsim {
                sim_lib: Library::new(lnx64_path.join("librdi_simulator_kernel.so"))
                    .expect("Error: could not load sim lib"),
                design_handle: xsim_open(&xsim_info),
                port_map: HashMap::new(),
            }
        }
    }

    pub fn create_port(&mut self, name: &str) {
        let port = CString::new(name).expect("Error: converting port name to CString");
        unsafe {
            let xsim_get_port_number: Symbol<XsimGetPortNumber> = self.sim_lib
                .get(b"xsi_get_port_name")
                .expect("Error: could not find xsi_get_port_name");
            let id = xsim_get_port_number(self.design_handle, port.as_ptr());
            self.port_map.insert(name.to_string(), id);
        }
    }
}
