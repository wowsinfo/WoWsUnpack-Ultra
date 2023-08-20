extern crate winreg;
use log::{error, info, warn};
use std::{collections::HashMap, fmt, path::Path};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

/// All supported game servers
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GameServer {
    WW, // Global (ASIA, EU, NA, RU)
    CN, // The Chinese server
    PT, // The Public Test server
    XX, // Unknown
}

impl GameServer {
    fn iter() -> impl Iterator<Item = &'static GameServer> {
        [GameServer::WW, GameServer::CN, GameServer::PT].iter()
    }

    const REGISTRY_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\";

    pub fn from_string(value: &str) -> GameServer {
        match value {
            "WOWS.WW.PRODUCTION" => GameServer::WW,
            "WOWS.CN.PRODUCTION" => GameServer::CN,
            "WOWS.PT.PRODUCTION" => GameServer::PT,
            _ => GameServer::XX,
        }
    }

    pub fn from_number(value: i32) -> GameServer {
        match value {
            0 => GameServer::WW,
            1 => GameServer::CN,
            2 => GameServer::PT,
            _ => GameServer::XX,
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

    /**
     * Get all available game directories path
     * @return A vector containing all available game directories path
     */
    pub fn available_path() -> Vec<String> {
        let mut dir = GameDirectory::new();
        let dir = dir.locate();
        let directory = &dir.directory;
        if directory.is_empty() {
            return Vec::new();
        }

        GameServer::iter()
            .flat_map(|server| dir.directory.get(&server).cloned())
            .collect()
    }

    /**
     * Get all available game servers, similar to `available_path` but returns [GameServer] instead of [String]
     * @return A vector containing all available game servers
     */
    pub fn available_server() -> Vec<GameServer> {
        let mut dir = GameDirectory::new();
        let dir = dir.locate();
        let directory = &dir.directory;
        if directory.is_empty() {
            return Vec::new();
        }

        GameServer::iter()
            .filter(|server| dir.directory.contains_key(server))
            .cloned()
            .collect()
    }

    pub fn locate(&mut self) -> &Self {
        let current_user = RegKey::predef(HKEY_CURRENT_USER);
        let uninstall = current_user.open_subkey(GameServer::REGISTRY_PATH);
        if uninstall.is_err() {
            error!("Failed to open registry key uninstall");
            return self;
        }

        // we no longer know the exact key of the Game, so we have to iterate through all of them
        let uninstall = uninstall.unwrap();
        uninstall
            .enum_keys()
            .flat_map(|key| {
                let folder = uninstall.open_subkey(key.ok()?).ok()?;
                let publisher: String = folder.get_value("Publisher").ok()?;
                // referenced from the group chat
                if !match publisher.as_str() {
                    "Wargaming.net" | "Wargaming Group Limited" | "360.cn" | "Lesta Games" => true,
                    _ => false,
                } {
                    return None;
                }

                let install_location = folder.get_value("InstallLocation").ok()?;
                // Find out the server
                let path = Path::new(&install_location).join("game_info.xml");
                if !path.exists() {
                    return None;
                }

                // Read until the line <id>xxx</id>
                let xml = std::fs::read_to_string(path).ok()?;
                let id = xml.lines().find(|line| line.contains("<id>"))?;
                let game_server_string = id.split(|c| c == '<' || c == '>').nth(2)?;
                log::info!("Found game server: {}", game_server_string);

                let server = GameServer::from_string(game_server_string);
                Some((server, install_location))
            })
            .for_each(|x| {
                let (server, path) = x;
                self.directory.insert(server, path);
            });

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

/// All supported game languages
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
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
