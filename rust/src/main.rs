mod game_unpack;
mod helper;
mod text_unpack;

use game_unpack::Unpacker;
extern crate log;
use env_logger::Env;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    }

    let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
    unpacker
        .extract_exact("gui/dogTags/medium/", "output")
        .unwrap();
    unpacker.extract_exact("gui/4k/", "output").unwrap();
    unpacker
        .extract_exact("content/GameParams.data", "output")
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::game_unpack::{GameLanguages, Unpacker};
    use crate::text_unpack::MoFileReader;

    #[test]
    fn dummy() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_unpacker_new() {
        let unpacker = Unpacker::new(
            r"C:\Games\World_of_Warships\res_packages",
            r"C:\Games\World_of_Warships\bin\5771708\idx",
        );
        assert!(unpacker.is_ok());
        let unpacker = unpacker.unwrap();
        let result = unpacker.extract_exact("gui/4k/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract_exact("content/GameParams.data", "output");
        assert!(result.is_ok());
        let result = unpacker.extract_exact("gui/dogTags", "output");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unpacker_new_auto() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships_PT");
        assert!(unpacker.is_ok());
        let unpacker = unpacker.unwrap();
        let result = unpacker.extract_exact("gui/4k/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract_exact("content/GameParams.data", "output");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unpacker_auto_search() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
        let results = unpacker.search("gui*", false);
        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(results.len() > 0);
    }

    #[test]
    fn test_extract_fuzzy() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
        let result = unpacker.extract("gui/*ap*", "output");
        assert!(result.is_ok());
    }

    #[test]
    fn test_mo_file_reader() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
        let text_path = unpacker.get_text_file_path(GameLanguages::JA);
        assert!(text_path.contains("ja/LC_MESSAGES"));
        let reader = MoFileReader::new(text_path);
        assert!(reader.is_ok());
        let reader = reader.unwrap();
        let result = reader.write_to_file("ja.json".to_string(), "output".to_string());
        assert!(result.is_ok());
    }
}
