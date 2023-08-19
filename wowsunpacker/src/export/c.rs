use std::ffi::CString;

use crate::{
    game::{GameDirectory, GameServer},
    tool::unpack_game_data as unpack_game_data_impl,
    types::UnpackResult,
};
use libc::{c_char, c_int};

/// C Structs

#[repr(C)]
pub struct GameDirectoryList {
    pub list: *const *const c_char,
    pub count: c_int,
}

#[repr(C)]
pub struct GameServerList {
    pub list: *const c_int,
    pub count: c_int,
}

/// Game Directory

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
    let str = CString::new(ww_dir);
    if str.is_err() {
        return std::ptr::null();
    }

    str.unwrap().into_raw()
}

/**
 * Get all available game directories
 * @return A list of C strings containing the game directories, or NULL if none are found
 */
#[no_mangle]
pub extern "C" fn get_all_game_directories() -> *const GameDirectoryList {
    let dirs = GameDirectory::auto();
    if dirs.is_empty() {
        return std::ptr::null();
    }

    let count = dirs.len() as c_int;
    let mut list = Vec::new();
    for dir in dirs {
        let dir = CString::new(dir);
        if dir.is_err() {
            return std::ptr::null();
        }
        list.push(dir.unwrap().into_raw());
    }
    
    let list = list.as_ptr() as *const *const c_char;
    let list = Box::new(GameDirectoryList { list, count });
    Box::into_raw(list)
}

/// Game Unpacker

#[no_mangle]
pub extern "C" fn unpack_game_data(
    server: c_int,
    entries: *const *const c_char,
    size: c_int,
    dest: *const c_char,
) -> c_int {
    let server = GameServer::from_number(server);
    let entries = unsafe { convert_cstring_list(entries, size) };
    if entries.is_err() {
        return 1;
    }

    let entries = entries.unwrap();
    let entries: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
    let dest = unsafe { convert_cstring(dest) };
    if dest.is_err() {
        return 1;
    }

    let dest = dest.unwrap();
    let result = unpack_game_data_impl(server, entries.as_slice(), dest.as_str());
    return  result.is_err() as c_int;;
}

/// Free

/**
 * Free a C string allocated by Rust
 * @param ptr: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr as *mut c_char);
    }
}

/**
 * Free a list of C strings allocated by Rust
 * @param list: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub extern "C" fn free_game_directory_list(ptr: *const GameDirectoryList) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let list = Box::from_raw(ptr as *mut GameDirectoryList);
        let list = std::slice::from_raw_parts_mut(list.list as *mut *mut c_char, list.count as usize);
        for ptr in list {
            free_cstring(*ptr);
        }
    }
}

/// Helper Functions

unsafe fn convert_cstring_list(
    list: *const *const c_char,
    size: c_int,
) -> UnpackResult<Vec<String>> {
    if list.is_null() {
        return Err("Null pointer".into());
    }

    if size <= 0 {
        return Err("Invalid size".into());
    }

    let mut result = Vec::new();
    for i in 0..size {
        let str = convert_cstring(*list.offset(i as isize))?;
        result.push(str);
    }
    Ok(result)
}

unsafe fn convert_cstring(ptr: *const c_char) -> UnpackResult<String> {
    if ptr.is_null() {
        return Err("Null pointer".into());
    }

    let s = CString::from_raw(ptr as *mut c_char);
    Ok(s.into_string()?)
}
