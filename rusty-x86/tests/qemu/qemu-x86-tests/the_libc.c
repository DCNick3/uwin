#ifdef TARGET_LINUX

static inline
int syscall(int syscall_number, int arg1, int arg2, int arg3)  {
    int ret;
    asm volatile (
        "pushl %%ebp\n\t"
        "movl %1, %%eax\n\t"
        "movl %2, %%ebx\n\t"
        "movl %3, %%ecx\n\t"
        "movl %4, %%edx\n\t"
        //"movl %4, %%edi\n\t"
        "int $0x80\n\t"
        "popl %%ebp\n\t"
        : "=a"(ret)
        : "g"(syscall_number), "g"(arg1), "g"(arg2), "g"(arg3)
        : "%ebx", "%ecx", "%edx", "%esi", "%edi"
    );
return ret;
}

int write(int fd, void* buffer, unsigned int size){
    return syscall(4, fd, (int) buffer, size);
}

void exit(int status) {
    syscall(1, status, 0, 0);
}

#else

#define OUTPUT_SIZE (20 * 1024 * 1024) // 2 MiB buffer
char output[OUTPUT_SIZE];

void exit(int status) {
    // Don't do anything when not on linux
}

#endif

void putchar_(char c) {
#ifdef TARGET_LINUX
    write(1, &c, 1);
#else
    static unsigned int output_index = 0;
    if (output_index == OUTPUT_SIZE) {
        __builtin_trap();
    }
    output[output_index++] = c;
#endif
}

int atexit(void (*function)(void)) {
    // ignore
    return 0;
}
