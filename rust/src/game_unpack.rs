use flate2::read::ZlibDecoder;
use log::{error, debug};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

///
/// GameFileUnpack.hpp
///

type Byte = u8;

const G_IDX_SIGNATURE: [Byte; 4] = [0x49, 0x53, 0x46, 0x50];
const G_SEP: char = '\\';

const HEADER_SIZE: u32 = 56;
// The order should be exact for bincode to work
#[derive(Debug, Deserialize)]
struct IdxHeader {
    first_block: [Byte; 12],
    nodes: i32,
    files: i32,
    unknown1: i64,
    unknown2: i64,
    third_offset: i64,
    trailer_offset: i64,
}

impl IdxHeader {
    fn parse(data: &[Byte]) -> Option<IdxHeader> {
        let data_size = data.len() as u32;
        if data_size != HEADER_SIZE {
            log::error!("Invalid IdxHeader size {}", data_size);
            return None;
        }

        let signature = &data[0..4];
        if signature != G_IDX_SIGNATURE {
            error!("Invalid Header signature");
            return None;
        }

        // drop the signature and decode to struct
        let data = &data[4..];
        let decoded =
            bincode::deserialize::<IdxHeader>(data).expect("Failed to deserialize IdxHeader");
        Some(decoded)
    }
}

const NODE_SIZE: u32 = 32;
#[derive(Debug)]
struct Node {
    unknown: [Byte; 8],
    name: String,
    id: u64,
    parent: u64,
}

impl Node {
    fn parse(data: &[Byte], full_data: &[Byte]) -> Option<Node> {
        let data_size = data.len() as u32;
        if data_size != NODE_SIZE {
            error!("Invalid Node size {}", data_size);
            return None;
        }

        let unknown: [Byte; 8] = data[0..8].try_into().unwrap();
        let pointer: u64 =
            bincode::deserialize(&data[8..16]).expect("Failed to deserialize string pointer");

        let full_data_size = full_data.len() as u64;
        if pointer >= full_data_size {
            error!(
                "String pointer {} outside data range {}",
                pointer, full_data_size
            );
            return None;
        }
        debug!("String pointer {}", pointer);

        let name = read_null_terminated_string(full_data, pointer as usize)
            .expect("Failed to get node name");
        debug!("Node name: {}", name);

        let id = bincode::deserialize(&data[16..24]).expect("Failed to deserialize node id");
        let parent =
            bincode::deserialize(&data[24..32]).expect("Failed to deserialize node parent");

        Some(Node {
            unknown,
            name,
            id,
            parent,
        })
    }
}

const FILE_RECORD_SIZE: u32 = 48;
#[derive(Debug, Clone)]
struct FileRecord {
    pkg_name: String,
    path: String,
    id: u64,
    offset: i64,
    size: i32,
    uncompressed_size: i64,
}

impl FileRecord {
    // std::optional<FileRecord> FileRecord::Parse(std::span<Byte> data, const std::unordered_map<uint64_t, Node>& nodes)
    fn parse(data: &[Byte], nodes: &HashMap<u64, Node>) -> Option<FileRecord> {
        let data_size = data.len() as u32;
        if data_size != FILE_RECORD_SIZE {
            error!("Invalid FileRecord size {}", data_size);
            return None;
        }
        // FileRecord fileRecord;

        // if (!TakeInto(data, fileRecord.Id))
        //     return {};

        // Take(data, 8);

        // if (!TakeInto(data, fileRecord.Offset))
        //     return {};

        // Take(data, 8);

        // if (!TakeInto(data, fileRecord.Size))
        //     return {};

        // Take(data, 4);

        // if (!TakeInto(data, fileRecord.UncompressedSize))
        //     return {};

        // std::vector<std::string_view> paths;
        // uint64_t current = fileRecord.Id;

        // while (nodes.contains(current))
        // {
        //     const Node& node = nodes.at(current);
        //     current = node.Parent;
        //     paths.emplace_back(node.Name);
        // }

        // std::ranges::reverse(paths);
        // fileRecord.Path = Core::String::Join(paths, "/");

        // return fileRecord;

        let id = bincode::deserialize(&data[0..8]).expect("Failed to deserialize file id");
        let offset =
            bincode::deserialize(&data[16..24]).expect("Failed to deserialize file offset");
        let size = bincode::deserialize(&data[32..36]).expect("Failed to deserialize file size");
        let uncompressed_size = bincode::deserialize(&data[40..48])
            .expect("Failed to deserialize file uncompressed size");

        let mut paths = Vec::new();
        let mut current = id;
        while nodes.contains_key(&current) {
            let node = nodes.get(&current).unwrap();
            current = node.parent;
            paths.push(node.name.as_str());
        }

        paths.reverse();
        let path = paths.join("/");

        return Some(FileRecord {
            pkg_name: "".to_string(),
            path,
            id,
            offset,
            size,
            uncompressed_size,
        });
    }
}

struct IdxFile {
    pkg_name: String,
    nodes: HashMap<u64, Node>,
    files: HashMap<String, FileRecord>,
}

impl IdxFile {
    fn parse(data: &[Byte]) -> Option<IdxFile> {
        let header_size = HEADER_SIZE as usize;
        let data_size = data.len() as usize;
        if data_size < header_size {
            error!("Invalid IdxFile size {}", data_size);
            return None;
        }

        // parse the header
        let header_data = &data[0..header_size];
        let header = IdxHeader::parse(header_data);
        if header.is_none() {
            error!("Failed to parse IdxHeader");
            return None;
        }

        let mut nodes: HashMap<u64, Node> = HashMap::new();

        // parser the node
        let node_size = NODE_SIZE as usize;
        let header = header.unwrap();
        let total_node_size = header.nodes as usize * node_size;
        if data_size < total_node_size {
            error!(
                "Data too small for {} nodes, expected {} but got {}",
                header.nodes, total_node_size, data_size
            );
        }

        for i in 0..header.nodes {
            let index = i as usize;
            // get the node data offset
            let node_data =
                &data[header_size + index * node_size..header_size + (index + 1) * node_size];
            let node = Node::parse(node_data, data);
            if node.is_none() {
                error!("Failed to parse Node");
                return None;
            }
            let node = node.unwrap();
            debug!("Node: {:?}", node);
            nodes.insert(node.id, node);
        }

        // parse file records
        let third_offset = header.third_offset as usize + 0x10;
        if data_size < third_offset {
            error!(
                "File record data ({}) smaller than offset ({})",
                data_size,
                header.third_offset + 0x10
            );
            return None;
        }

        let file_record_data = &data[third_offset..data_size];
        let file_record_size = FILE_RECORD_SIZE as usize;
        let header_files = header.files as usize;
        let total_file_record_size = header_files * file_record_size;
        if file_record_data.len() < total_file_record_size {
            error!(
                "File record too small for {} RawFileRecords, expected at least {} bytes but only got {}",
                header_files, total_file_record_size, file_record_data.len()
            );
            return None;
        }

        let mut files: HashMap<String, FileRecord> = HashMap::new();
        for i in 0..header_files {
            let index = i as usize;
            let file_record_data =
                &file_record_data[index * file_record_size..(index + 1) * file_record_size];
            let file_record = FileRecord::parse(file_record_data, &nodes);
            if file_record.is_none() {
                error!("Failed to parse RawFileRecord");
                return None;
            }
            let file_record = file_record.unwrap();
            debug!("FileRecord: {:?}", file_record);
            files.insert(file_record.path.clone(), file_record);
        }

        // parse trailer
        let trailer_offset = header.trailer_offset as usize + 0x10;
        if data_size < trailer_offset {
            error!(
                "Trailer data ({}) smaller than offset ({})",
                data_size, trailer_offset
            );
            return None;
        }

        let trailer_data = &data[trailer_offset..data_size];
        let offset = 8 + 8 + 8;
        let pkg_name = read_null_terminated_string(trailer_data, offset);
        if pkg_name.is_none() {
            error!("Failed to get file pkg name");
            return None;
        }

        let pkg_name = pkg_name.unwrap();
        debug!("PkgName: {}", pkg_name);

        return Some(IdxFile {
            pkg_name,
            nodes,
            files,
        });
    }
}

struct TreeNode {
    nodes: HashMap<String, TreeNode>,
    file: Option<FileRecord>,
}

impl TreeNode {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            file: None,
        }
    }

    fn createWith(file: FileRecord) -> Self {
        Self {
            nodes: HashMap::new(),
            file: Some(file),
        }
    }
}

struct DirectoryTree {
    root: TreeNode,
}

impl DirectoryTree {
    fn find(&self, path: &str) -> Option<&TreeNode> {
        let mut current = &self.root;
        for part in path.split("/") {
            if part.is_empty() {
                continue;
            }

            if let Some(node) = current.nodes.get(part) {
                current = node;
            } else {
                return None;
            }
        }
        Some(current)
    }

    fn insert(&mut self, file_record: &FileRecord) {
        if let Some(_) = file_record.path.rfind('/') {
            // under a directory
            self.create_path(file_record);
        } else {
            // under root
            self.root.nodes.insert(
                file_record.path.clone(),
                TreeNode::createWith(file_record.clone()),
            );
        }
    }

    /// Add the file record to the directory tree
    /// Create the path if it doesn't exist
    fn create_path(&mut self, file_record: &FileRecord) {
        let mut current = &mut self.root;
        for part in file_record.path.split("/") {
            if part.is_empty() {
                continue;
            }

            // Insert it if it doesn't exist
            if !current.nodes.contains_key(part) {
                let new_node = TreeNode::new();
                current.nodes.insert(part.to_string(), new_node);
            }
            current = current
                .nodes
                .get_mut(part)
                .expect("Failed to find the node");
        }

        current.nodes.insert(
            file_record.path.clone(),
            TreeNode::createWith(file_record.clone()),
        );
    }
}

pub struct Unpacker {
    directory_tree: DirectoryTree,
    pkg_path: String,
}

impl Unpacker {
    /**
     * Create a new Unpacker
     * @param pkg_path The path to the pkg file
     * @param idx_file The path to the idx file
     */
    pub fn new(pkg_path: &str, idx_path: &str) -> Option<Self> {
        if !Path::new(idx_path).exists() {
            error!("IdxPath does not exist: {}", idx_path);
            return None;
        }

        let mut unpacker = Unpacker {
            directory_tree: DirectoryTree {
                root: TreeNode::new(),
            },
            pkg_path: pkg_path.to_string(),
        };

        for entry in fs::read_dir(idx_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "idx" {
                // read with buffer to speed up
                let mut file = BufReader::new(File::open(path).unwrap());
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();

                let idx_file = IdxFile::parse(&data);
                if idx_file.is_none() {
                    error!("Failed to parse idxFile");
                    return None;
                }

                let idx_file = idx_file.unwrap();
                for (path, file_record) in idx_file.files {
                    unpacker.directory_tree.insert(&FileRecord {
                        pkg_name: idx_file.pkg_name.clone(),
                        path,
                        id: file_record.id,
                        offset: file_record.offset,
                        size: file_record.size,
                        uncompressed_size: file_record.uncompressed_size,
                    });
                }
            }
        }

        return Some(unpacker);
    }

    pub fn extract(&self, node_name: &str, dest: &str) -> bool {
        let node_result = self.directory_tree.find(node_name);
        if node_result.is_none() {
            error!(
                "There exists no node with name {} in directory tree",
                node_name
            );
            return false;
        }
        let root_node = node_result.unwrap();

        let mut stack = vec![root_node];

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            for (_, child) in &node.nodes {
                stack.push(child);
            }

            if node.file.is_some() {
                if !self.extract_file(&node.file.as_ref().unwrap(), dest) {
                    error!(
                        "Failed to extract file: {}",
                        node.file.as_ref().unwrap().path
                    );
                    return false;
                }
            }
        }

        return true;
    }

    pub fn exact_folder(&self, folder_name: &str, dest: &str) -> bool {
        let node_result = self.directory_tree.find(folder_name);
        if node_result.is_none() {
            error!(
                "There exists no node with name {} in directory tree",
                folder_name
            );
            return false;
        }
        let root_node = node_result.unwrap();

        let mut stack = vec![root_node];

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            for (_, child) in &node.nodes {
                stack.push(child);
            }

            if node.file.is_some() {
                if !self.extract_file(&node.file.as_ref().unwrap(), dest) {
                    error!(
                        "Failed to extract file: {}",
                        node.file.as_ref().unwrap().path
                    );
                    return false;
                }
            }
        }

        return true;
    }

    fn extract_file(&self, file_record: &FileRecord, dest: &str) -> bool {
        let pkg_file_path = Path::new(&self.pkg_path).join(&file_record.pkg_name);
        let pkg_file = File::open(pkg_file_path).unwrap();

        let file_size = pkg_file.metadata().unwrap().len() as usize;
        let file_record_size = file_record.size as usize;
        let file_record_offset = file_record.offset as usize;
        if file_record_offset + file_record_size > file_size {
            error!(
                "Got offset ({} - {}) out of size bounds ({})",
                file_record_offset,
                file_record_offset + file_record_size,
                file_size
            );
            return false;
        }

        // remove the filename
        let out_dir = Path::new(dest).join(&file_record.path);
        let out_dir = out_dir.parent().unwrap();
        if !out_dir.exists() {
            fs::create_dir_all(out_dir).unwrap();
        }

        let file_path = Path::new(dest).join(&file_record.path);
        let file_path = file_path.to_str().unwrap();
        let mut file = File::open(file_path).unwrap();
        file.seek(SeekFrom::Start(file_record_offset as u64))
            .unwrap();

        let mut data = vec![0; file_record.size as usize];
        file.read_exact(&mut data).unwrap();

        let file_uncompressed_size = file_record.uncompressed_size as usize;
        // check if data is compressed and decompress with zlib
        if file_record_size != file_uncompressed_size {
            let mut decoder = ZlibDecoder::new(data.as_slice());
            let mut inflated = Vec::new();
            decoder.read_to_end(&mut inflated).unwrap();
            return write_file_data(file_path, &inflated);
        }

        return write_file_data(file_path, &data);
    }
}

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
    let data_size = data.len();
    for i in offset..data_size {
        if data[i as usize] == 0 {
            length = i - offset;
            break;
        }
    }

    if length == 0 {
        error!("Invalid String Length");
        return None;
    }

    let string_data = &data[offset..(offset + length)];
    return match take_string(string_data, length) {
        Some(s) => Some(s),
        None => {
            error!("Failed to read string");
            None
        }
    };
}

fn take_string(data: &[Byte], size: usize) -> Option<String> {
    if data.len() >= size {
        return match String::from_utf8(data[0..size].to_vec()) {
            Ok(string) => Some(string),
            Err(e) => {
                error!("Failed to deserialize string - {}", e);
                None
            }
        }
    }

    return None;
}

unsafe fn take_into<T>(data: &[Byte], dest: &mut T) -> bool {
    let size = std::mem::size_of::<T>();
    if data.len() >= size {
        std::ptr::copy_nonoverlapping(data.as_ptr(), dest as *mut T as *mut Byte, size);
        return true;
    }
    return false;
}
