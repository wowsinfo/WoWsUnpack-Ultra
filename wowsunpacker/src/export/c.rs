use crate::game::{GameDirectory, GameServer};
use libc::{c_char, malloc};

/// Get a list of game directories. null is returned if that game server is not found.
#[no_mangle]
pub extern "C" fn get_game_directory(server: i32) -> *const c_char {
    let server = GameServer::from(server);
    if server.is_none() {
        return std::ptr::null();
    }

    let server = server.unwrap();
    let ww_dir = GameDirectory::new().locate().get_game_directory(&server);
    if ww_dir.is_none() {
        return std::ptr::null();
    }

    let ww_dir = ww_dir.unwrap();
    // TODO: is this needed or we can simply use CString instead?
    // create a c string using malloc
    unsafe {
        let length = ww_dir.len();
        let char_pointer = malloc(length + 1) as *mut c_char;
        // write ww_dir to the pointer
        std::ptr::copy_nonoverlapping(ww_dir.as_ptr() as *const c_char, char_pointer, length);
        // add a null terminator
        *(char_pointer as *mut c_char).add(length) = 0;
        char_pointer
    }
}

// let game_dir = GameServer::values()
// .iter()
// .map(|server| ww_dir.get_game_directory(server).unwrap_or_default())
// .collect::<Vec<_>>();
// let mut result: Vec<*const c_char> = Vec::new();
// for dir in game_dir {
// result.push(dir.as_ptr() as *const c_char);
// }
// result.as_mut_ptr()
