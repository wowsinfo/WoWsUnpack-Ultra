use crate::{game::{GameServer, GameDirectory, GameLanguages}, types::UnpackResult, unpacker::{LangUnpacker, GameUnpacker}};


pub fn unpack_languages(server: GameServer, dest: &str) -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(server)
        .ok_or("Failed to find World of Warships game directory")?;

    let unpacker = GameUnpacker::auto(&ww_dir)?;
    for lang in GameLanguages::values().iter() {
        println!("Unpacking language: {}", lang);
        let lang_dir = unpacker.get_lang_path(lang);
        LangUnpacker::new(lang_dir)?
            .decode()?
            .write_to_file(&lang.to_filename(), dest)?;
    }

    Ok(())
}
