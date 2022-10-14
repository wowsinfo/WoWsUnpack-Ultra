extern crate log;
use env_logger::Env;
mod game_unpack;
use game_unpack::Unpacker;

fn main() {
    // allow all logs only in debug mode
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
    }
    
    let unpacker = Unpacker::new(r"C:\Games\World_of_Warships\res_packages", r"C:\Games\World_of_Warships\bin\5771708\idx").expect("Failed to create unpacker");
    let success = unpacker.extract("content/GameParams.data", "output");
    if success {
        println!("Success!");
    } else {
        println!("Failed!");
    }
}
