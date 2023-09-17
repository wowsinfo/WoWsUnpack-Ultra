use crate::unpacker::GameUnpacker;

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

    pub fn goto(&mut self, path: &str) -> &Self {
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

    pub fn list_files(&self) -> Vec<String> {
        return Vec::new();
    }

    pub fn list_directories(&self) -> Vec<String> {
        return Vec::new();
    }

    /// Unpack the file based on the current position
    /// # Arguments
    /// * `path` - The path to extract, separator is always `/`
    /// # Returns
    /// * The unpacked file as a byte array
    pub fn unpack_file(&self, path: &str) -> Vec<u8> {
        return Vec::new();
    }

    /// Split the path into a vector of strings
    fn split_path(&self, path: &str) -> Vec<String> {
        return Vec::new();
    }
}
