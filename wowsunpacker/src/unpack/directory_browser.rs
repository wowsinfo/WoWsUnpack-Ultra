use crate::unpacker::GameUnpacker;

use super::game_unpack::TreeNode;

pub struct DirectoryBrowser<'a> {
    // a reference of GameUnpacker
    unpacker: &'a GameUnpacker,
    position: Vec<String>,
}

impl DirectoryBrowser<'_> {
    pub fn new(unpacker: &'_ GameUnpacker) -> DirectoryBrowser<'_> {
        return DirectoryBrowser {
            unpacker: unpacker,
            position: Vec::new(),
        };
    }

    pub fn navigate_to(&mut self, path: &str) -> &Self {
        self.position.push(path.to_string());
        self
    }

    pub fn go_back(&mut self) -> &Self {
        self.position.pop();
        self
    }

    pub fn reset(&mut self) -> &Self {
        self.position.clear();
        self
    }

    fn current_node(&self) -> Option<&TreeNode> {
        return self.unpacker.directory_tree.goto(&self.position);
    }

    pub fn file_list(&self) -> Vec<&String> {
        match self.current_node() {
            None => Vec::new(),
            Some(node) => node.files().collect(),
        }
    }

    pub fn directory_list(&self) -> Vec<&String> {
        match self.current_node() {
            None => Vec::new(),
            Some(node) => node.directories().collect(),
        }
    }

    /// Unpack the file based on the current position
    /// # Arguments
    /// * `path` - The path to extract, separator is always `/`
    /// # Returns
    /// * The unpacked file as a byte array
    pub fn unpack_file(&self, path: &str) {
        match self.current_node() {
            Some(node) => {
                // TODO: 
                // ignore the parameter and unpack the current file
                if node.is_file() {
                    // self.unpacker.extract_exact(node_name, dest)
                } else {
                }
            }
            None => return,
        }
    }

    /// Split the path into a vector of strings
    fn split_path(&self, path: &str) -> Vec<String> {
        return Vec::new();
    }
}
