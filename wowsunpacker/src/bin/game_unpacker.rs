use wowsunpacker::{
    game::GameServer, logger::setup_default_logger, tool::unpack_game_data, types::UnpackResult,
};

fn main() -> UnpackResult<()> {
    // let ww_dir = GameDirectory::new()
    //     .locate()
    //     .info()
    //     .get_game_directory(GameServer::WW)
    //     .ok_or("Failed to find World of Warships game directory")?
    //     .to_string();

    // GameUnpacker::auto(&ww_dir)?
    //     .build_directory_tree()?
    //     .extract_exact("gui/dogTags/medium/", "output")?
    //     .extract_exact("gui/4k/", "output")?
    //     .extract_exact("content/GameParams.data", "output")?;

    setup_default_logger();
    unpack_game_data(
        GameServer::WW,
        &["gui/dogTags/medium/", "gui/4k/", "content/GameParams.data"],
        "output",
    )?;
    Ok(())
}
