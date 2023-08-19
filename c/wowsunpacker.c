#include "../wowsunpacker/wowsunpacker.h"

#include <Windows.h>
#include <stdio.h>

int main() {
    printf("Test manual server\n");
    const char* path = get_game_directory(GAME_SERVER_WW);
    printf("Hello, World!, %s\n", path);
    free_cstring(path);

    printf("Test auto directory\n");
    const GameDirectoryList* directories = get_all_game_directories();
    if (directories == NULL) {
        printf("Error: Could not find any game directories\n");
        return 1;
    }

    for (int i = 0; i < directories->count; i++) {
        printf(" Directory: %s\n", directories->list[i]);
    }

    printf("Unpack game data\n");
    const char* entries[] = {"gui/dogTags/medium/", "gui/4k/",
                             "content/GameParams.data"};
    int result = unpack_game_data(GAME_SERVER_WW, entries, 3, "output");
    printf("Result: %d\n", result);
    return result;
}
