#ifndef _WOWS_UNPACK_H_
#define _WOWS_UNPACK_H_

// Match with Rust Game Server
enum GameServer {
    GAME_SERVER_WW = 0,
    GAME_SERVER_CN = 1,
    GAME_SERVER_PT = 2,
};

/// Decode GameParams.data to GameParams.json
/// path - a string pointing to the GameParams.data file
/// compact - a boolean indicating the format mode
int cwowsunpack(char*, int);

#endif
