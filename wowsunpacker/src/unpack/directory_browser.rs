use crate::{types::UnpackResult, unpacker::GameUnpacker};

use super::game_unpack::TreeNode;

pub struct DirectoryBrowser<'a> {
    // a reference of GameUnpacker
    unpacker: &'a GameUnpacker,
    position: Vec<String>,
}

impl<'a> DirectoryBrowser<'a> {
    pub fn new(unpacker: &'a GameUnpacker) -> DirectoryBrowser<'a> {
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
    /// * `path` - The path to extract, use "/" as the separator, you can unpack a file or a directory, don't start the path with "/"
    /// # Returns
    /// * The unpacked file as a byte array
    pub fn unpack(&self, path: &str, dest: &str) -> UnpackResult<()> {
        if self.current_node().is_some() {
            let unpack_path = self.position.join("/") + "/" + path;
            self.unpacker.extract_exact(&unpack_path, dest)?;
        }

        Ok(())
    }

    /// Unpack the current directory or file
    pub fn unpack_current(&self, dest: &str) -> UnpackResult<()> {
        if self.current_node().is_some() {
            let unpack_path = self.position.join("/");
            self.unpacker.extract_exact(&unpack_path, dest)?;
        }

        Ok(())
    }

    pub fn validate_current(&self) -> UnpackResult<()> {
        if self.current_node().is_none() {
            return Err("Invalid path".into());
        }

        Ok(())
    }
}
