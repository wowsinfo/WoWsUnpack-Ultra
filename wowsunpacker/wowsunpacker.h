#ifndef _WOWSUNPACKER_RUST_H_
#define _WOWSUNPACKER_RUST_H_

#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

/**
 * All supported game languages
 */
typedef enum GameLanguages {
  GAME_LANGUAGES_CS,
  GAME_LANGUAGES_DE,
  GAME_LANGUAGES_EN,
  GAME_LANGUAGES_ES,
  GAME_LANGUAGES_ES_MX,
  GAME_LANGUAGES_FR,
  GAME_LANGUAGES_IT,
  GAME_LANGUAGES_JA,
  GAME_LANGUAGES_KO,
  GAME_LANGUAGES_NL,
  GAME_LANGUAGES_PL,
  GAME_LANGUAGES_PT,
  GAME_LANGUAGES_PT_BR,
  GAME_LANGUAGES_RU,
  GAME_LANGUAGES_TH,
  GAME_LANGUAGES_UK,
  GAME_LANGUAGES_ZH,
  GAME_LANGUAGES_ZH_SG,
  GAME_LANGUAGES_ZH_TW,
} GameLanguages;

/**
 * All supported game servers
 */
typedef enum GameServer {
  GAME_SERVER_WW,
  GAME_SERVER_CN,
  GAME_SERVER_PT,
  GAME_SERVER_XX,
} GameServer;

/**
 * C Structs
 */
typedef struct GameDirectoryList {
  const char *const *list;
  int count;
} GameDirectoryList;

typedef struct GameServerList {
  const int *list;
  int count;
} GameServerList;

/**
 *  * Game Directory & Game Server
 *  * Get the game directory for a given server  * @param server: The server to get the game directory for  * @return A C string containing the game directory, or NULL if the server is not found
 */
const char *get_game_directory(int server);

/**
 *  * Get all available game directories  * @return A [GameDirectoryList] containing all available game directories
 */
const struct GameDirectoryList *get_all_game_directories(void);

/**
 *  * Get the first available game directory  * @return The first available game directory, or NULL if none are available
 */
const char *get_first_game_directory(void);

/**
 *  * Get all available game servers  * @return A [GameServerList] containing all available game servers
 */
const struct GameServerList *get_all_game_servers(void);

/**
 *  * Get the first available game server  * @return The first available game server, or -1 if none are available
 */
int get_first_game_server(void);

/**
 *  * Game Unpacker
 *  * Extract a list of entries/paths from the game data  * @param server: The game server id  * @param entries: The list of entries/paths to extract  * @param size: The size of the entries list, Rust doesn't know the size otherwise  * @param dest: The destination directory to extract to  * @return 0 if successful, 1 if not
 */
int unpack_game_data(int server,
                     const char *const *entries,
                     int size,
                     const char *dest);

/**
 * Free
 *  * Free a C string allocated by Rust [CString]  * @param ptr: The pointer to free  * @return Nothing
 */
void free_cstring(const char *ptr);

/**
 *  * Free a [GameDirectoryList] allocated by Rust  * @param ptr: The pointer to free  * @return Nothing
 */
void free_game_directory_list(const struct GameDirectoryList *ptr);

/**
 *  * Free a [GameServerList] allocated by Rust  * @param ptr: The pointer to free  * @return Nothing
 */
void free_game_server_list(const struct GameServerList *ptr);

#endif /* _WOWSUNPACKER_RUST_H_ */
