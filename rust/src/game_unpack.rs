use flate2::bufread::DeflateDecoder;
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub type UnpackError = Box<dyn Error>;

// the index file header
const G_IDX_SIGNATURE: [u8; 4] = [0x49, 0x53, 0x46, 0x50];

const HEADER_SIZE: u32 = 56;
// The order should be exact for bincode to work
#[derive(Debug, Deserialize)]
struct IdxHeader {
    _first_block: [u8; 12],
    nodes: i32,
    files: i32,
    _unknown1: i64,
    _unknown2: i64,
    third_offset: i64,
    trailer_offset: i64,
}

impl IdxHeader {
    fn parse(data: &[u8]) -> Option<IdxHeader> {
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
    name: String,
    id: u64,
    parent: u64,
}

impl Node {
    /**
     * @brief Parse a node with its offset and get the name from full_data
     * @param data The data to parse
     * @param offset The offset of the node
     * @param full_data The full data of the file
     * @return The parsed node
     */
    fn parse(data: &[u8], offset: usize, full_data: &[u8]) -> Option<Node> {
        let data_size = data.len() as u32;
        if data_size != NODE_SIZE {
            error!("Invalid Node size {}", data_size);
            return None;
        }

        let pointer: u64 =
            bincode::deserialize(&data[8..16]).expect("Failed to deserialize string pointer");
        // the offset here is very important because the raw point address is incorrect
        let pointer = pointer + offset as u64;
        let full_data_size = full_data.len() as u64;
        if pointer >= full_data_size {
            error!(
                "String pointer {} outside data range {}",
                pointer, full_data_size
            );
            return None;
        }
        // print the pointer in its hex form
        debug!("String pointer 0x{:x}", pointer);

        let name =
            read_null_terminated_string(full_data, pointer as usize).unwrap_or("".to_string());
        debug!("Node name: {}", name);

        let id = bincode::deserialize(&data[16..24]).expect("Failed to deserialize node id");
        let parent =
            bincode::deserialize(&data[24..32]).expect("Failed to deserialize node parent");

        Some(Node { name, id, parent })
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
    // std::optional<FileRecord> FileRecord::Parse(std::span<u8> data, const std::unordered_map<uint64_t, Node>& nodes)
    fn parse(data: &[u8], nodes: &HashMap<u64, Node>) -> Option<FileRecord> {
        let data_size = data.len() as u32;
        if data_size != FILE_RECORD_SIZE {
            error!("Invalid FileRecord size {}", data_size);
            return None;
        }

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
    // nodes: HashMap<u64, Node>,
    files: HashMap<String, FileRecord>,
}

impl IdxFile {
    fn parse(data: &[u8]) -> Option<IdxFile> {
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

        let node_size = NODE_SIZE as usize;
        let header = header.unwrap();
        let header_nodes = header.nodes as usize;
        info!(
            "Parsed IdxHeader with {} nodes and {} files",
            header_nodes, header.files
        );
        let total_node_size = header_nodes * node_size;
        if data_size < total_node_size {
            error!(
                "Data too small for {} nodes, expected {} but got {}",
                header_nodes, total_node_size, data_size
            );
        }

        // parser the node
        let mut nodes: HashMap<u64, Node> = HashMap::new();
        for i in 0..header_nodes {
            // get the node data offset consider the header size
            let offset = header_size + i * node_size;
            let node_data = &data[offset..offset + node_size];
            let node = Node::parse(node_data, offset, data);
            if node.is_none() {
                // first few nodes are empty
                warn!("This node is invalid");
                continue;
            }
            let node = node.unwrap();
            debug!("Node: {:?}", node);
            nodes.insert(node.id, node);
        }
        assert_eq!(nodes.len(), header_nodes);

        // parse file records
        let third_offset = header.third_offset as usize + 0x10;
        if data_size < third_offset {
            error!(
                "File record data ({}) smaller than offset ({})",
                data_size, third_offset
            );
            return None;
        }

        let file_record_data = &data[third_offset..];
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
            let index = i * file_record_size;
            let file_record_data = &file_record_data[index..index + file_record_size];
            let file_record = FileRecord::parse(file_record_data, &nodes);
            if file_record.is_none() {
                error!("Failed to parse RawFileRecord");
                return None;
            }
            let file_record = file_record.unwrap();
            debug!("FileRecord: {:?}", file_record);
            files.insert(file_record.path.clone(), file_record);
        }
        assert_eq!(files.len(), header_files);

        // parse trailer
        let trailer_offset = header.trailer_offset as usize + 0x10;
        if data_size < trailer_offset {
            error!(
                "Trailer data ({}) smaller than offset ({})",
                data_size, trailer_offset
            );
            return None;
        }

        let trailer_data = &data[trailer_offset..];
        // the name starts from byte 24
        let trailer_data = &trailer_data[24..];
        let pkg_name = read_null_terminated_string(trailer_data, 0);
        if pkg_name.is_none() {
            error!("Failed to get file pkg name");
            return None;
        }

        let pkg_name = pkg_name.unwrap();
        debug!("PkgName: {}", pkg_name);

        return Some(IdxFile {
            pkg_name,
            // nodes,
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

    fn create_with(file: FileRecord) -> Self {
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
                TreeNode::create_with(file_record.clone()),
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
            TreeNode::create_with(file_record.clone()),
        );
    }
}

pub struct Unpacker {
    directory_tree: DirectoryTree,
    pkg_path: String,
}

impl Unpacker {
    pub fn new_auto(game_path: &str) -> Result<Self, UnpackError> {
        let pkg_path = Path::new(game_path).join("res_packages");
        if !pkg_path.exists() {
            return Err(Box::from("Failed to find res_packages directory"));
        }

        // need to find the latest index folder
        let bin_path = Path::new(game_path).join("bin");
        if !bin_path.exists() {
            return Err(Box::from("Failed to find bin directory"));
        }

        // filter out folders in bin_path
        let mut latest_folder = 0;
        for entry in std::fs::read_dir(bin_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path
                    .file_name()
                    .ok_or("Failed to get folder name")?
                    .to_str()
                    .ok_or("Failed to get folder name")?;
                if let Ok(folder_num) = folder_name.parse::<u32>() {
                    if folder_num > latest_folder {
                        latest_folder = folder_num;
                    }
                }
            }
        }

        let latest_folder = latest_folder.to_string();
        // game_path/bin/latest_folder
        let idx_path = Path::new(game_path)
            .join("bin")
            .join(latest_folder)
            .join("idx");
        let idx_path = idx_path.to_str().ok_or("Failed to get idx path")?;
        let pkg_path = pkg_path.to_str().ok_or("Failed to get pkg path")?;

        info!("Idx path: {}", idx_path);
        info!("Pkg path: {}", pkg_path);

        return Self::new(pkg_path, idx_path);
    }

    /**
     * Create a new Unpacker
     * @param pkg_path The path to the pkg file
     * @param idx_file The path to the idx file
     */
    pub fn new(pkg_path: &str, idx_path: &str) -> Result<Self, UnpackError> {
        if !Path::new(idx_path).exists() {
            return Err(Box::from("IdxPath does not exist"));
        }

        // pkg_path needs to have res_package in the string
        if !pkg_path.contains("res_packages") {
            return Err(Box::from("PkgPath does not contain res_package"));
        }

        let mut unpacker = Unpacker {
            directory_tree: DirectoryTree {
                root: TreeNode::new(),
            },
            pkg_path: pkg_path.to_string(),
        };

        for entry in std::fs::read_dir(idx_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "idx" {
                let filename = path.clone();
                let filename = filename
                    .file_name()
                    .ok_or("Failed to get filename")?
                    .to_str()
                    .ok_or("Failed to convert filename to str")?;
                info!("Parsing idx file: {}", filename);

                // read with buffer to speed up
                let mut file = BufReader::new(File::open(path)?);
                let mut data = Vec::new();
                file.read_to_end(&mut data)?;

                let idx_file = IdxFile::parse(&data).ok_or("Failed to parse idx file")?;
                info!("Parsed idx file: {}", filename);
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

        Ok(unpacker)
    }

    pub fn extract(&self, node_name: &str, dest: &str) -> Result<(), UnpackError> {
        let node_result = self.directory_tree.find(node_name);
        if node_result.is_none() {
            warn!(
                "There exists no node with name {} in directory tree",
                node_name
            );
            return Ok(());
        }

        // extract the node
        let root_node = node_result.ok_or("Failed to find node")?;
        let mut stack = vec![root_node];
        while !stack.is_empty() {
            let node = stack.pop().ok_or("Failed to pop node from stack")?;
            for (_, child) in &node.nodes {
                stack.push(child);
            }

            // make sure the record is valid
            if node.file.is_none() {
                continue;
            }
            
            let file = node.file.as_ref().ok_or("Failed to get file record ref")?;
            self.extract_file(file, dest)?;
        }

        Ok(())
    }

    /**
     * Extract a file_record from the pkg file
     * @param file_record The file record
     * @param dest The destination path
     * @return true if success
     */
    fn extract_file(&self, file_record: &FileRecord, dest: &str) -> Result<(), UnpackError> {
        info!("Extracting record: {:?}", file_record);
        let pkg_file_path = Path::new(&self.pkg_path).join(&file_record.pkg_name);
        info!("Pkg file path: {}", pkg_file_path.display());
        let pkg_file = File::open(pkg_file_path)?;

        let pkg_file_size = pkg_file.metadata()?.len() as usize;
        let file_size = file_record.size as usize;
        let file_offset = file_record.offset as usize;
        let file_end_offset = file_offset + file_size;
        if file_end_offset > pkg_file_size {
            return Err(Box::from(format!(
                "Got offset ({} - {}) out of size bounds ({})",
                file_offset, file_end_offset, pkg_file_size
            )));
        }

        // remove the filename
        let out_dir = Path::new(dest).join(&file_record.path);
        let out_dir = out_dir.parent().ok_or("Failed to get parent dir")?;
        if !out_dir.exists() {
            std::fs::create_dir_all(out_dir)?;
            println!("Created directory: {}", out_dir.display());
        }
        info!("Extracting file: {}", file_record.path);

        // go to the file offset
        let mut pkg_reader = BufReader::new(&pkg_file);
        pkg_reader.seek(SeekFrom::Start(file_offset as u64))?;
        let mut raw_data = vec![0; file_size];
        pkg_reader.read_exact(&mut raw_data)?;

        // get the output path ready
        let file_path = Path::new(dest).join(&file_record.path);
        let file_path = file_path.to_str().ok_or("Failed to convert path to str")?;
        let file_uncompressed_size = file_record.uncompressed_size as usize;
        println!(
            "Unpacking file: {} ({}/{})",
            file_path, file_size, file_uncompressed_size
        );
        // decompress if necessary with zlib
        if file_size != file_uncompressed_size {
            let mut decompressed_data = vec![0; file_uncompressed_size];
            let mut decompressor = DeflateDecoder::new(raw_data.as_slice());
            decompressor.read(&mut decompressed_data).unwrap();

            if decompressed_data.len() != file_uncompressed_size {
                panic!(
                    "Decompressed size ({}) does not match expected size ({})",
                    decompressed_data.len(),
                    file_uncompressed_size
                );
            }
            return write_file_data(file_path, &decompressed_data);
        }

        return write_file_data(file_path, &raw_data);
    }
}

///
/// Helpers
///

fn write_file_data(file_name: &str, data: &[u8]) -> Result<(), UnpackError> {
    // write the data
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?
        .write_all(data)?;
    Ok(())
}

fn read_null_terminated_string(data: &[u8], offset: usize) -> Option<String> {
    let mut length = 0;
    let data_size = data.len();
    // stop until we find a null character
    for i in offset..data_size {
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
    return match take_string(string_data, length) {
        Some(s) => Some(s),
        None => {
            error!("Failed to read string");
            None
        }
    };
}

fn take_string(data: &[u8], size: usize) -> Option<String> {
    if data.len() >= size {
        return match String::from_utf8(data[0..size].to_vec()) {
            Ok(string) => Some(string),
            Err(e) => {
                error!("Failed to deserialize string - {}", e);
                None
            }
        };
    }

    return None;
}
