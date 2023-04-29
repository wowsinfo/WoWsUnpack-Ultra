#include <stdio.h>
#include <Windows.h>
#include "paramsunpack.h"

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("Usage: run.exe <path> [compact]\n", argv[0]);
        return 1;
    }
    return cparamsunpack(argv[1], argc > 2 ? atoi(argv[2]) : 0);
}
