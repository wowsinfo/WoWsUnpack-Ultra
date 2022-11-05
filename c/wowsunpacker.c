#include <stdio.h>
#include <Windows.h>
#include "wowsunpacker.h"

extern char* get_game_directory(int);

int main() {
    printf("Test\n");
    char* path = get_game_directory(GAME_SERVER_WW);
    printf("Hello, World!, %s\n", path);
    free(path);
    return 0;
}
