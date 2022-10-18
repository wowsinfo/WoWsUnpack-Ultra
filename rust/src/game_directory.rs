extern crate winreg;
use log::{info, warn};
use std::{collections::HashMap, path::Path};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum GameServer {
    WW, // Global (ASIA, EU, NA, RU)
    CN, // The Chinese server
    PT, // The Public Test server
}

impl GameServer {
    fn values() -> Vec<GameServer> {
        vec![GameServer::WW, GameServer::CN, GameServer::PT]
    }

    fn get_registry_key(&self) -> &'static str {
        match self {
            GameServer::WW => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\WOWS.WW.PRODUCTION"
            }
            GameServer::CN => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\WOWS.CN.PRODUCTION"
            }
            GameServer::PT => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\WOWS.PT.PRODUCTION"
            }
        }
    }
}

pub struct GameDirectory {
    game_directory_info: HashMap<GameServer, String>,
}

impl GameDirectory {
    pub fn new() -> Self {
        Self {
            game_directory_info: HashMap::new(),
        }
    }

    pub fn locate(&mut self) {
        for server in GameServer::values() {
            let current_user = RegKey::predef(HKEY_CURRENT_USER);
            let wows = current_user.open_subkey(server.get_registry_key());
            if wows.is_err() {
                warn!("Failed to open registry key for {:?}", server);
                continue;
            }

            let wows = wows.unwrap();
            let install_location = wows.get_value("InstallLocation");
            if install_location.is_err() {
                warn!("Failed to get InstallLocation for {:?}", server);
                continue;
            }

            let path: String = install_location.unwrap();
            info!("Found game directory: {}", path);
            if path.is_empty() {
                continue;
            }

            // make sure the path is valid
            if Path::new(&path).exists() {
                self.game_directory_info.insert(server, path.to_string());
            }
        }
    }

    pub fn info(&self) {
        let count = self.game_directory_info.len();
        if count == 0 {
            println!("No game directory found.");
            return;
        }

        println!("Found {} game directory:", count);
        for (server, path) in &self.game_directory_info {
            println!("{:?}: {}", server, path);
        }
    }

    pub fn get_game_directory(&self, server: GameServer) -> Option<&String> {
        self.game_directory_info.get(&server)
    }
}
