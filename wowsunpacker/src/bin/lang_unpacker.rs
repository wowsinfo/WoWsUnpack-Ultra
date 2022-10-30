use wowsunpacker::{
    game_directory::{GameDirectory, GameServer},
    game_unpack::Unpacker,
    text_unpack::{GameLanguages, MoFileReader},
};

fn main() {
    let mut game_dir = GameDirectory::new();
    game_dir.locate();
    let ww_dir = game_dir.get_game_directory(GameServer::WW);
    if ww_dir.is_none() {
        panic!("Failed to find World of Warships game directory");
    }

    let ww_dir = ww_dir.unwrap();
    let unpacker = Unpacker::new_auto(ww_dir).unwrap();

    for lang in GameLanguages::values().iter() {
        println!("Unpacking language: {}", lang);
        let lang_dir = unpacker.get_text_file_path(lang);
        let reader = MoFileReader::new(lang_dir).unwrap();
        reader
            .write_to_file(lang.to_filename(), "output".to_string())
            .unwrap();
    }
}
