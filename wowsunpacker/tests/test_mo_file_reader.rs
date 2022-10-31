#[cfg(test)]
mod test_mo_file_reader {
    use wowsunpacker::{
        game::GameLanguages,
        unpacker::{GameUnpacker, LangUnpacker},
    };

    #[test]
    fn read_japanese_mo() {
        let unpacker = GameUnpacker::auto(r"C:\Games\World_of_Warships").unwrap();
        let text_path = unpacker.get_lang_path(&GameLanguages::JA);
        assert!(text_path.contains("ja/LC_MESSAGES"));
        let reader = LangUnpacker::new(text_path);
        assert!(reader.is_ok());
        let mut reader = reader.unwrap();
        let result = reader.decode();
        assert!(result.is_ok());
        let result = reader.write_to_file("ja.json".to_string(), "output".to_string());
        assert!(result.is_ok());
    }
}
