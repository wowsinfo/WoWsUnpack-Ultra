use crate::types::UnpackResult;
use std::ffi::{c_char, c_int, CString};

#[allow(non_snake_case)]
pub fn ParamsUnpack(path: &str, compact: bool) -> UnpackResult<()> {
    unsafe {
        let lib = libloading::Library::new("HenryQuan.WoWsUnpack.dll")?;
        let func: libloading::Symbol<unsafe extern "C" fn(*const c_char, c_int)> =
            lib.get(b"unpack")?;
        // rust string doesn't contain the null terminator
        let cpath = CString::new(path)?;
        Ok(func(cpath.as_ptr(), compact as c_int))
    }
}
