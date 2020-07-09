use libloading::{Library, Symbol};
use libloading::os::unix::Symbol as RawSymbol;
use std::ffi::CString;
use std::os::raw::{c_char, c_void, c_int, c_longlong};
use std::collections::HashMap;

#[repr(C)]
struct XsiInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

#[repr(C)]
struct XsiValue {
    a: c_int,
    b: c_int,
}

impl XsiValue {
    pub fn from_i32(input: i32) -> XsiValue {
        XsiValue { a: input as c_int, b: 0 }
    }

    pub fn to_i32(input: XsiValue) -> i32 {
        input.a as i32
    }
}

type XsiHandle = *mut c_void;
type XsiOpen = fn(*const XsiInfo) -> XsiHandle;
type XsiGetPortNumber = fn(XsiHandle, *const c_char) -> c_int;
type XsiPutValue = fn(XsiHandle, c_int, *const XsiValue) -> ();
type XsiGetValue = fn(XsiHandle, c_int, *const XsiValue) -> c_int;
type XsiRun = fn(XsiHandle, c_longlong) -> ();
type XsiClose = fn(XsiHandle) -> ();

#[derive(Clone, Debug)]
struct XsiTable {
    get_port_name: RawSymbol<XsiGetPortNumber>,
    put_value: RawSymbol<XsiPutValue>,
    get_value: RawSymbol<XsiGetValue>,
    run: RawSymbol<XsiRun>,
    close: RawSymbol<XsiClose>,
}

impl XsiTable {
    pub unsafe fn new(lib: &Library) -> XsiTable {
        let xsi_get_port_name: Symbol<XsiGetPortNumber> = lib.get(b"xsi_get_port_number")
                .expect("Error: could not find xsi_get_port_number");
        let xsi_put_value: Symbol<XsiPutValue> = lib.get(b"xsi_put_value")
            .expect("Error: could not find xsi_put_value");
        let xsi_get_value: Symbol<XsiGetValue> = lib.get(b"xsi_get_value")
            .expect("Error: could not find xsi_get_value");
        let xsi_run: Symbol<XsiRun> = lib.get(b"xsi_run")
            .expect("Error: could not find xsi_run");
        let xsi_close: Symbol<XsiClose> = lib.get(b"xsi_close")
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

pub type Xsim = Xsi;

#[derive(Debug)]
pub struct Xsi {
    design_lib: Library,
    xsi_lib: Library,
    handle: XsiHandle,
    table: XsiTable,
    ports: HashMap<String, c_int>,
}

impl Xsi {
    pub fn new() -> Xsi {
        let design_lib = Library::new("xsim/xsim.dir/work.testbench/xsimk.so")
            .expect("Error: could not load design lib");
        let xsi_lib = Library::new("/tools/Xilinx/Vivado/2020.1/lib/lnx64.o/librdi_simulator_kernel.so")
            .expect("Error: could not load sim lib");
        let log_file = CString::new("").expect("Error: specifying path"); // empty for now
        let wdb_file = CString::new("").expect("Error: specifying path"); // empty for now
        let info = XsiInfo {
            log_file: log_file.as_ptr(),
            wdb_file: wdb_file.as_ptr(),
        };
        unsafe {
            let xsi_open: Symbol<XsiOpen> = design_lib.get(b"xsi_open")
                .expect("Error: could not find xsi_open");
            let handle = xsi_open(&info);
            let table = XsiTable::new(&xsi_lib);
            Xsi {
                design_lib: design_lib,
                xsi_lib: xsi_lib,
                handle: handle,
                table: table,
                ports: HashMap::new(),
            }
        }
    }

    fn add_port(&mut self, name: &str) -> c_int {
        if !self.ports.contains_key(name) {
            let port_name = CString::new(name).expect(&format!("Error: specifying {} name", name));
            let port_id = (self.table.get_port_name)(self.handle, port_name.as_ptr());
            self.ports.insert(name.to_string(), port_id);
            port_id
        } else {
            self.ports.get(name).unwrap().clone()
        }
    }

    pub fn poke(&mut self, name: &str, value: i32) {
        let port_id = self.add_port(name);
        (self.table.put_value)(self.handle, port_id, &XsiValue::from_i32(value));
    }

    pub fn peek(&mut self, name: &str) -> i32 {
        let port_id = self.add_port(name);
        let mut value = XsiValue { a: 0, b: 0 };
        (self.table.get_value)(self.handle, port_id, &mut value);
        XsiValue::to_i32(value)
    }

    pub fn eval(&self) {
        // eval 10 time-units, no need to expose this?
        (self.table.run)(self.handle, 10 as c_longlong);
    }
}

impl Drop for Xsi {
    fn drop(&mut self) {
        (self.table.close)(self.handle);
    }
}
