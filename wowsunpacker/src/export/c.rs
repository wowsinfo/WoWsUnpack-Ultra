use std::ffi::CString;

use crate::game::{GameDirectory, GameServer};
use libc::{c_char, c_int};

/**
 * Get the game directory for a given server
 * @param server: The server to get the game directory for
 * @return A C string containing the game directory, or NULL if the server is not found
 */
#[no_mangle]
pub extern "C" fn get_game_directory(server: c_int) -> *const c_char {
    let server = GameServer::from_number(server);
    let ww_dir = GameDirectory::new().locate().get_game_directory(&server);
    if ww_dir.is_none() {
        return std::ptr::null();
    }

    // return C string
    let ww_dir = ww_dir.unwrap();
    CString::new(ww_dir).unwrap().into_raw()
}

/**
 * Get all available game directories
 * @return A list of C strings containing the game directories, or NULL if none are found
 */
#[no_mangle]
pub extern "C" fn get_all_game_directories() -> *const *const c_char {
    let dirs = GameDirectory::auto();
    if dirs.is_empty() {
        return std::ptr::null();
    }

    let mut list = Vec::new();
    for dir in dirs {
        if dir.is_none() {
            list.push(CString::new("").unwrap().into_raw());
            continue;
        }

        let dir = dir.unwrap();
        list.push(CString::new(dir).unwrap().into_raw());
    }
    Box::into_raw(list.into_boxed_slice()) as *const *const c_char
}

/**
 * Free a C string allocated by Rust
 * @param ptr: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *const c_char) {
    unsafe {
        if ptr.is_null() {
            return;
        }

        let _ = CString::from_raw(ptr as *mut c_char);
    }
}

/**
 * Free a list of C strings allocated by Rust
 * @param list: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub extern "C" fn free_cstring_list(list: *const *const c_char) {
    if list.is_null() {
        return;
    }

// TODO: this needs to be improved, not all lists are 3 items long
    let list = unsafe { Vec::from_raw_parts(list as *mut *const c_char, 3, 3) };
    for s in list.iter() {
        unsafe {
            let a = CString::from_raw(*s as *mut c_char);
            println!("{:?}", a)
        }
    }
}
