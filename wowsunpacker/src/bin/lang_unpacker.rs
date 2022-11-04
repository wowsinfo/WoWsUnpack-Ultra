use wowsunpacker::{
    game::GameServer, logger::setup_default_logger, tool::unpack_languages, types::UnpackResult,
};

fn main() -> UnpackResult<()> {
    setup_default_logger();
    unpack_languages(GameServer::WW, "output")?;
    Ok(())
}
