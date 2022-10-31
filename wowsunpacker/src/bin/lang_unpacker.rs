use wowsunpacker::{
    game::{GameDirectory, GameLanguages, GameServer},
    logger::setup_logger,
    unpacker::{GameUnpacker, LangUnpacker},
};

fn main() {
    setup_logger();
    let mut game_dir = GameDirectory::new();
    game_dir.locate();
    let ww_dir = game_dir.get_game_directory(GameServer::WW);
    if ww_dir.is_none() {
        panic!("Failed to find World of Warships game directory");
    }

    let ww_dir = ww_dir.unwrap();
    let unpacker = GameUnpacker::auto(ww_dir).unwrap();

    for lang in GameLanguages::values().iter() {
        println!("Unpacking language: {}", lang);
        let lang_dir = unpacker.get_lang_path(lang);
        let mut reader = LangUnpacker::new(lang_dir).unwrap();
        reader.decode().unwrap();
        reader
            .write_to_file(lang.to_filename(), "output".to_string())
            .unwrap();
    }
}
