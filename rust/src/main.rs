extern crate log;

mod game_unpack;
use game_unpack::Unpacker;

fn main() {
    env_logger::init();
    
    let unpacker = Unpacker::new(r"C:\Games\World_of_Warships\res_packages", r"C:\Games\World_of_Warships\bin\5771708\idx").expect("Failed to create unpacker");
    unpacker.extract("content/GameParams.data", "output");
}
