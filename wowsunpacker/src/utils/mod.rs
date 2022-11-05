pub mod functions;
pub mod game;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{game::{GameDirectory, GameLanguages}, functions::read_string};
    use crate::utils::game::GameServer;

    #[test]
    fn test_game_directory() {
        let mut game_dir = GameDirectory::new();
        game_dir.locate().info();
        assert!(game_dir.get_game_directory(&GameServer::WW).is_some());
    }

    #[test]
    fn test_game_languages() {
        let langs = GameLanguages::JA;
        assert_eq!(langs.to_filename(), "ja.json");
        assert_eq!(langs.to_folder_string(), "ja");
    }

    #[test]
    fn test_read_string() {
        let mut hello_world = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 00, 11, 22, 123, 44];
        
        // read until 0
        let output = read_string(&hello_world, 0);
        assert!(output.is_some());
        let output = output.unwrap();
        assert_eq!(output, "Hello World");

        // read with offset
        let output = read_string(&hello_world, 6);
        assert!(output.is_some());
        let output = output.unwrap();
        assert_eq!(output, "World");

        // read the first part
        hello_world[5] = 0;
        let output = read_string(&hello_world, 0);
        assert!(output.is_some());
        let output = output.unwrap();
        assert_eq!(output, "Hello");

        // string start from 0
        hello_world[0] = 0;
        let output = read_string(&hello_world, 0);
        assert!(output.is_none());
    }
}
