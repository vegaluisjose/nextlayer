use crate::backend::xsim::xsi::XsiValue;
use std::convert::From;
use std::os::raw::c_int;

impl From<i32> for XsiValue {
    fn from(input: i32) -> Self {
        XsiValue {
            a: input as c_int,
            b: 0,
        }
    }
}

impl From<XsiValue> for i32 {
    fn from(input: XsiValue) -> Self {
        input.a as i32
    }
}
