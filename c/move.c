/**
 * Copy dlls of wowsunpack from the release folder to the working folder.
 */

#include <Windows.h>
#include <stdio.h>

#define DLL_COUNT 7
#define MAX_PATH_LENGTH 256

int main() {
    const char* const DLLs[DLL_COUNT] = {
        "Newtonsoft.Json.dll",
        "Razorvine.Pickle.dll",
        "System.Buffers.dll",
        "System.Memory.dll",
        "System.Numerics.Vectors.dll",
        "System.Runtime.CompilerServices.Unsafe.dll",
        "HenryQuan.WoWsUnpack.dll"};

    for (int i = 0; i < DLL_COUNT; i++) {
        // get current working directory
        char cwd[MAX_PATH_LENGTH];
        GetCurrentDirectory(MAX_PATH_LENGTH, cwd);

        // get the path of the DLL
        char dll_path[MAX_PATH_LENGTH];
        char offset[8] = "";
        const char* dll_name = DLLs[i];
        if (strcmp(dll_name, "HenryQuan.WoWsUnpack.dll") == 0) {
            strcpy(offset, "x64\\");
        }

        sprintf(dll_path, "%s\\..\\wowsunpack\\wowsunpack\\bin\\Release\\%s%s",
                cwd, offset, dll_name);
        // copy the DLL to the current working directory, replace the existing one if it exists
        CopyFile(dll_path, dll_name, FALSE);

        printf("Copied %s\n", DLLs[i]);
    }

    printf("Copied all DLLs to current directory\n");
    return 0;
}