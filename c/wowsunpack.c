#include "wowsunpack.h"
#include <Windows.h>
#include <stdio.h>

typedef void (*FUNC)(char*, int);

int cwowsunpack(char* path, int compact) {
    // load library
    HMODULE hModule = LoadLibrary("HenryQuan.WoWsUnpack.dll");
    if (hModule == NULL) {
        printf("Error: Could not load HenryQuan.WoWsUnpack.dll\n");
        return 1;
    }

    // get function address
    FUNC fp = (FUNC)GetProcAddress(hModule, "unpack");
    if (fp == NULL) {
        printf("Error: Could not find unpack() in HenryQuan.WoWsUnpack.dll\n");
        return 1;
    }

    fp(path, compact);
    return 0;
}
