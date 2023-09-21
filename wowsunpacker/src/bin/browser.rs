use std::io::{stdin, stdout, Write};

use wowsunpacker::{
    browser::DirectoryBrowser, game::GameDirectory, logger::setup_logger, types::UnpackResult,
    unpacker::GameUnpacker,
};

fn _print_browser_info(browser: &DirectoryBrowser) {
    println!("Files: {:?}", browser.file_list());
    println!("Directories: {:?}", browser.directory_list());
}

fn _test_browser() -> UnpackResult<()> {
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
    _print_browser_info(&browser);
    browser.unpack("test4k_2x.png", "output")?;
    browser.go_back();
    browser.unpack("4k/test4k.png", "output")?;
    browser.navigate_to("bg");

    // let's try invalid path
    browser.reset();
    browser.navigate_to("gui");
    browser.navigate_to("fghobqiua");
    _print_browser_info(&browser);

    // try a folder with both files and directories
    browser.go_back();
    browser.navigate_to("dogTags");
    browser.unpack_current("output")?;
    _print_browser_info(&browser);
    Ok(())
}

fn prompt() -> UnpackResult<()> {
    print!("> ");
    stdout().flush()?;
    Ok(())
}

fn main() -> UnpackResult<()> {
    setup_logger("warn", "warn");

    // _test_browser();

    // get input until user types "> exit"
    println!("Supported commands:");
    println!("- cd <path>\t\t\tNavigate to a directory");
    println!("- ls [file|directory]\t\tList files or directories");
    println!("- extract <path> <output>\tExtract a file or directory");
    println!("- exit\t\t\t\tExit the program\n");

    // Get the first available game directory
    print!("Locating game directory from");
    let directory = GameDirectory::available_path();
    let first_directory = directory.first().expect("No game directory found");
    let mut unpacker = GameUnpacker::auto(first_directory)?;
    println!(" {}...", first_directory);
    unpacker.build_directory_tree()?;
    let mut browser = DirectoryBrowser::new(&unpacker);

    // flush stdout
    prompt()?;
    let mut user_input = String::new();
    while let Ok(_) = stdin().read_line(&mut user_input) {
        match user_input.trim() {
            "exit" => break,
            _ => {
                let mut args = user_input.trim().split(' ');
                match args.next() {
                    Some("cd") => {
                        if let Some(path) = args.next() {
                            match path {
                                ".." => browser.go_back(),
                                "." => &browser,
                                _ => browser.navigate_to(path),
                            };

                            if browser.validate_current().is_err() {
                                println!("Invalid path: {}", path);
                                browser.go_back();
                                println!("{:?}", browser.directory_list());
                            }
                        }
                    }
                    Some("ls") => {
                        if let Some(arg) = args.next() {
                            match arg {
                                "file" => println!("{:?}", browser.file_list()),
                                "directory" => println!("{:?}", browser.directory_list()),
                                _ => println!("Invalid argument"),
                            }
                        } else {
                            // Print both by default
                            println!("{:?}", browser.directory_list());
                            println!("{:?}", browser.file_list());
                        }
                    }
                    Some("extract") => {
                        if let Some(path) = args.next() {
                            let mut output = "output";
                            if let Some(user_output) = args.next() {
                                output = user_output;
                            }
                            
                            let extract_result = match path {
                                "." => browser.unpack_current(output),
                                _ => browser.unpack(path, output),
                            };

                            if extract_result.is_err() {
                                println!("Unable to extract {}", path);
                            }
                        } else {
                            println!("Path not specified");
                        }
                    }
                    Some(command) => println!("Invalid command: {}", command),
                    None => println!("Invalid command"),
                }
            }
        }

        user_input.clear();
        prompt()?;
    }

    Ok(())
}
