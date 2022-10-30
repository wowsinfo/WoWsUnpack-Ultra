use wowsunpacker::game_unpack::Unpacker;
use wowsunpacker::game_directory::{GameDirectory, GameServer};

fn main() {
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
