use pyo3::prelude::*;
use wowsunpacker::game::{GameDirectory as RustGameDirectory, GameServer};

/// A wrapper around the Rust implementation of GameDirectory
#[pyclass]
struct GameDirectory {
    game_directory: RustGameDirectory,
}

#[pymethods]
impl GameDirectory {
    /// Global (ASIA, EU, NA, RU)
    #[classattr]
    const GAME_SERVER_WW: i32 = 0;
    /// The Chinese server
    #[classattr]
    const GAME_SERVER_CN: i32 = 1;
    /// The Public Test server
    #[classattr]
    const GAME_SERVER_PT: i32 = 2;
    /// A list of game servers
    #[classattr]
    const GAME_SERVERS: [i32; 3] = [0, 1, 2];

    #[new]
    fn new() -> Self {
        GameDirectory {
            game_directory: RustGameDirectory::new(),
        }
    }

    /// Locates all game directories
    fn locate(&mut self) {
        self.game_directory.locate();
    }

    /// Gets the game directory for a specific server
    fn get_game_directory(&self, server: i32) -> PyResult<Option<String>> {
        let server = GameServer::from(server);
        if server.is_none() {
            return Ok(None);
        }

        let server = server.unwrap();
        let directory = self.game_directory.get_game_directory(&server);
        Ok(directory)
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn wowsdirectory(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GameDirectory>()?;

    Ok(())
}
