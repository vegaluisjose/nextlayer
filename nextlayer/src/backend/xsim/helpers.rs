use crate::backend::xsim::xsi::*;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::os::raw::{c_int, c_longlong};
use std::path::Path;

impl XsiTable {
    pub unsafe fn new(lib: &Library) -> XsiTable {
        let xsi_get_port_name: Symbol<XsiGetPortNumber> = lib
            .get(b"xsi_get_port_number")
            .expect("Error: could not find xsi_get_port_number");
        let xsi_put_value: Symbol<XsiPutValue> = lib
            .get(b"xsi_put_value")
            .expect("Error: could not find xsi_put_value");
        let xsi_get_value: Symbol<XsiGetValue> = lib
            .get(b"xsi_get_value")
            .expect("Error: could not find xsi_get_value");
        let xsi_run: Symbol<XsiRun> = lib.get(b"xsi_run").expect("Error: could not find xsi_run");
        let xsi_close: Symbol<XsiClose> = lib
            .get(b"xsi_close")
            .expect("Error: could not find xsi_close");
        XsiTable {
            get_port_name: xsi_get_port_name.into_raw(),
            put_value: xsi_put_value.into_raw(),
            get_value: xsi_get_value.into_raw(),
            run: xsi_run.into_raw(),
            close: xsi_close.into_raw(),
        }
    }
}

impl Xsi {
    pub fn new(design_lib_path: &Path) -> Xsi {
        let env_var = env::var("XILINX_VIVADO")
            .expect("Error: Vivado not found, have you sourced settings64.sh?");
        let xilinx_path = Path::new(&env_var);
        let rdi_path = xilinx_path.join("lib/lnx64.o/librdi_simulator_kernel.so");
        let design_lib = Library::new(design_lib_path).expect("Error: could not load design lib");
        let xsi_lib = Library::new(&rdi_path).expect("Error: could not load sim lib");
        let log_file = CString::new("").expect("Error: specifying path"); // empty for now
        let wdb_file = CString::new("").expect("Error: specifying path"); // empty for now
        let info = XsiInfo {
            log_file: log_file.as_ptr(),
            wdb_file: wdb_file.as_ptr(),
        };
        unsafe {
            let xsi_open: Symbol<XsiOpen> = design_lib
                .get(b"xsi_open")
                .expect("Error: could not find xsi_open");
            let handle = xsi_open(&info);
            let table = XsiTable::new(&xsi_lib);
            Xsi {
                design_lib,
                xsi_lib,
                handle,
                table,
                ports: HashMap::new(),
            }
        }
    }

    fn add_port(&mut self, name: &str) -> c_int {
        if !self.ports.contains_key(name) {
            let port_name =
                CString::new(name).unwrap_or_else(|_| panic!("Error: specifying {} name", name));
            let port_id = (self.table.get_port_name)(self.handle, port_name.as_ptr());
            self.ports.insert(name.to_string(), port_id);
            port_id
        } else {
            *self.ports.get(name).unwrap()
        }
    }

    pub fn poke(&mut self, name: &str, value: i32) {
        let port_id = self.add_port(name);
        (self.table.put_value)(self.handle, port_id, &XsiValue::from(value));
    }

    pub fn peek(&mut self, name: &str) -> i32 {
        let port_id = self.add_port(name);
        let mut value = XsiValue { a: 0, b: 0 };
        (self.table.get_value)(self.handle, port_id, &mut value);
        i32::from(value)
    }

    pub fn eval(&self) {
        // eval 10 time-units, no need to expose this?
        (self.table.run)(self.handle, 10 as c_longlong);
    }

    pub fn free(&self) {
        (self.table.close)(self.handle);
    }
}
