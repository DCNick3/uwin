
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#define SIZE (1ULL << 32ULL)

int main() {
    pid_t pid = getpid();

    char buffer[1024];
    sprintf(buffer, "cat /proc/%d/maps", pid);

    printf("\n\n");
    system(buffer);
    printf("\n\n");

    printf("Map the region\n");

    unsigned char* ptr = mmap(NULL, SIZE, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);

    printf("ptr = %p, errno = %s\n", ptr, strerror(errno));


    printf("\n\n");
    system(buffer);
    printf("\n\n");


    printf("Map a page at offset 0x1000\n");

    unsigned char* ptr1 = mmap(ptr + 0x1000, 0x1000, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, 0, 0);

    printf("ptr1 = %p, errno = %s\n", ptr1, strerror(errno));


    printf("\n\n");
    system(buffer);
    printf("\n\n");

    printf("Unmap & map w/o access the page at offset 0x1000\n");

    munmap(ptr + 0x1000, 0x1000);
    printf("errno = %s\n", strerror(errno));
    //ptr1 = mmap(ptr + 0x1000, 0x1000, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, 0, 0);
    //printf("ptr1 = %p, errno = %s", ptr1, strerror(errno));


    printf("\n\n");
    system(buffer);
    printf("\n\n");


    munmap(ptr, SIZE);
    printf("errno = %s\n", strerror(errno));


    printf("\n\n");
    system(buffer);
    printf("\n\n");
}



