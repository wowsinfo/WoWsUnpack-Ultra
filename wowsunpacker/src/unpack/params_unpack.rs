use crate::types::UnpackResult;
use std::ffi::{c_char, c_int, CString};

pub struct ParamsUnpacker {
    lib: libloading::Library,
}

impl ParamsUnpacker {
    pub fn new() -> UnpackResult<Self> {
        let lib = unsafe { libloading::Library::new("HenryQuan.WoWsUnpack.dll") }?;
        Ok(Self { lib })
    }

    pub fn unpack(&self, path: &str, compact: bool) -> UnpackResult<()> {
        unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn(*const c_char, c_int)> =
                self.lib.get(b"unpack")?;
            // rust string doesn't contain the null terminator
            let cpath = CString::new(path)?;
            Ok(func(cpath.as_ptr(), compact as c_int))
        }
    }
}

#[test]
fn test_params_unpack() {
    let unpacker = ParamsUnpacker::new().unwrap();
    unpacker
        .unpack("../../output/content/GameParams.data", false)
        .unwrap();
}
