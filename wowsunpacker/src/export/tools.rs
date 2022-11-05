use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    game::{GameDirectory, GameLanguages, GameServer},
    types::UnpackResult,
    unpacker::{GameUnpacker, LangUnpacker, ParamsUnpacker},
};

pub fn unpack_languages(server: GameServer, dest: &str) -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(server)
        .ok_or("Failed to find World of Warships game directory")?;

    let unpacker = GameUnpacker::auto(&ww_dir)?;
    GameLanguages::values()
        .par_iter()
        .try_for_each(|lang| -> Option<()> {
            info!("Unpacking language: {}", lang);
            let lang_dir = unpacker.get_lang_path(lang);
            LangUnpacker::new(lang_dir)
                .ok()?
                .decode()
                .ok()?
                .write_to_file(&lang.to_filename(), dest)
                .ok()?;
            Some(())
        })
        .ok_or("Failed to unpack languages")?;

    Ok(())
}

pub fn unpack_game_data(server: GameServer, entries: &[&str], dest: &str) -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(server)
        .ok_or("Failed to find World of Warships game directory")?;

    let mut unpacker = GameUnpacker::auto(&ww_dir)?;
    unpacker.build_directory_tree()?;
    entries
        .par_iter()
        .try_for_each(|entry| {
            info!("Unpacking: {}", entry);
            unpacker.extract_exact(entry, dest).ok()?;
            Some(())
        })
        .ok_or("Failed to unpack game data")?;

    Ok(())
}

pub fn unpack_game_params(server: GameServer, dest: &str) -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(server)
        .ok_or("Failed to find World of Warships game directory")?;

    let game_params = "content/GameParams.data";
    GameUnpacker::auto(&ww_dir)?
        .build_directory_tree()?
        .extract_exact(game_params, dest)?;

    // call the params unpacker to get the json file
    let params_path = format!("{}/{}", dest, game_params);
    ParamsUnpacker::new()?.unpack(&params_path, false)?;
    Ok(())
}
