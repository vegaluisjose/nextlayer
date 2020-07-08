use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::{c_char, c_void, c_int, c_longlong};
use std::collections::HashMap;

#[repr(C)]
pub struct XsiInfo {
    log_file: *const c_char,
    wdb_file: *const c_char,
}

#[repr(C)]
pub struct XsiValue {
    a: c_int,
    b: c_int,
}

type XsiHandle = *mut c_void;
type XsiOpen = fn(*const XsiInfo) -> XsiHandle;
type XsiGetPortNumber = fn(XsiHandle, *const c_char) -> c_int;
type XsiPutValue = fn(XsiHandle, c_int, *const XsiValue) -> ();
type XsiGetValue = fn(XsiHandle, c_int, *const XsiValue) -> c_int;
type XsiRun = fn(XsiHandle, c_longlong) -> ();
type XsiClose = fn(XsiHandle) -> ();

pub struct Xsi {
    lib: Library,
    handle: XsiHandle,
    ports: HashMap<String, c_int>,
}

impl Xsi {
    pub fn new() -> Xsi {
        let design_lib = Library::new("xsim/xsim.dir/work.testbench/xsimk.so")
            .expect("Error: could not load design lib");
        let log_file = CString::new("").expect("Error: specifying path"); // empty for now
        let wdb_file = CString::new("").expect("Error: specifying path"); // empty for now
        let info = XsiInfo {
            log_file: log_file.as_ptr(),
            wdb_file: wdb_file.as_ptr(),
        };
        unsafe {
            let xsi_open: Symbol<XsiOpen> = design_lib.get(b"xsi_open")
                .expect("Error: could not find xsi_open");
            Xsi {
                lib: Library::new("/tools/Xilinx/Vivado/2020.1/lib/lnx64.o/librdi_simulator_kernel.so")
                    .expect("Error: could not load sim lib"),
                handle: xsi_open(&info),
                ports: HashMap::new(),
            }
        }
    }

    fn add_port(&mut self, name: &str) {
        if !self.ports.contains_key(name) {
            let port_name = CString::new(name)
                .expect(&format!("Error: specifying {} name", name));
            unsafe {
                let xsi_get_port_number: Symbol<XsiGetPortNumber> = self.lib.get(b"xsi_get_port_number")
                    .expect("Error: could not find xsi_get_port_number");
                let port_id = xsi_get_port_number(self.handle, port_name.as_ptr());
                self.ports.insert(name.to_string(), port_id);
            }
        }
    }

    pub fn poke_port(&mut self, name: &str, value: i32) {
        self.add_port(name);
        println!("{} id:{}", name, self.ports.get(name).unwrap());
    }
}

#[no_mangle]
pub extern fn run_xsim() {
    println!("Running...");
    let mut sim = Xsi::new();
    sim.poke_port("reset", 0);
    println!("Finishing...");
    //let clock_name = CString::new("clock").expect("Error: specifying clock name");
    //let reset_name = CString::new("reset").expect("Error: specifying reset name");
    //let count_name = CString::new("count").expect("Error: specifying count name");
    //let info = XsiInfo {
    //    log_file: log_file.as_ptr(),
    //    wdb_file: wdb_file.as_ptr(),
    //};
    //let one = XsiValue { a: 1, b: 0 };
    //let zero = XsiValue { a: 0, b: 0 };
    //let mut val = XsiValue { a: 0, b: 0 };
    //unsafe {
    //    let xsi_get_port_number: Symbol<XsiGetPortNumber> = xsi_lib.get(b"xsi_get_port_number").expect("Error: could not find xsi_get_port_number");
    //    let xsi_put_value: Symbol<XsiPutValue> = xsi_lib.get(b"xsi_put_value").expect("Error: could not find xsi_put_value");
    //    let xsi_get_value: Symbol<XsiGetValue> = xsi_lib.get(b"xsi_get_value").expect("Error: could not find xsi_get_value");
    //    let xsi_run: Symbol<XsiRun> = xsi_lib.get(b"xsi_run").expect("Error: could not find xsi_run");
    //    let xsi_close: Symbol<XsiClose> = xsi_lib.get(b"xsi_close").expect("Error: could not find xsi_close");

    //    let clock = xsi_get_port_number(design_handle, clock_name.as_ptr());
    //    let reset = xsi_get_port_number(design_handle, reset_name.as_ptr());
    //    let count = xsi_get_port_number(design_handle, count_name.as_ptr());

    //    println!("clock id:{}", clock);
    //    println!("reset id:{}", reset);
    //    println!("count id:{}", count);

    //    for _ in 0..10 {
    //        xsi_put_value(design_handle, reset.clone(), &one);
    //        xsi_put_value(design_handle, clock.clone(), &zero);
    //        xsi_run(design_handle, 10);
    //        xsi_put_value(design_handle, clock.clone(), &one);
    //        xsi_run(design_handle, 10);
    //    }
    //    xsi_put_value(design_handle, reset.clone(), &zero);

    //    for _ in 0..10 {
    //        xsi_put_value(design_handle, clock.clone(), &zero);
    //        xsi_run(design_handle, 10);
    //        xsi_put_value(design_handle, clock.clone(), &one);
    //        xsi_run(design_handle, 10);
    //        xsi_get_value(design_handle, count.clone(), &mut val);
    //        println!("count:{}", val.a);
    //    }

    //    xsi_close(design_handle);
    //    println!("End...");
    //}
}
