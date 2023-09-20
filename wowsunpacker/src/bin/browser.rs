use wowsunpacker::{
    browser::DirectoryBrowser, logger::setup_logger, types::UnpackResult, unpacker::GameUnpacker,
};

fn print_browser_info(browser: &DirectoryBrowser) {
    println!("Files: {:?}", browser.file_list());
    println!("Directories: {:?}", browser.directory_list());
}

fn main() -> UnpackResult<()> {
    setup_logger("off", "off");

    let folder_path = "gui";

    let mut unpacker = GameUnpacker::auto("C:\\Games\\World_of_Warships")?;
    unpacker.build_directory_tree()?;
    let mut browser = DirectoryBrowser::new(&unpacker);
    unpacker
        .directory_tree
        .find(folder_path)
        .expect(format!("{} not found", folder_path).as_str())
        .print_children(2);
    browser.navigate_to("gui");
    browser.navigate_to("4k");
    print_browser_info(&browser);
    browser.unpack("test4k_2x.png", "output")?;
    browser.go_back();
    browser.unpack("4k/test4k.png", "output")?;
    browser.navigate_to("bg");

    // let's try invalid path
    browser.reset();
    browser.navigate_to("gui");
    browser.navigate_to("fghobqiua");
    print_browser_info(&browser);

    // try a folder with both files and directories
    browser.go_back();
    browser.navigate_to("dogTags");
    browser.unpack_current("output")?;
    print_browser_info(&browser);

    Ok(())
}
