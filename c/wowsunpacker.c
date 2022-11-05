#include <stdio.h>
#include <Windows.h>

extern int dummy();

int main() {
    printf("Test\n");
    int number = dummy();
    printf("Hello, World!, %d\n", number);
    return 0;
}
