use std::ffi::CString;

use crate::{
    game::{GameDirectory, GameServer},
    tool::unpack_game_data as unpack_game_data_impl,
    tool::unpack_game_params as unpack_game_params_impl,
    tool::unpack_languages as unpack_languages_impl,
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

///
/// Game Directory & Game Server
///

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
 * @return A [GameDirectoryList] containing all available game directories
 */
#[no_mangle]
pub extern "C" fn get_all_game_directories() -> *const GameDirectoryList {
    let dirs = GameDirectory::available_path();
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

/**
 * Get the first available game directory
 * @return The first available game directory, or NULL if none are available
 */
#[no_mangle]
pub extern "C" fn get_first_game_directory() -> *const c_char {
    let dirs = GameDirectory::available_path();
    if dirs.is_empty() {
        return std::ptr::null();
    }

    // return C string
    let dir = dirs[0].clone();
    let str = CString::new(dir);
    if str.is_err() {
        return std::ptr::null();
    }

    str.unwrap().into_raw()
}

/**
 * Get all available game servers
 * @return A [GameServerList] containing all available game servers
 */
#[no_mangle]
pub extern "C" fn get_all_game_servers() -> *const GameServerList {
    let servers = GameDirectory::available_server();
    if servers.is_empty() {
        return std::ptr::null();
    }

    let count = servers.len() as c_int;
    let mut list = Vec::new();
    for server in servers {
        list.push(server as c_int);
    }

    let list = list.as_ptr() as *const c_int;
    let list = Box::new(GameServerList { list, count });
    Box::into_raw(list)
}

/**
 * Get the first available game server
 * @return The first available game server, or -1 if none are available
 */
#[no_mangle]
pub extern "C" fn get_first_game_server() -> c_int {
    let servers = GameDirectory::available_server();
    if servers.is_empty() {
        return -1;
    }

    servers[0] as c_int
}

///
/// Game Unpacker, Language and Params
///

/**
 * Extract all languages from the game data
 * @param server: The game server id
 * @param dest: The destination directory to extract to
 * @return 0 if successful, 1 if not
 */
#[no_mangle]
pub extern "C" fn unpack_languages(server: c_int, dest: *const c_char) -> c_int {
    let server = GameServer::from_number(server);
    let dest = unsafe { convert_cstring(dest) };
    if dest.is_err() {
        return 1;
    }

    let dest = dest.unwrap();
    let result = unpack_languages_impl(server, dest.as_str());
    return result.is_err() as c_int;
}

/**
 * Extract a list of entries/paths from the game data
 * @param server: The game server id
 * @param entries: The list of entries/paths to extract
 * @param size: The size of the entries list, Rust doesn't know the size otherwise
 * @param dest: The destination directory to extract to
 * @return 0 if successful, 1 if not
 */
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
    return result.is_err() as c_int;
}

#[no_mangle]
pub extern "C" fn unpack_game_params(server: c_int, dest: *const c_char) -> c_int {
    let server = GameServer::from_number(server);
    let dest = unsafe { convert_cstring(dest) };
    if dest.is_err() {
        return 1;
    }

    let dest = dest.unwrap();
    let result = unpack_game_params_impl(server, dest.as_str());
    return result.is_err() as c_int;
}

///
/// Free
///

/**
 * Free a C string allocated by Rust [CString]
 * @param ptr: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub unsafe extern "C" fn free_cstring(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }

    // freed when it goes out of scope
    let _ = CString::from_raw(ptr as *mut c_char);
}

/**
 * Free a [GameDirectoryList] allocated by Rust
 * @param ptr: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub unsafe extern "C" fn free_game_directory_list(ptr: *const GameDirectoryList) {
    if ptr.is_null() {
        return;
    }

    let list = Box::from_raw(ptr as *mut GameDirectoryList);
    let list = std::slice::from_raw_parts(list.list, list.count as usize);
    for ptr in list {
        free_cstring(*ptr);
    }
}

/**
 * Free a [GameServerList] allocated by Rust
 * @param ptr: The pointer to free
 * @return Nothing
 */
#[no_mangle]
pub unsafe extern "C" fn free_game_server_list(ptr: *const GameServerList) {
    if ptr.is_null() {
        return;
    }

    let list = Box::from_raw(ptr as *mut GameServerList);
    let _ = std::slice::from_raw_parts(list.list, list.count as usize);
}

///
/// Helper Functions
///

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
