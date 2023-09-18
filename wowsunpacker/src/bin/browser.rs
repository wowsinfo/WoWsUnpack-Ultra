use log::info;
use wowsunpacker::{
    browser::DirectoryBrowser, logger::setup_default_logger, types::UnpackResult,
    unpacker::GameUnpacker,
};

fn main() -> UnpackResult<()> {
    setup_default_logger();

    let folder_path = "gui";

    let mut unpacker = GameUnpacker::auto("C:\\Games\\World_of_Warships")?;
    unpacker.build_directory_tree()?;
    let mut browser = DirectoryBrowser::new(&unpacker);
    unpacker
        .directory_tree
        .find(folder_path)
        .expect(format!("{} not found", folder_path).as_str())
        .print_children(1);
    browser.goto("gui");
    browser.goto("4k");
    info!("Files: {:?}", browser.list_files());
    info!("Directories: {:?}", browser.list_directories());
    browser.unpack_file("test4k.png");
    browser.go_back();
    browser.goto("bg");

    // let's try invalid path
    browser.reset();
    browser.goto("gui");
    browser.goto("fghobqiua");
    info!("Files: {:?}", browser.list_files());
    info!("Directories: {:?}", browser.list_directories());
    Ok(())
}
