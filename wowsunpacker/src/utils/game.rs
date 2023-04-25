extern crate winreg;
use log::{info, warn};
use std::{collections::HashMap, fmt, path::Path};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum GameServer {
    WW, // Global (ASIA, EU, NA, RU)
    CN, // The Chinese server
    PT, // The Public Test server
}

impl GameServer {
    pub fn values() -> Vec<GameServer> {
        vec![GameServer::WW, GameServer::CN, GameServer::PT]
    }

    fn get_registry_key(&self) -> &'static str {
        match self {
            GameServer::WW => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\1527964767"
            }
            GameServer::CN => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\WOWS.CN.PRODUCTION"
            }
            GameServer::PT => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\2376840996"
            }
        }
    }

    pub fn from(value: i32) -> Option<GameServer> {
        match value {
            0 => Some(GameServer::WW),
            1 => Some(GameServer::CN),
            2 => Some(GameServer::PT),
            _ => None,
        }
    }
}

pub struct GameDirectory {
    directory: HashMap<GameServer, String>,
}

impl GameDirectory {
    pub fn new() -> Self {
        Self {
            directory: HashMap::new(),
        }
    }

    pub fn locate(&mut self) -> &Self {
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
                self.directory.insert(server, path.to_string());
            }
        }

        self
    }

    pub fn info(&self) -> &Self {
        let count = self.directory.len();
        if count == 0 {
            warn!("No game directory found.");
            return self;
        }

        info!("Found {} game directory:", count);
        for (server, path) in &self.directory {
            info!("{:?}: {}", server, path);
        }

        self
    }

    pub fn get_game_directory(&self, server: &GameServer) -> Option<String> {
        Some(self.directory.get(server)?.to_owned())
    }
}

///
/// All supported game languages
///

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum GameLanguages {
    CS,
    DE,
    EN,
    ES,
    ES_MX,
    FR,
    IT,
    JA,
    KO,
    NL,
    PL,
    PT,
    PT_BR,
    RU,
    TH,
    UK,
    ZH,
    ZH_SG,
    ZH_TW,
}

impl fmt::Display for GameLanguages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameLanguages {
    pub fn to_folder_string(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn to_filename(&self) -> String {
        format!("{}.json", self.to_folder_string())
    }

    pub fn values() -> Vec<GameLanguages> {
        vec![
            GameLanguages::CS,
            GameLanguages::DE,
            GameLanguages::EN,
            GameLanguages::ES,
            GameLanguages::ES_MX,
            GameLanguages::FR,
            GameLanguages::IT,
            GameLanguages::JA,
            GameLanguages::KO,
            GameLanguages::NL,
            GameLanguages::PL,
            GameLanguages::PT,
            GameLanguages::PT_BR,
            GameLanguages::RU,
            GameLanguages::TH,
            GameLanguages::UK,
            GameLanguages::ZH,
            GameLanguages::ZH_SG,
            GameLanguages::ZH_TW,
        ]
    }
}
