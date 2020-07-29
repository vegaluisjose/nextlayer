use libloading::os::unix::Symbol as RawSymbol;
use libloading::Library;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int, c_longlong, c_void};

pub type XsiHandle = *mut c_void;
pub type XsiOpen = fn(*const XsiInfo) -> XsiHandle;
pub type XsiGetPortNumber = fn(XsiHandle, *const c_char) -> c_int;
pub type XsiPutValue = fn(XsiHandle, c_int, *const XsiValue) -> ();
pub type XsiGetValue = fn(XsiHandle, c_int, *const XsiValue) -> c_int;
pub type XsiRun = fn(XsiHandle, c_longlong) -> ();
pub type XsiClose = fn(XsiHandle) -> ();

#[repr(C)]
pub struct XsiInfo {
    pub log_file: *const c_char,
    pub wdb_file: *const c_char,
}

#[repr(C)]
pub struct XsiValue {
    pub a: c_int,
    pub b: c_int,
}

#[derive(Clone, Debug)]
pub struct XsiTable {
    pub get_port_name: RawSymbol<XsiGetPortNumber>,
    pub put_value: RawSymbol<XsiPutValue>,
    pub get_value: RawSymbol<XsiGetValue>,
    pub run: RawSymbol<XsiRun>,
    pub close: RawSymbol<XsiClose>,
}

#[derive(Debug)]
pub struct Xsi {
    pub design_lib: Library,
    pub xsi_lib: Library,
    pub handle: XsiHandle,
    pub table: XsiTable,
    pub ports: HashMap<String, c_int>,
}
