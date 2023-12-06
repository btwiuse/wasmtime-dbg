#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>

int main() {
    void *addr = NULL;
    size_t length = 6444598427648;
    int prot = PROT_NONE;
    int flags = MAP_PRIVATE | MAP_ANONYMOUS;
    int fd = -1;
    off_t offset = 0;

    void *result = mmap(addr, length, prot, flags, fd, offset);
    if (result == MAP_FAILED) {
        perror("mmap failed");
        return 1;
    }

    printf("Memory mapped at address: %p\n", result);

    return 0;
}
