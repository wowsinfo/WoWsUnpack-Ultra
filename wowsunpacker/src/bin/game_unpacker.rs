use wowsunpacker::{
    game::GameServer, logger::setup_default_logger, tool::unpack_game_data, types::UnpackResult,
};

fn main() -> UnpackResult<()> {
    setup_default_logger();
    unpack_game_data(
        GameServer::WW,
        &["gui/dogTags/medium/", "gui/4k/", "content/GameParams.data"],
        "output",
    )?;
    Ok(())
}
