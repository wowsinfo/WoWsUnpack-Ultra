#include <stdio.h>
#include <Windows.h>
#include "../wowsunpacker/wowsunpacker.h"

int main() {
    printf("Test manual server\n");
    const char* path = get_game_directory(GAME_SERVER_WW);
    printf("Hello, World!, %s\n", path);
    free_cstring(path);

    printf("Test auto directory\n");
    const char* const* paths = get_all_game_directories();
    for (int i = 0; i < _GAME_SERVER_COUNT; i++) {
        printf("Hello, World!, %s\n", paths[i]);
    }
    free_cstring_list(paths);

    return 0;
}
