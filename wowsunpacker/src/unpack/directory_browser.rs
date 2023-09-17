use crate::unpacker::GameUnpacker;

pub struct DirectoryBrowser {
    // a reference of GameUnpacker
    unpacker: &GameUnpacker,
}

impl DirectoryBrowser {
    pub fn new(unpacker: &GameUnpacker) -> DirectoryBrowser {
        DirectoryBrowser { unpacker }
    }
}
