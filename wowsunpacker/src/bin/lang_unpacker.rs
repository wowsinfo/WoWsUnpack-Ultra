use wowsunpacker::{
    game::{GameDirectory, GameLanguages, GameServer},
    logger::setup_default_logger,
    types::UnpackResult,
    unpacker::{GameUnpacker, LangUnpacker},
};

fn main() -> UnpackResult<()> {
    setup_default_logger();

    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(GameServer::WW)
        .ok_or("Failed to find World of Warships game directory")?
        .to_string();

    let unpacker = GameUnpacker::auto(&ww_dir)?;
    for lang in GameLanguages::values().iter() {
        println!("Unpacking language: {}", lang);
        let lang_dir = unpacker.get_lang_path(lang);
        LangUnpacker::new(lang_dir)?
            .decode()?
            .write_to_file(&lang.to_filename(), "output")?;
    }

    Ok(())
}
