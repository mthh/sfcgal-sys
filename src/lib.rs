#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::sync::{Once, ONCE_INIT};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn initialize() {
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| unsafe {
        sfcgal_init();
        w_sfcgal_init_handlers();
    });
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::{CStr, CString};

    fn _string(raw_ptr: *const std::os::raw::c_char) -> String {
        let c_str = unsafe { CStr::from_ptr(raw_ptr) };
        std::str::from_utf8(c_str.to_bytes()).unwrap().to_string()
    }

    #[test]
    fn it_works() {
        initialize();
        let wkt1 = CString::new("POINT(1.0 1.0)").expect("CString::new failed");
        let point1 = unsafe { sfcgal_io_read_wkt(wkt1.as_ptr(), 14) };
        let wkt2 = CString::new("POINT(10.0 1.0)").expect("CString::new failed");
        let point2 = unsafe { sfcgal_io_read_wkt(wkt2.as_ptr(), 15) };
        let distance = unsafe { sfcgal_geometry_distance(point1, point2) };
        assert_eq!(distance, 9.0);
    }

    #[test]
    fn it_handles_errors() {
        initialize();
        let wkt1 = CString::new("POINT(1, 1)").expect("CString::new failed");
        let _point1 = unsafe { sfcgal_io_read_wkt(wkt1.as_ptr(), 10) };
        let message = unsafe { w_sfcgal_get_last_error() };
        assert_eq!(
            _string(message),
            String::from("WKT parse error, Coordinate dimension < 2 (, 1)"),
        );
    }

}
