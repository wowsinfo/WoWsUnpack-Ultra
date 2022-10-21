use log::{error, warn};


pub fn read_null_terminated_string(data: &[u8], offset: usize) -> Option<String> {
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
    if string_data.len() < length {
        warn!("String data is smaller than expected");
        return None;
    }

    match String::from_utf8(string_data[0..length].to_vec()) {
        Ok(string) => Some(string),
        Err(e) => {
            error!("Failed to deserialize string - {}", e);
            None
        }
    }
}
