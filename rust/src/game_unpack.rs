use log::{error, info, warn};
use std::{array, vec};
use std::collections::HashMap;

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
