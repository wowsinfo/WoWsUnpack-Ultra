use wowsunpacker::{
    game::{GameDirectory, GameServer},
    unpacker::GameUnpacker,
};

fn main() {
    let mut game_dir = GameDirectory::new();
    game_dir.locate();
    game_dir.info();
    let ww_dir = game_dir.get_game_directory(GameServer::WW);
    if ww_dir.is_none() {
        panic!("Failed to find World of Warships game directory");
    }

    let ww_dir = ww_dir.unwrap();
    let mut unpacker = GameUnpacker::auto(ww_dir).unwrap();
    unpacker.build_directory_tree().unwrap();
    unpacker
        .extract_exact("gui/dogTags/medium/", "output")
        .unwrap();
    unpacker.extract_exact("gui/4k/", "output").unwrap();
    unpacker
        .extract_exact("content/GameParams.data", "output")
        .unwrap();
}
