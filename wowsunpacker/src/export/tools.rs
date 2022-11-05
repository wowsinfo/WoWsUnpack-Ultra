use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    game::{GameDirectory, GameLanguages, GameServer},
    types::UnpackResult,
    unpacker::{GameUnpacker, LangUnpacker},
};

pub fn unpack_languages(server: GameServer, dest: &str) -> UnpackResult<()> {
    let ww_dir = GameDirectory::new()
        .locate()
        .get_game_directory(server)
        .ok_or("Failed to find World of Warships game directory")?;

    let unpacker = GameUnpacker::auto(&ww_dir)?;
    // for lang in GameLanguages::values().iter() {
    //     println!("Unpacking language: {}", lang);
    //     let lang_dir = unpacker.get_lang_path(lang);
    //     LangUnpacker::new(lang_dir)?
    //         .decode()?
    //         .write_to_file(&lang.to_filename(), dest)?;
    // }

    // unpack all languages in parallel
    // GameLanguages::values().par_iter().for_each(|lang| {
    //     info!("Unpacking language: {}", lang);
    //     let lang_dir = unpacker.get_lang_path(lang);
    //     match LangUnpacker::new(lang_dir) {
    //         Ok(mut unpacker) => {
    //             match unpacker.decode() {
    //                 Ok(unpacker) => {
    //                     match unpacker.write_to_file(&lang.to_filename(), dest) {
    //                         Ok(_) => info!("Unpacked language: {}", lang),
    //                         Err(e) => error!("Failed to write language file: {}", e),
    //                     }
    //                 },
    //                 Err(e) => error!("Failed to decode language file: {}", e),
    //             }
    //         },
    //         Err(e) => error!("Failed to create language unpacker: {}", e),
    //     }
    // });

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
