// See https://www.gnu.org/software/gettext/manual/html_node/MO-Files.html for the format specification

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use log::{debug, info, warn};
use serde::Deserialize;

use crate::types::UnpackResult;
use crate::utils::functions::read_null_terminated_string;

#[derive(Debug, Deserialize)]
struct MoHeader {
    magic: u32,
    _revision: u32,
    num_strings: u32,
    offset_originals: u32,
    offset_translations: u32,
    _table_size: u32,
    _table_offset: u32,
}

impl MoHeader {
    fn parse(data: &[u8]) -> Option<Self> {
        let decoded = bincode::deserialize::<MoHeader>(data).ok()?;
        if decoded.magic != 0xde120495 && decoded.magic != 0x950412de {
            return None;
        }

        Some(decoded)
    }
}

#[derive(Debug, Deserialize)]
struct MoEntry {
    length: u32,
    offset: u32,
}

pub struct LangUnpacker {
    file_path: String,
    text_data: HashMap<String, String>,
    decoded: bool,
}

impl LangUnpacker {
    pub fn new(file_path: String) -> UnpackResult<Self> {
        // validate the file exists
        if !Path::new(&file_path).exists() {
            return Err(Box::from(format!("File {} does not exist", file_path)));
        }

        Ok(Self {
            file_path,
            text_data: HashMap::new(),
            decoded: false,
        })
    }

    pub fn decode(&mut self) -> UnpackResult<()> {
        if self.decoded {
            warn!("Text data already decoded");
            return Ok(());
        }

        let mut file = File::open(&mut self.file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let header = MoHeader::parse(&data).ok_or("Invalid MO file")?;
        info!("{:?}", header);

        let text_data = &mut self.text_data;

        let key_offset = header.offset_originals;
        let value_offset = header.offset_translations;
        for entry in 0..header.num_strings {
            // get the key string
            let index = (entry * 8 + key_offset) as usize;
            let mo_entry = bincode::deserialize::<MoEntry>(&data[index..index + 8])?;
            debug!("{:?}", mo_entry);
            // the string can actually be empty
            let key_string = read_null_terminated_string(&data, mo_entry.offset as usize)
                .unwrap_or(String::from(""));
            // some string has null terminator in the middle so it is shorter than the expected length
            // we allow it here because because the actual string seems to be duplicated twice or more
            if key_string.len() > mo_entry.length as usize {
                panic!(
                    "Key string {} is longer than length {}",
                    key_string, mo_entry.length
                );
            }

            // get the translation value
            let index = (entry * 8 + value_offset) as usize;
            let mo_entry = bincode::deserialize::<MoEntry>(&data[index..index + 8])?;
            debug!("{:?}", mo_entry);
            let value_string = read_null_terminated_string(&data, mo_entry.offset as usize)
                .unwrap_or(String::from(""));
            if value_string.len() > mo_entry.length as usize {
                panic!(
                    "Value string {} is longer than length {}",
                    value_string, mo_entry.length
                );
            }

            text_data.insert(key_string, value_string);
        }

        self.decoded = true;
        Ok(())
    }

    pub fn write_to_file(&self, file_name: String, dest: String) -> UnpackResult<()> {
        if !self.decoded {
            return Err(Box::from(
                "Text data is not decoded yet, call decode() before writing",
            ));
        }

        let file_path = Path::new(&dest).join(file_name);
        let mut file = File::create(file_path)?;
        // encode it to json
        let json = serde_json::to_string(&self.text_data)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
