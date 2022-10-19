extern crate log;
use env_logger::Env;
mod game_unpack;
use game_unpack::Unpacker;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    }

    let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
    unpacker.extract("gui/dogTags/medium/", "output").unwrap();
    unpacker.extract("gui/4k/", "output").unwrap();
    unpacker
        .extract("content/GameParams.data", "output")
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::game_unpack::Unpacker;

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
        let result = unpacker.extract("gui/4k/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract("content/GameParams.data", "output");
        assert!(result.is_ok());
        let result = unpacker.extract("gui/dogTags", "output");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unpacker_new_auto() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships_PT");
        assert!(unpacker.is_ok());
        let unpacker = unpacker.unwrap();
        let result = unpacker.extract("gui/4k/", "output");
        assert!(result.is_ok());
        let result = unpacker.extract("content/GameParams.data", "output");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unpacker_auto_search() {
        let unpacker = Unpacker::new_auto(r"C:\Games\World_of_Warships").unwrap();
        let results = unpacker.search("gui*", false);
        assert!(results.len() > 0);
    }
}
