use wowsunpacker::{
    browser::DirectoryBrowser, logger::setup_default_logger, types::UnpackResult,
    unpacker::GameUnpacker,
};

fn main() -> UnpackResult<()> {
    setup_default_logger();
    let unpacker = GameUnpacker::auto("C:\\Games\\World_of_Warships")?;
    let browser = DirectoryBrowser::new(&unpacker);
    Ok(())
}
