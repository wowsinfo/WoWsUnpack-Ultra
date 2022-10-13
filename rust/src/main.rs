extern crate log;
use env_logger::Env;
mod game_unpack;
use game_unpack::Unpacker;

fn main() {
    // allow all logs
    let env = Env::default().filter_or("RUST_LOG", "trace");
    env_logger::init_from_env(env);
    
    let unpacker = Unpacker::new(r"C:\Games\World_of_Warships\res_packages", r"C:\Games\World_of_Warships\bin\5771708\idx").expect("Failed to create unpacker");
    unpacker.extract("content/GameParams.data", "output");
}
