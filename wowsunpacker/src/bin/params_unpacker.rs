use wowsunpacker::{
    game::GameServer, logger::setup_default_logger, tool::unpack_game_params, types::UnpackResult,
};

fn main() -> UnpackResult<()> {
    // run this from /target/debug, DLLs are placed there, cargo run doesn't work
    setup_default_logger();
    unpack_game_params(GameServer::WW, "output")?;
    Ok(())
}
