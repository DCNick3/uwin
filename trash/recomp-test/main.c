
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

// I am dumb
typedef struct {
    uint32_t gpr[8];
    uint8_t flags[8];
} context;


uint8_t memory[1ULL << 32];

void sub_00401000(context*, uint8_t*);

void magic_MessageBoxA(context* ctx, uint8_t* mem) {
    ctx->gpr[4] += 5 * 4; // 5 dwords: 4 for arguments and 1 for the ret value
    
    printf("Message box!\n");
    
}

int main() {
    
    context ctx = {0};
    
    ctx.gpr[4] = 0x1000; // init ESP
    
    // well, we don't have actual memory here, so...
    // write the IAT entry for the MessageBoxA
    *((uint32_t*)&memory[0x404030]) = 0x406000;
    
    sub_00401000(&ctx, memory);
    
    return 0;
}
