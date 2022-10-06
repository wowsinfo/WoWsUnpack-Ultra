use log::{error, info, warn};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{array, vec};

///
/// GameFileUnpack.hpp
/// 

type Byte = u8;

const G_IDX_SIGNATURE: [Byte; 4] = [0x49, 0x53, 0x46, 0x50];

const HEADER_SIZE: u32 = 56;
struct IdxHeader {
    nodes: i32,
    files: i32,
    third_offset: i64,
    trailer_offset: i64,
    first_block: [Byte; 12],
    unknown1: i64,
    unknown2: i64,
}

const NODE_SIZE: u32 = 32;
struct Node {
    name: String,
    id: u64,
    parent: u64,
    unknown: [Byte; 8],
}

const FILE_RECORD_SIZE: u32 = 48;
struct FileRecord {
    pkg_name: String,
    path: String,
    id: u64,
    offset: i64,
    size: i32,
    uncompressed_size: i64,
}

struct IdxFile {
    pkg_name: String,
    nodes: HashMap<u64, Node>,
    files: HashMap<String, FileRecord>,
}

struct DirectoryTree {
    root: HashMap<String, DirectoryTree>,
}

struct TreeNode {
    nodes: HashMap<String, TreeNode>,
    file: Option<FileRecord>,
}

struct Unpacker {
    directory_tree: DirectoryTree,
    pkg_path: String,
}
///
/// GameFileUnpack.cpp
///

const G_SEP: char = '\\';

///
/// Helpers
/// 

fn write_file_data(file_name: &str, data: &[Byte]) -> bool {
    // write the data
    match OpenOptions::new().write(true).create(true).open(file_name) {
        Ok(mut file) => match file.write_all(data) {
            Ok(_) => true,
            Err(e) => {
                error!("Failed to write data to outfile {} - {}", file_name, e);
                false
            }
        },
        Err(e) => {
            error!("Failed to open file for writing {} - {}", file_name, e);
            false
        }
    }
}

fn read_null_terminated_string(data: &[Byte], offset: usize) -> Option<String> {
    let mut length = 0;

    for i in offset..data.len() {
        if data[i] == 0 {
            length = i - offset;
            break;
        }
    }

    if length == 0 {
        error!("Invalid String Length");
        return None;
    }

    let string_data = &data[offset..(offset + length)];
    let string = take_string(string_data, length);
    if string.is_none() {
        error!("Failed to read string");
        return None;
    }

    return string;
}

fn take_string(data: &[Byte], size: usize) -> Option<String> {
    if data.len() >= size {
        let string = String::from_utf8(data[0..size].to_vec());
        if string.is_err() {
            error!("Failed to convert string");
            return None;
        }
        return Some(string.unwrap());
    }
    return None;
}
