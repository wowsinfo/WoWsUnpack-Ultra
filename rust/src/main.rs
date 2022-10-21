extern crate log;
use env_logger::Env;

mod game_unpack;
use game_unpack::Unpacker;

mod helper;

mod text_unpack;

mod game_directory;
use game_directory::{GameDirectory, GameServer};

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    }

    let mut game_dir = GameDirectory::new();
    game_dir.locate();
    game_dir.info();
    let ww_dir = game_dir.get_game_directory(GameServer::WW);
    if ww_dir.is_none() {
        panic!("Failed to find World of Warships game directory");
    }

    let ww_dir = ww_dir.unwrap();
    let unpacker = Unpacker::new_auto(ww_dir).unwrap();
    unpacker.extract_exact("gui/dogTags/medium/", "output").unwrap();
    unpacker.extract_exact("gui/4k/", "output").unwrap();
    unpacker
        .extract_exact("content/GameParams.data", "output")
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::game_unpack::{GameLanguages, Unpacker};
    use crate::text_unpack::MoFileReader;
    use crate::game_directory::{GameDirectory, GameServer};

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

    #[test]
    fn find_game_directory() {
        let mut game_dir = GameDirectory::new();
        game_dir.locate();
        let ww_dir = game_dir.get_game_directory(GameServer::WW);
        assert!(ww_dir.is_some());
        game_dir.info();
    }

    #[test]
    fn test_unpack_with_auto_game_directory() {
        let mut game_dir = GameDirectory::new();
        game_dir.locate();
        let ww_dir = game_dir.get_game_directory(GameServer::WW);
        assert!(ww_dir.is_some());

        let ww_dir = ww_dir.unwrap();
        let unpacker = Unpacker::new_auto(ww_dir);
        assert!(unpacker.is_ok());

        let unpacker = unpacker.unwrap();
        let result = unpacker.extract("gui/dogTags/medium/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract("gui/4k/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract("content/GameParams.data", "output");
        assert!(result.is_ok());
    }
}
