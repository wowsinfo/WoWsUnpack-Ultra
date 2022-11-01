use wowsunpacker::{
    game::{GameDirectory, GameServer},
    types::UnpackResult,
    unpacker::GameUnpacker,
};

fn main() -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .info()
        .get_game_directory(GameServer::WW)
        .ok_or("Failed to find World of Warships game directory")?
        .to_string();

    GameUnpacker::auto(&ww_dir)?
        .build_directory_tree()?
        .extract_exact("gui/dogTags/medium/", "output")?
        .extract_exact("gui/4k/", "output")?
        .extract_exact("content/GameParams.data", "output")?;

    Ok(())
}
