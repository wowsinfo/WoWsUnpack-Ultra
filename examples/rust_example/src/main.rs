use wowsunpacker::{
    game_directory::{GameDirectory, GameServer},
    game_unpack::Unpacker,
};

fn main() {
    let mut games = GameDirectory::new();
    games.locate();
    let game_dirtory = games.get_game_directory(GameServer::WW).unwrap();
    let unpacker = Unpacker::new_auto(game_dirtory).unwrap();
    unpacker
        .extract_exact("content/GameParams.data", "output")
        .unwrap();
}
