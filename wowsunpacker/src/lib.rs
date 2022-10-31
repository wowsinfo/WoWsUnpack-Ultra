extern crate log;
mod unpack;
mod utils;

// only export whatever needed here
pub mod types;

// allow users to show more info in debug mode
pub mod logger {
    extern crate log;
    use env_logger::Env;

    pub fn setup_logger() {
        if cfg!(debug_assertions) {
            env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
        } else {
            env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();
        }
    }
}

pub mod unpacker {
    pub use crate::unpack::game_unpack::GameUnpacker;
    pub use crate::unpack::lang_unpack::LangUnpacker;
    pub use crate::unpack::params_unpack::ParamsUnpacker;
}

pub mod game {
    pub use crate::utils::game::{GameDirectory, GameLanguages, GameServer};
}
