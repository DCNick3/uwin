/*
 *  x86 CPU test
 *
 *  Copyright (c) 2003 Fabrice Bellard
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, see <http://www.gnu.org/licenses/>.
 */
// #include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include <math.h>
#include <errno.h>

#include "the_libc.h"

// ???
// #define TEST_P4_FLAGS

// those are disabled because rusty_x86 does not implement them (yet)
// #define TEST_FLOATS 1
#define TEST_CMOV  1
#define TEST_FCOMI 1
// #define TEST_PARITY 1
// #define TEST_AUXCARRY 1
// #define TEST_LOOP 1
#define TEST_CMPS 1
// #define TEST_LODS 1
// #define TEST_XADD 1
// #define TEST_CMPXCHG 1
#define TEST_ADC 1
// #define TEST_ROL 1
// #define TEST_ROR 1
// #define TEST_RCL 1
#define TEST_RCR 1
#define TEST_SHLD 1
#define TEST_SHRD 1
#define TEST_BT 1
#define TEST_BTC 1
#define TEST_BTR 1
#define TEST_BTS 1
#define TEST_BSR 1
// #define TEST_BSF 1
// #define TEST_XLAT 1
#define TEST_CBW 1
#define TEST_CWDE 1
// #define TEST_BSWAP 1
// #define TEST_LEA16_EN 1

#define FMT64X "%016" PRIx64
#define FMTLX "%08lx"
#define X86_64_ONLY(x)

#define xglue(x, y) x ## y
#define glue(x, y) xglue(x, y)
#define stringify(s)	tostring(s)
#define tostring(s)	#s

#define CC_C   	0x0001
#define CC_P 	0x0004
#define CC_A	0x0010
#define CC_Z	0x0040
#define CC_S    0x0080
#define CC_O    0x0800

#define __init_call	__attribute__ ((unused,__section__ ("initcall")))


#if TEST_PARITY & TEST_AUXCARRY
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O | CC_A)
#elseif TEST_PARITY
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O)
#elseif TEST_AUXCARRY
#define CC_MASK (CC_C | CC_Z | CC_S | CC_O | CC_A)
#else
#define CC_MASK (CC_C | CC_Z | CC_S | CC_O)
#endif


#if defined(__x86_64__)
static inline long i2l(long v)
{
    return v | ((v ^ 0xabcd) << 32);
}
#else
static inline long i2l(long v)
{
    return v;
}
#endif

#define OP add
#include "test-i386.h"

#define OP sub
#include "test-i386.h"

#define OP xor
#include "test-i386.h"

#define OP and
#include "test-i386.h"

#define OP or
#include "test-i386.h"

#define OP cmp
#include "test-i386.h"

#ifdef TEST_ADC
#define OP adc
#define OP_CC
#include "test-i386.h"
#endif

#define OP sbb
#define OP_CC
#include "test-i386.h"

#define OP inc
#define OP_CC
#define OP1
#include "test-i386.h"

#define OP dec
#define OP_CC
#define OP1
#include "test-i386.h"

#define OP neg
#define OP_CC
#define OP1
#include "test-i386.h"

#define OP not
#define OP_CC
#define OP1
#include "test-i386.h"

#undef CC_MASK
#ifdef TEST_PARITY
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O)
#else
#define CC_MASK (CC_C | CC_Z | CC_S | CC_O)
#endif

#define OP shl
#include "test-i386-shift.h"

#define OP shr
#include "test-i386-shift.h"

#define OP sar
#include "test-i386-shift.h"

#ifdef TEST_ROL
#define OP rol
#define OP_ROTATE
#include "test-i386-shift.h"
#endif

#ifdef TEST_ROR
#define OP ror
#define OP_ROTATE
#include "test-i386-shift.h"
#endif

#ifdef TEST_RCR
#define OP rcr
#define OP_CC
#define OP_ROTATE
#include "test-i386-shift.h"
#endif

#ifdef TEST_RCL
#define OP rcl
#define OP_CC
#define OP_ROTATE
#include "test-i386-shift.h"
#endif

#ifdef TEST_SHLD
#define OP shld
#define OP_SHIFTD
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

#ifdef TEST_SHRD
#define OP shrd
#define OP_SHIFTD
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

/* XXX: should be more precise ? */
#undef CC_MASK
#define CC_MASK (CC_C)

#ifdef TEST_BT
#define OP bt
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

#ifdef TEST_BTS
#define OP bts
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

#ifdef TEST_BTR
#define OP btr
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

#ifdef TEST_BTC
#define OP btc
#define OP_NOBYTE
#include "test-i386-shift.h"
#endif

/* lea test (modrm support) */
#define TEST_LEAQ(STR)\
{\
    asm("lea " STR ", %0"\
        : "=r" (res)\
        : "a" (eax), "b" (ebx), "c" (ecx), "d" (edx), "S" (esi), "D" (edi));\
    printf("lea %s = " FMTLX "\n", STR, res);\
}

#define TEST_LEA(STR)\
{\
    asm("lea " STR ", %0"\
        : "=r" (res)\
        : "a" (eax), "b" (ebx), "c" (ecx), "d" (edx), "S" (esi), "D" (edi));\
    printf("lea %s = " FMTLX "\n", STR, res);\
}

#define TEST_LEA16(STR)\
{\
    asm(".code16 ; .byte 0x67 ; leal " STR ", %0 ; .code32"\
        : "=r" (res)\
        : "a" (eax), "b" (ebx), "c" (ecx), "d" (edx), "S" (esi), "D" (edi));\
    printf("lea %s = %08lx\n", STR, res);\
}


void test_lea(void)
{
    long eax, ebx, ecx, edx, esi, edi, res;
    eax = i2l(0x0001);
    ebx = i2l(0x0002);
    ecx = i2l(0x0004);
    edx = i2l(0x0008);
    esi = i2l(0x0010);
    edi = i2l(0x0020);

    TEST_LEA("0x4000");

    TEST_LEA("(%%eax)");
    TEST_LEA("(%%ebx)");
    TEST_LEA("(%%ecx)");
    TEST_LEA("(%%edx)");
    TEST_LEA("(%%esi)");
    TEST_LEA("(%%edi)");

    TEST_LEA("0x40(%%eax)");
    TEST_LEA("0x40(%%ebx)");
    TEST_LEA("0x40(%%ecx)");
    TEST_LEA("0x40(%%edx)");
    TEST_LEA("0x40(%%esi)");
    TEST_LEA("0x40(%%edi)");

    TEST_LEA("0x4000(%%eax)");
    TEST_LEA("0x4000(%%ebx)");
    TEST_LEA("0x4000(%%ecx)");
    TEST_LEA("0x4000(%%edx)");
    TEST_LEA("0x4000(%%esi)");
    TEST_LEA("0x4000(%%edi)");

    TEST_LEA("(%%eax, %%ecx)");
    TEST_LEA("(%%ebx, %%edx)");
    TEST_LEA("(%%ecx, %%ecx)");
    TEST_LEA("(%%edx, %%ecx)");
    TEST_LEA("(%%esi, %%ecx)");
    TEST_LEA("(%%edi, %%ecx)");

    TEST_LEA("0x40(%%eax, %%ecx)");
    TEST_LEA("0x4000(%%ebx, %%edx)");

    TEST_LEA("(%%ecx, %%ecx, 2)");
    TEST_LEA("(%%edx, %%ecx, 4)");
    TEST_LEA("(%%esi, %%ecx, 8)");

    TEST_LEA("(,%%eax, 2)");
    TEST_LEA("(,%%ebx, 4)");
    TEST_LEA("(,%%ecx, 8)");

    TEST_LEA("0x40(,%%eax, 2)");
    TEST_LEA("0x40(,%%ebx, 4)");
    TEST_LEA("0x40(,%%ecx, 8)");


    TEST_LEA("-10(%%ecx, %%ecx, 2)");
    TEST_LEA("-10(%%edx, %%ecx, 4)");
    TEST_LEA("-10(%%esi, %%ecx, 8)");

    TEST_LEA("0x4000(%%ecx, %%ecx, 2)");
    TEST_LEA("0x4000(%%edx, %%ecx, 4)");
    TEST_LEA("0x4000(%%esi, %%ecx, 8)");

#if defined(__x86_64__)
    TEST_LEAQ("0x4000");
    TEST_LEAQ("0x4000(%%rip)");

    TEST_LEAQ("(%%rax)");
    TEST_LEAQ("(%%rbx)");
    TEST_LEAQ("(%%rcx)");
    TEST_LEAQ("(%%rdx)");
    TEST_LEAQ("(%%rsi)");
    TEST_LEAQ("(%%rdi)");

    TEST_LEAQ("0x40(%%rax)");
    TEST_LEAQ("0x40(%%rbx)");
    TEST_LEAQ("0x40(%%rcx)");
    TEST_LEAQ("0x40(%%rdx)");
    TEST_LEAQ("0x40(%%rsi)");
    TEST_LEAQ("0x40(%%rdi)");

    TEST_LEAQ("0x4000(%%rax)");
    TEST_LEAQ("0x4000(%%rbx)");
    TEST_LEAQ("0x4000(%%rcx)");
    TEST_LEAQ("0x4000(%%rdx)");
    TEST_LEAQ("0x4000(%%rsi)");
    TEST_LEAQ("0x4000(%%rdi)");

    TEST_LEAQ("(%%rax, %%rcx)");
    TEST_LEAQ("(%%rbx, %%rdx)");
    TEST_LEAQ("(%%rcx, %%rcx)");
    TEST_LEAQ("(%%rdx, %%rcx)");
    TEST_LEAQ("(%%rsi, %%rcx)");
    TEST_LEAQ("(%%rdi, %%rcx)");

    TEST_LEAQ("0x40(%%rax, %%rcx)");
    TEST_LEAQ("0x4000(%%rbx, %%rdx)");

    TEST_LEAQ("(%%rcx, %%rcx, 2)");
    TEST_LEAQ("(%%rdx, %%rcx, 4)");
    TEST_LEAQ("(%%rsi, %%rcx, 8)");

    TEST_LEAQ("(,%%rax, 2)");
    TEST_LEAQ("(,%%rbx, 4)");
    TEST_LEAQ("(,%%rcx, 8)");

    TEST_LEAQ("0x40(,%%rax, 2)");
    TEST_LEAQ("0x40(,%%rbx, 4)");
    TEST_LEAQ("0x40(,%%rcx, 8)");


    TEST_LEAQ("-10(%%rcx, %%rcx, 2)");
    TEST_LEAQ("-10(%%rdx, %%rcx, 4)");
    TEST_LEAQ("-10(%%rsi, %%rcx, 8)");

    TEST_LEAQ("0x4000(%%rcx, %%rcx, 2)");
    TEST_LEAQ("0x4000(%%rdx, %%rcx, 4)");
    TEST_LEAQ("0x4000(%%rsi, %%rcx, 8)");
#else
#ifdef TEST_LEA16_EN
    /* limited 16 bit addressing test */
    TEST_LEA16("0x4000");
    TEST_LEA16("(%%bx)");
    TEST_LEA16("(%%si)");
    TEST_LEA16("(%%di)");
    TEST_LEA16("0x40(%%bx)");
    TEST_LEA16("0x40(%%si)");
    TEST_LEA16("0x40(%%di)");
    TEST_LEA16("0x4000(%%bx)");
    TEST_LEA16("0x4000(%%si)");
    TEST_LEA16("(%%bx,%%si)");
    TEST_LEA16("(%%bx,%%di)");
    TEST_LEA16("0x40(%%bx,%%si)");
    TEST_LEA16("0x40(%%bx,%%di)");
    TEST_LEA16("0x4000(%%bx,%%si)");
    TEST_LEA16("0x4000(%%bx,%%di)");
#endif
#endif
}

#define TEST_JCC(JCC, v1, v2)\
{\
    int res;\
    asm("movl $1, %0\n\t"\
        "cmpl %2, %1\n\t"\
        "j" JCC " 1f\n\t"\
        "movl $0, %0\n\t"\
        "1:\n\t"\
        : "=r" (res)\
        : "r" (v1), "r" (v2));\
    printf("%-10s %d\n", "j" JCC, res);\
\
    asm("movl $0, %0\n\t"\
        "cmpl %2, %1\n\t"\
        "set" JCC " %b0\n\t"\
        : "=r" (res)\
        : "r" (v1), "r" (v2));\
    printf("%-10s %d\n", "set" JCC, res);\
 if (TEST_CMOV) {\
    long val = i2l(1);\
    long res = i2l(0x12345678);\
X86_64_ONLY(\
    asm("cmpl %2, %1\n\t"\
        "cmov" JCC "q %3, %0\n\t"\
        : "=r" (res)\
        : "r" (v1), "r" (v2), "m" (val), "0" (res));\
        printf("%-10s R=" FMTLX "\n", "cmov" JCC "q", res);)\
    asm("cmpl %2, %1\n\t"\
        "cmov" JCC "l %k3, %k0\n\t"\
        : "=r" (res)\
        : "r" (v1), "r" (v2), "m" (val), "0" (res));\
        printf("%-10s R=" FMTLX "\n", "cmov" JCC "l", res);\
    asm("cmpl %2, %1\n\t"\
        "cmov" JCC "w %w3, %w0\n\t"\
        : "=r" (res)\
        : "r" (v1), "r" (v2), "r" (1), "0" (res));\
        printf("%-10s R=" FMTLX "\n", "cmov" JCC "w", res);\
 } \
}

/* various jump tests */
void test_jcc(void)
{
    TEST_JCC("ne", 1, 1);
    TEST_JCC("ne", 1, 0);

    TEST_JCC("e", 1, 1);
    TEST_JCC("e", 1, 0);

    TEST_JCC("l", 1, 1);
    TEST_JCC("l", 1, 0);
    TEST_JCC("l", 1, -1);

    TEST_JCC("le", 1, 1);
    TEST_JCC("le", 1, 0);
    TEST_JCC("le", 1, -1);

    TEST_JCC("ge", 1, 1);
    TEST_JCC("ge", 1, 0);
    TEST_JCC("ge", -1, 1);

    TEST_JCC("g", 1, 1);
    TEST_JCC("g", 1, 0);
    TEST_JCC("g", 1, -1);

    TEST_JCC("b", 1, 1);
    TEST_JCC("b", 1, 0);
    TEST_JCC("b", 1, -1);

    TEST_JCC("be", 1, 1);
    TEST_JCC("be", 1, 0);
    TEST_JCC("be", 1, -1);

    TEST_JCC("ae", 1, 1);
    TEST_JCC("ae", 1, 0);
    TEST_JCC("ae", 1, -1);

    TEST_JCC("a", 1, 1);
    TEST_JCC("a", 1, 0);
    TEST_JCC("a", 1, -1);

#ifdef TEST_PARITY
    TEST_JCC("p", 1, 1);
    TEST_JCC("p", 1, 0);

    TEST_JCC("np", 1, 1);
    TEST_JCC("np", 1, 0);
#endif

    TEST_JCC("o", 0x7fffffff, 0);
    TEST_JCC("o", 0x7fffffff, -1);

    TEST_JCC("no", 0x7fffffff, 0);
    TEST_JCC("no", 0x7fffffff, -1);

    TEST_JCC("s", 0, 1);
    TEST_JCC("s", 0, -1);
    TEST_JCC("s", 0, 0);

    TEST_JCC("ns", 0, 1);
    TEST_JCC("ns", 0, -1);
    TEST_JCC("ns", 0, 0);
}

#ifdef TEST_LOOP

#define TEST_LOOP(insn) \
{\
    for(i = 0; i < sizeof(ecx_vals) / sizeof(long); i++) {\
        ecx = ecx_vals[i];\
        for(zf = 0; zf < 2; zf++) {\
    asm("test %2, %2\n\t"\
        "movl $1, %0\n\t"\
          insn " 1f\n\t" \
        "movl $0, %0\n\t"\
        "1:\n\t"\
        : "=a" (res)\
        : "c" (ecx), "b" (!zf)); \
    printf("%-10s ECX=" FMTLX " ZF=%ld r=%d\n", insn, ecx, zf, res);      \
        }\
   }\
}

void test_loop(void)
{
    long ecx, zf;
    const long ecx_vals[] = {
        0,
        1,
        0x10000,
        0x10001,
#if defined(__x86_64__)
        0x100000000L,
        0x100000001L,
#endif
    };
    int i, res;

#if !defined(__x86_64__)
    TEST_LOOP("jcxz");
    TEST_LOOP("loopw");
    TEST_LOOP("loopzw");
    TEST_LOOP("loopnzw");
#endif

    TEST_LOOP("jecxz");
    TEST_LOOP("loopl");
    TEST_LOOP("loopzl");
    TEST_LOOP("loopnzl");
}

#endif

#undef CC_MASK
#ifdef TEST_P4_FLAGS
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O | CC_A)
#else
#define CC_MASK (CC_O | CC_C)
#endif

#define OP mul
#include "test-i386-muldiv.h"

#define OP imul
#include "test-i386-muldiv.h"

void test_imulw2(long op0, long op1)
{
    long res, s1, s0, flags;
    s0 = op0;
    s1 = op1;
    res = s0;
    flags = 0;
    asm volatile ("push %4\n\t"
         "popf\n\t"
         "imulw %w2, %w0\n\t"
         "pushf\n\t"
         "pop %1\n\t"
         : "=q" (res), "=g" (flags)
         : "q" (s1), "0" (res), "1" (flags));
    printf("%-10s A=" FMTLX " B=" FMTLX " R=" FMTLX " CC=%04lx\n",
           "imulw", s0, s1, res, flags & CC_MASK);
}

void test_imull2(long op0, long op1)
{
    long res, s1, s0, flags;
    s0 = op0;
    s1 = op1;
    res = s0;
    flags = 0;
    asm volatile ("push %4\n\t"
         "popf\n\t"
         "imull %k2, %k0\n\t"
         "pushf\n\t"
         "pop %1\n\t"
         : "=q" (res), "=g" (flags)
         : "q" (s1), "0" (res), "1" (flags));
    printf("%-10s A=" FMTLX " B=" FMTLX " R=" FMTLX " CC=%04lx\n",
           "imull", s0, s1, res, flags & CC_MASK);
}

#if defined(__x86_64__)
void test_imulq2(long op0, long op1)
{
    long res, s1, s0, flags;
    s0 = op0;
    s1 = op1;
    res = s0;
    flags = 0;
    asm volatile ("push %4\n\t"
         "popf\n\t"
         "imulq %2, %0\n\t"
         "pushf\n\t"
         "pop %1\n\t"
         : "=q" (res), "=g" (flags)
         : "q" (s1), "0" (res), "1" (flags));
    printf("%-10s A=" FMTLX " B=" FMTLX " R=" FMTLX " CC=%04lx\n",
           "imulq", s0, s1, res, flags & CC_MASK);
}
#endif

#define TEST_IMUL_IM(size, rsize, op0, op1)\
{\
    long res, flags, s1;\
    flags = 0;\
    res = 0;\
    s1 = op1;\
    asm volatile ("push %3\n\t"\
         "popf\n\t"\
         "imul" size " $" #op0 ", %" rsize "2, %" rsize "0\n\t" \
         "pushf\n\t"\
         "pop %1\n\t"\
         : "=r" (res), "=g" (flags)\
         : "r" (s1), "1" (flags), "0" (res));\
    printf("%-10s A=" FMTLX " B=" FMTLX " R=" FMTLX " CC=%04lx\n",\
           "imul" size " im", (long)op0, (long)op1, res, flags & CC_MASK);\
}


#undef CC_MASK
#define CC_MASK (0)

#define OP div
#include "test-i386-muldiv.h"

#define OP idiv
#include "test-i386-muldiv.h"

void test_mul(void)
{
    test_imulb(0x1234561d, 4);
    test_imulb(3, -4);
    test_imulb(0x80, 0x80);
    test_imulb(0x10, 0x10);

    test_imulw(0, 0x1234001d, 45);
    test_imulw(0, 23, -45);
    test_imulw(0, 0x8000, 0x8000);
    test_imulw(0, 0x100, 0x100);

    test_imull(0, 0x1234001d, 45);
    test_imull(0, 23, -45);
    test_imull(0, 0x80000000, 0x80000000);
    test_imull(0, 0x10000, 0x10000);

    test_mulb(0x1234561d, 4);
    test_mulb(3, -4);
    test_mulb(0x80, 0x80);
    test_mulb(0x10, 0x10);

    test_mulw(0, 0x1234001d, 45);
    test_mulw(0, 23, -45);
    test_mulw(0, 0x8000, 0x8000);
    test_mulw(0, 0x100, 0x100);

    test_mull(0, 0x1234001d, 45);
    test_mull(0, 23, -45);
    test_mull(0, 0x80000000, 0x80000000);
    test_mull(0, 0x10000, 0x10000);

    test_imulw2(0x1234001d, 45);
    test_imulw2(23, -45);
    test_imulw2(0x8000, 0x8000);
    test_imulw2(0x100, 0x100);

    test_imull2(0x1234001d, 45);
    test_imull2(23, -45);
    test_imull2(0x80000000, 0x80000000);
    test_imull2(0x10000, 0x10000);

    TEST_IMUL_IM("w", "w", 45, 0x1234);
    TEST_IMUL_IM("w", "w", -45, 23);
    TEST_IMUL_IM("w", "w", 0x8000, 0x80000000);
    TEST_IMUL_IM("w", "w", 0x7fff, 0x1000);

    TEST_IMUL_IM("l", "k", 45, 0x1234);
    TEST_IMUL_IM("l", "k", -45, 23);
    TEST_IMUL_IM("l", "k", 0x8000, 0x80000000);
    TEST_IMUL_IM("l", "k", 0x7fff, 0x1000);

    test_idivb(0x12341678, 0x127e);
    test_idivb(0x43210123, -5);
    test_idivb(0x12340004, -1);

    test_idivw(0, 0x12345678, 12347);
    test_idivw(0, -23223, -45);
    test_idivw(0, 0x12348000, -1);
    test_idivw(0x12343, 0x12345678, 0x81238567);

    test_idivl(0, 0x12345678, 12347);
    test_idivl(0, -233223, -45);
    test_idivl(0, 0x80000000, -1);
    test_idivl(0x12343, 0x12345678, 0x81234567);

    test_divb(0x12341678, 0x127e);
    test_divb(0x43210123, -5);
    test_divb(0x12340004, -1);

    test_divw(0, 0x12345678, 12347);
    test_divw(0, -23223, -45);
    test_divw(0, 0x12348000, -1);
    test_divw(0x12343, 0x12345678, 0x81238567);

    test_divl(0, 0x12345678, 12347);
    test_divl(0, -233223, -45);
    test_divl(0, 0x80000000, -1);
    test_divl(0x12343, 0x12345678, 0x81234567);

#if defined(__x86_64__)
    test_imulq(0, 0x1234001d1234001d, 45);
    test_imulq(0, 23, -45);
    test_imulq(0, 0x8000000000000000, 0x8000000000000000);
    test_imulq(0, 0x100000000, 0x100000000);

    test_mulq(0, 0x1234001d1234001d, 45);
    test_mulq(0, 23, -45);
    test_mulq(0, 0x8000000000000000, 0x8000000000000000);
    test_mulq(0, 0x100000000, 0x100000000);

    test_imulq2(0x1234001d1234001d, 45);
    test_imulq2(23, -45);
    test_imulq2(0x8000000000000000, 0x8000000000000000);
    test_imulq2(0x100000000, 0x100000000);

    TEST_IMUL_IM("q", "", 45, 0x12341234);
    TEST_IMUL_IM("q", "", -45, 23);
    TEST_IMUL_IM("q", "", 0x8000, 0x8000000000000000);
    TEST_IMUL_IM("q", "", 0x7fff, 0x10000000);

    test_idivq(0, 0x12345678abcdef, 12347);
    test_idivq(0, -233223, -45);
    test_idivq(0, 0x8000000000000000, -1);
    test_idivq(0x12343, 0x12345678, 0x81234567);

    test_divq(0, 0x12345678abcdef, 12347);
    test_divq(0, -233223, -45);
    test_divq(0, 0x8000000000000000, -1);
    test_divq(0x12343, 0x12345678, 0x81234567);
#endif
}

#define TEST_BSX(op, size, op0)\
{\
    long res, val, resz;\
    val = op0;\
    asm("xor %1, %1\n"\
        "mov $0x12345678, %0\n"\
        #op " %" size "2, %" size "0 ; setz %b1" \
        : "=&r" (res), "=&q" (resz)\
        : "r" (val));\
    printf("%-10s A=" FMTLX " R=" FMTLX " %ld\n", #op, val, res, resz);\
}

void test_bsx(void)
{

#ifdef TEST_BSR
    TEST_BSX(bsrw, "w", 0);
    TEST_BSX(bsrw, "w", 0x12340128);

    TEST_BSX(bsrl, "k", 0);
    TEST_BSX(bsrl, "k", 0x00340128);
#endif

#ifdef TEST_BSF
    TEST_BSX(bsfw, "w", 0);
    TEST_BSX(bsfw, "w", 0x12340128);

    TEST_BSX(bsfl, "k", 0);
    TEST_BSX(bsfl, "k", 0x00340128);
#endif

#if defined(__x86_64__)
    TEST_BSX(bsrq, "", 0);
    TEST_BSX(bsrq, "", 0x003401281234);
    TEST_BSX(bsfq, "", 0);
    TEST_BSX(bsfq, "", 0x003401281234);
#endif
}

/**********************************************/

#ifdef TEST_FLOATS
union float64u {
    double d;
    uint64_t l;
};

union float64u q_nan = { .l = 0xFFF8000000000000LL };
union float64u s_nan = { .l = 0xFFF0000000000000LL };

void test_fops(double a, double b)
{
    printf("a=%f b=%f a+b=%f\n", a, b, a + b);
    printf("a=%f b=%f a-b=%f\n", a, b, a - b);
    printf("a=%f b=%f a*b=%f\n", a, b, a * b);
    printf("a=%f b=%f a/b=%f\n", a, b, a / b);
    printf("a=%f b=%f fmod(a, b)=%f\n", a, b, fmod(a, b));
    printf("a=%f sqrt(a)=%f\n", a, sqrt(a));
    printf("a=%f sin(a)=%f\n", a, sin(a));
    printf("a=%f cos(a)=%f\n", a, cos(a));
    printf("a=%f tan(a)=%f\n", a, tan(a));
    printf("a=%f log(a)=%f\n", a, log(a));
    printf("a=%f exp(a)=%f\n", a, exp(a));
    printf("a=%f b=%f atan2(a, b)=%f\n", a, b, atan2(a, b));
    /* just to test some op combining */
    printf("a=%f asin(sin(a))=%f\n", a, asin(sin(a)));
    printf("a=%f acos(cos(a))=%f\n", a, acos(cos(a)));
    printf("a=%f atan(tan(a))=%f\n", a, atan(tan(a)));

}

void fpu_clear_exceptions(void)
{
    struct QEMU_PACKED {
        uint16_t fpuc;
        uint16_t dummy1;
        uint16_t fpus;
        uint16_t dummy2;
        uint16_t fptag;
        uint16_t dummy3;
        uint32_t ignored[4];
        long double fpregs[8];
    } float_env32;

    asm volatile ("fnstenv %0\n" : "=m" (float_env32));
    float_env32.fpus &= ~0x7f;
    asm volatile ("fldenv %0\n" : : "m" (float_env32));
}

/* XXX: display exception bits when supported */
#define FPUS_EMASK 0x0000
//#define FPUS_EMASK 0x007f

void test_fcmp(double a, double b)
{
    long eflags, fpus;

    fpu_clear_exceptions();
    asm("fcom %2\n"
        "fstsw %%ax\n"
        : "=a" (fpus)
        : "t" (a), "u" (b));
    printf("fcom(%f %f)=%04lx\n",
           a, b, fpus & (0x4500 | FPUS_EMASK));
    fpu_clear_exceptions();
    asm("fucom %2\n"
        "fstsw %%ax\n"
        : "=a" (fpus)
        : "t" (a), "u" (b));
    printf("fucom(%f %f)=%04lx\n",
           a, b, fpus & (0x4500 | FPUS_EMASK));
    if (TEST_FCOMI) {
        /* test f(u)comi instruction */
        fpu_clear_exceptions();
        asm("fcomi %3, %2\n"
            "fstsw %%ax\n"
            "pushf\n"
            "pop %0\n"
            : "=r" (eflags), "=a" (fpus)
            : "t" (a), "u" (b));
        printf("fcomi(%f %f)=%04lx %02lx\n",
               a, b, fpus & FPUS_EMASK, eflags & (CC_Z | CC_P | CC_C));
        fpu_clear_exceptions();
        asm("fucomi %3, %2\n"
            "fstsw %%ax\n"
            "pushf\n"
            "pop %0\n"
            : "=r" (eflags), "=a" (fpus)
            : "t" (a), "u" (b));
        printf("fucomi(%f %f)=%04lx %02lx\n",
               a, b, fpus & FPUS_EMASK, eflags & (CC_Z | CC_P | CC_C));
    }
    fpu_clear_exceptions();
    asm volatile("fxam\n"
                 "fstsw %%ax\n"
                 : "=a" (fpus)
                 : "t" (a));
    printf("fxam(%f)=%04lx\n", a, fpus & 0x4700);
    fpu_clear_exceptions();
}

void test_fcvt(double a)
{
    float fa;
    long double la;
    int16_t fpuc;
    int i;
    int64_t lla;
    int ia;
    int16_t wa;
    double ra;

    fa = a;
    la = a;
    printf("(float)%f = %f\n", a, fa);
    printf("(long double)%f = %Lf\n", a, la);
    printf("a=" FMT64X "\n", *(uint64_t *)&a);
    printf("la=" FMT64X " %04x\n", *(uint64_t *)&la,
           *(unsigned short *)((char *)(&la) + 8));

    /* test all roundings */
    asm volatile ("fstcw %0" : "=m" (fpuc));
    for(i=0;i<4;i++) {
        uint16_t val16;
        val16 = (fpuc & ~0x0c00) | (i << 10);
        asm volatile ("fldcw %0" : : "m" (val16));
        asm volatile ("fist %0" : "=m" (wa) : "t" (a));
        asm volatile ("fistl %0" : "=m" (ia) : "t" (a));
        asm volatile ("fistpll %0" : "=m" (lla) : "t" (a) : "st");
        asm volatile ("frndint ; fstl %0" : "=m" (ra) : "t" (a));
        asm volatile ("fldcw %0" : : "m" (fpuc));
        printf("(short)a = %d\n", wa);
        printf("(int)a = %d\n", ia);
        printf("(int64_t)a = " FMT64X "\n", lla);
        printf("rint(a) = %f\n", ra);
    }
}

#define TEST(N) \
    asm("fld" #N : "=t" (a)); \
    printf("fld" #N "= %f\n", a);

void test_fconst(void)
{
    double a;
    TEST(1);
    TEST(l2t);
    TEST(l2e);
    TEST(pi);
    TEST(lg2);
    TEST(ln2);
    TEST(z);
}

void test_fbcd(double a)
{
    unsigned short bcd[5];
    double b;

    asm("fbstp %0" : "=m" (bcd[0]) : "t" (a) : "st");
    asm("fbld %1" : "=t" (b) : "m" (bcd[0]));
    printf("a=%f bcd=%04x%04x%04x%04x%04x b=%f\n",
           a, bcd[4], bcd[3], bcd[2], bcd[1], bcd[0], b);
}

#define TEST_ENV(env, save, restore)\
{\
    memset((env), 0xaa, sizeof(*(env)));\
    for(i=0;i<5;i++)\
        asm volatile ("fldl %0" : : "m" (dtab[i]));\
    asm volatile (save " %0\n" : : "m" (*(env)));\
    asm volatile (restore " %0\n": : "m" (*(env)));\
    for(i=0;i<5;i++)\
        asm volatile ("fstpl %0" : "=m" (rtab[i]));\
    for(i=0;i<5;i++)\
        printf("res[%d]=%f\n", i, rtab[i]);\
    printf("fpuc=%04x fpus=%04x fptag=%04x\n",\
           (env)->fpuc,\
           (env)->fpus & 0xff00,\
           (env)->fptag);\
}

void test_fenv(void)
{
    struct __attribute__((__packed__)) {
        uint16_t fpuc;
        uint16_t dummy1;
        uint16_t fpus;
        uint16_t dummy2;
        uint16_t fptag;
        uint16_t dummy3;
        uint32_t ignored[4];
        long double fpregs[8];
    } float_env32;
    struct __attribute__((__packed__)) {
        uint16_t fpuc;
        uint16_t fpus;
        uint16_t fptag;
        uint16_t ignored[4];
        long double fpregs[8];
    } float_env16;
    double dtab[8];
    double rtab[8];
    int i;

    for(i=0;i<8;i++)
        dtab[i] = i + 1;

    TEST_ENV(&float_env16, "data16 fnstenv", "data16 fldenv");
    TEST_ENV(&float_env16, "data16 fnsave", "data16 frstor");
    TEST_ENV(&float_env32, "fnstenv", "fldenv");
    TEST_ENV(&float_env32, "fnsave", "frstor");

    /* test for ffree */
    for(i=0;i<5;i++)
        asm volatile ("fldl %0" : : "m" (dtab[i]));
    asm volatile("ffree %st(2)");
    asm volatile ("fnstenv %0\n" : : "m" (float_env32));
    asm volatile ("fninit");
    printf("fptag=%04x\n", float_env32.fptag);
}


#define TEST_FCMOV(a, b, eflags, CC)\
{\
    double res;\
    asm("push %3\n"\
        "popf\n"\
        "fcmov" CC " %2, %0\n"\
        : "=t" (res)\
        : "0" (a), "u" (b), "g" (eflags));\
    printf("fcmov%s eflags=0x%04lx-> %f\n", \
           CC, (long)eflags, res);\
}

void test_fcmov(void)
{
    double a, b;
    long eflags, i;

    a = 1.0;
    b = 2.0;
    for(i = 0; i < 4; i++) {
        eflags = 0;
        if (i & 1)
            eflags |= CC_C;
        if (i & 2)
            eflags |= CC_Z;
        TEST_FCMOV(a, b, eflags, "b");
        TEST_FCMOV(a, b, eflags, "e");
        TEST_FCMOV(a, b, eflags, "be");
        TEST_FCMOV(a, b, eflags, "nb");
        TEST_FCMOV(a, b, eflags, "ne");
        TEST_FCMOV(a, b, eflags, "nbe");
    }
    TEST_FCMOV(a, b, 0, "u");
    TEST_FCMOV(a, b, CC_P, "u");
    TEST_FCMOV(a, b, 0, "nu");
    TEST_FCMOV(a, b, CC_P, "nu");
}

void test_floats(void)
{
    test_fops(2, 3);
    test_fops(1.4, -5);
    test_fcmp(2, -1);
    test_fcmp(2, 2);
    test_fcmp(2, 3);
    test_fcmp(2, q_nan.d);
    test_fcmp(q_nan.d, -1);
    test_fcmp(-1.0/0.0, -1);
    test_fcmp(1.0/0.0, -1);
    test_fcvt(0.5);
    test_fcvt(-0.5);
    test_fcvt(1.0/7.0);
    test_fcvt(-1.0/9.0);
    test_fcvt(32768);
    test_fcvt(-1e20);
    test_fcvt(-1.0/0.0);
    test_fcvt(1.0/0.0);
    test_fcvt(q_nan.d);
    test_fconst();
    test_fbcd(1234567890123456.0);
    test_fbcd(-123451234567890.0);
    test_fenv();
    if (TEST_CMOV) {
        test_fcmov();
    }
}
#endif

#define TEST_XCHG(op, size, opconst)\
{\
    long op0, op1;\
    op0 = i2l(0x12345678);\
    op1 = i2l(0xfbca7654);\
    asm(#op " %" size "0, %" size "1" \
        : "=q" (op0), opconst (op1) \
        : "0" (op0));\
    printf("%-10s A=" FMTLX " B=" FMTLX "\n",\
           #op, op0, op1);\
}

#define TEST_CMPXCHG(op, size, opconst, eax)\
{\
    long op0, op1, op2;\
    op0 = i2l(0x12345678);\
    op1 = i2l(0xfbca7654);\
    op2 = i2l(eax);\
    asm(#op " %" size "0, %" size "1" \
        : "=q" (op0), opconst (op1) \
        : "0" (op0), "a" (op2));\
    printf("%-10s EAX=" FMTLX " A=" FMTLX " C=" FMTLX "\n",\
           #op, op2, op0, op1);\
}

void test_xchg(void)
{
#if defined(__x86_64__)
    TEST_XCHG(xchgq, "", "+q");
#endif
    TEST_XCHG(xchgl, "k", "+q");
    TEST_XCHG(xchgw, "w", "+q");
    TEST_XCHG(xchgb, "b", "+q");

#if defined(__x86_64__)
    TEST_XCHG(xchgq, "", "+m");
#endif
    TEST_XCHG(xchgl, "k", "+m");
    TEST_XCHG(xchgw, "w", "+m");
    TEST_XCHG(xchgb, "b", "+m");

#ifdef TEST_XADD
#if defined(__x86_64__)
    TEST_XCHG(xaddq, "", "+q");
#endif
    TEST_XCHG(xaddl, "k", "+q");
    TEST_XCHG(xaddw, "w", "+q");
    TEST_XCHG(xaddb, "b", "+q");

    {
        int res;
        res = 0x12345678;
        asm("xaddl %1, %0" : "=r" (res) : "0" (res));
        printf("xaddl same res=%08x\n", res);
    }

#if defined(__x86_64__)
    TEST_XCHG(xaddq, "", "+m");
#endif
    TEST_XCHG(xaddl, "k", "+m");
    TEST_XCHG(xaddw, "w", "+m");
    TEST_XCHG(xaddb, "b", "+m");
#endif // TEST_XADD

#if defined(__x86_64__)
#ifdef TEST_CMPXCHG
    TEST_CMPXCHG(cmpxchgq, "", "+q", 0xfbca7654);
#endif
    TEST_CMPXCHG(cmpxchgl, "k", "+q", 0xfbca7654);
    TEST_CMPXCHG(cmpxchgw, "w", "+q", 0xfbca7654);
    TEST_CMPXCHG(cmpxchgb, "b", "+q", 0xfbca7654);

#if defined(__x86_64__)
    TEST_CMPXCHG(cmpxchgq, "", "+q", 0xfffefdfc);
#endif
    TEST_CMPXCHG(cmpxchgl, "k", "+q", 0xfffefdfc);
    TEST_CMPXCHG(cmpxchgw, "w", "+q", 0xfffefdfc);
    TEST_CMPXCHG(cmpxchgb, "b", "+q", 0xfffefdfc);

#if defined(__x86_64__)
    TEST_CMPXCHG(cmpxchgq, "", "+m", 0xfbca7654);
#endif
    TEST_CMPXCHG(cmpxchgl, "k", "+m", 0xfbca7654);
    TEST_CMPXCHG(cmpxchgw, "w", "+m", 0xfbca7654);
    TEST_CMPXCHG(cmpxchgb, "b", "+m", 0xfbca7654);

#if defined(__x86_64__)
    TEST_CMPXCHG(cmpxchgq, "", "+m", 0xfffefdfc);
#endif
    TEST_CMPXCHG(cmpxchgl, "k", "+m", 0xfffefdfc);
    TEST_CMPXCHG(cmpxchgw, "w", "+m", 0xfffefdfc);
    TEST_CMPXCHG(cmpxchgb, "b", "+m", 0xfffefdfc);

    {
        uint64_t op0, op1, op2;
        long eax, edx;
        long i, eflags;

        for(i = 0; i < 2; i++) {
            op0 = 0x123456789abcdLL;
            eax = i2l(op0 & 0xffffffff);
            edx = i2l(op0 >> 32);
            if (i == 0)
                op1 = 0xfbca765423456LL;
            else
                op1 = op0;
            op2 = 0x6532432432434LL;
            asm("cmpxchg8b %2\n"
                "pushf\n"
                "pop %3\n"
                : "=a" (eax), "=d" (edx), "=m" (op1), "=g" (eflags)
                : "0" (eax), "1" (edx), "m" (op1), "b" ((int)op2), "c" ((int)(op2 >> 32)));
            printf("cmpxchg8b: eax=" FMTLX " edx=" FMTLX " op1=" FMT64X " CC=%02lx\n",
                   eax, edx, op1, eflags & CC_Z);
        }
    }
#endif // TEST_CMPXCHG
}

void test_misc(void)
{
    long res;
    
#ifdef TEST_XLAT
    long i;
    char table[256];

    for(i=0;i<256;i++) table[i] = 256 - i;
    res = 0x12345678;
    asm ("xlat" : "=a" (res) : "b" (table), "0" (res));
    printf("xlat: EAX=" FMTLX "\n", res);
#endif

#if defined(__x86_64__)
    /* specific popl test */
    asm volatile ("push $12345432 ; push $0x9abcdef ; pop (%%rsp) ; pop %0"
                  : "=g" (res));
    printf("popl esp=" FMTLX "\n", res);
#else
    /* specific popl test */
    asm volatile ("pushl $12345432 ; pushl $0x9abcdef ; popl (%%esp) ; popl %0"
                  : "=g" (res));
    printf("popl esp=" FMTLX "\n", res);

    /* specific popw test */
    asm volatile ("pushl $12345432 ; pushl $0x9abcdef ; popw (%%esp) ; addl $2, %%esp ; popl %0"
                  : "=g" (res));
    printf("popw esp=" FMTLX "\n", res);
#endif
}

uint8_t str_buffer[4096];

#undef CC_MASK
#if TEST_PARITY & TEST_AUXCARRY
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O | CC_A)
#elseif TEST_PARITY
#define CC_MASK (CC_C | CC_P | CC_Z | CC_S | CC_O)
#elseif TEST_AUXCARRY
#define CC_MASK (CC_C | CC_Z | CC_S | CC_O | CC_A)
#else
#define CC_MASK (CC_C | CC_Z | CC_S | CC_O)
#endif

#define TEST_STRING1(OP, size, DF, REP)\
{\
    long esi, edi, eax, ecx, eflags;\
\
    esi = (long)(str_buffer + sizeof(str_buffer) / 2);\
    edi = (long)(str_buffer + sizeof(str_buffer) / 2) + 16;\
    eax = i2l(0x12345678);\
    ecx = 17;\
\
    asm volatile ("push $0\n\t"\
                  "popf\n\t"\
                  DF "\n\t"\
                  REP #OP size "\n\t"\
                  "cld\n\t"\
                  "pushf\n\t"\
                  "pop %4\n\t"\
                  : "=S" (esi), "=D" (edi), "=a" (eax), "=c" (ecx), "=g" (eflags)\
                  : "0" (esi), "1" (edi), "2" (eax), "3" (ecx));\
    printf("%-10s ESI=" FMTLX " EDI=" FMTLX " EAX=" FMTLX " ECX=" FMTLX " EFL=%04x\n",\
           REP #OP size, esi, edi, eax, ecx,\
           (int)(eflags & CC_MASK));\
}

#define TEST_STRING(OP, REP)\
    TEST_STRING1(OP, "b", "", REP);\
    TEST_STRING1(OP, "w", "", REP);\
    TEST_STRING1(OP, "l", "", REP);\
    X86_64_ONLY(TEST_STRING1(OP, "q", "", REP));\
    TEST_STRING1(OP, "b", "std", REP);\
    TEST_STRING1(OP, "w", "std", REP);\
    TEST_STRING1(OP, "l", "std", REP);\
    X86_64_ONLY(TEST_STRING1(OP, "q", "std", REP))

void test_string(void)
{
    int i;
    for(i = 0;i < sizeof(str_buffer); i++)
        str_buffer[i] = i + 0x56;
   TEST_STRING(stos, "");
   TEST_STRING(stos, "rep ");
#ifdef TEST_LODS
   TEST_STRING(lods, ""); /* to verify stos */
   TEST_STRING(lods, "rep ");
#endif

   TEST_STRING(movs, "");
   TEST_STRING(movs, "rep ");
#ifdef TEST_LODS
   TEST_STRING(lods, ""); /* to verify movs */
#endif

   /* XXX: better tests */
   TEST_STRING(scas, "");
   TEST_STRING(scas, "repz ");
   TEST_STRING(scas, "repnz ");
#ifdef TEST_CMPS
   TEST_STRING(cmps, "");
   TEST_STRING(cmps, "repz ");
   TEST_STRING(cmps, "repnz ");
#endif
}

long enter_stack[4096];

#if defined(__x86_64__)
#define RSP "%%rsp"
#define RBP "%%rbp"
#else
#define RSP "%%esp"
#define RBP "%%ebp"
#endif

#if !defined(__x86_64__)
/* causes an infinite loop, disable it for now.  */
#define TEST_ENTER(size, stack_type, level)
#else
#define TEST_ENTER(size, stack_type, level)\
{\
    long esp_save, esp_val, ebp_val, ebp_save, i;\
    stack_type *ptr, *stack_end, *stack_ptr;\
    memset(enter_stack, 0, sizeof(enter_stack));\
    stack_end = stack_ptr = (stack_type *)(enter_stack + 4096);\
    ebp_val = (long)stack_ptr;\
    for(i=1;i<=32;i++)\
       *--stack_ptr = i;\
    esp_val = (long)stack_ptr;\
    asm("mov " RSP ", %[esp_save]\n"\
        "mov " RBP ", %[ebp_save]\n"\
        "mov %[esp_val], " RSP "\n"\
        "mov %[ebp_val], " RBP "\n"\
        "enter" size " $8, $" #level "\n"\
        "mov " RSP ", %[esp_val]\n"\
        "mov " RBP ", %[ebp_val]\n"\
        "mov %[esp_save], " RSP "\n"\
        "mov %[ebp_save], " RBP "\n"\
        : [esp_save] "=r" (esp_save),\
        [ebp_save] "=r" (ebp_save),\
        [esp_val] "=r" (esp_val),\
        [ebp_val] "=r" (ebp_val)\
        :  "[esp_val]" (esp_val),\
        "[ebp_val]" (ebp_val));\
    printf("level=%d:\n", level);\
    printf("esp_val=" FMTLX "\n", esp_val - (long)stack_end);\
    printf("ebp_val=" FMTLX "\n", ebp_val - (long)stack_end);\
    for(ptr = (stack_type *)esp_val; ptr < stack_end; ptr++)\
        printf(FMTLX "\n", (long)ptr[0]);\
}
#endif

static void test_enter(void)
{
#if defined(__x86_64__)
    TEST_ENTER("q", uint64_t, 0);
    TEST_ENTER("q", uint64_t, 1);
    TEST_ENTER("q", uint64_t, 2);
    TEST_ENTER("q", uint64_t, 31);
#else
    TEST_ENTER("l", uint32_t, 0);
    TEST_ENTER("l", uint32_t, 1);
    TEST_ENTER("l", uint32_t, 2);
    TEST_ENTER("l", uint32_t, 31);
#endif

    TEST_ENTER("w", uint16_t, 0);
    TEST_ENTER("w", uint16_t, 1);
    TEST_ENTER("w", uint16_t, 2);
    TEST_ENTER("w", uint16_t, 31);
}

#define TEST_CONV_RAX(op)\
{\
    unsigned long a, r;\
    a = i2l(0x8234a6f8);\
    r = a;\
    asm volatile(#op : "=a" (r) : "0" (r));\
    printf("%-10s A=" FMTLX " R=" FMTLX "\n", #op, a, r);\
}

#define TEST_CONV_RAX_RDX(op)\
{\
    unsigned long a, d, r, rh;                   \
    a = i2l(0x8234a6f8);\
    d = i2l(0x8345a1f2);\
    r = a;\
    rh = d;\
    asm volatile(#op : "=a" (r), "=d" (rh) : "0" (r), "1" (rh));   \
    printf("%-10s A=" FMTLX " R=" FMTLX ":" FMTLX "\n", #op, a, r, rh);  \
}

void test_conv(void)
{
#ifdef TEST_CBW
    TEST_CONV_RAX(cbw);
#endif
#ifdef TEST_CWDE
    TEST_CONV_RAX(cwde);
#endif
#if defined(__x86_64__)
    TEST_CONV_RAX(cdqe);
#endif

    TEST_CONV_RAX_RDX(cwd);
    TEST_CONV_RAX_RDX(cdq);
#if defined(__x86_64__)
    TEST_CONV_RAX_RDX(cqo);
#endif

#ifdef TEST_BSWAP
    {
        unsigned long a, r;
        a = i2l(0x12345678);
        asm volatile("bswapl %k0" : "=r" (r) : "0" (a));
        printf("%-10s: A=" FMTLX " R=" FMTLX "\n", "bswapl", a, r);
    }
#endif
#if defined(__x86_64__)
    {
        unsigned long a, r;
        a = i2l(0x12345678);
        asm volatile("bswapq %0" : "=r" (r) : "0" (a));
        printf("%-10s: A=" FMTLX " R=" FMTLX "\n", "bswapq", a, r);
    }
#endif
}

extern void *__start_initcall;
extern void *__stop_initcall;


int _start()
{
    void **ptr;
    void (*func)(void);

    ptr = &__start_initcall;
    while (ptr != &__stop_initcall) {
        func = *ptr++;
        func();
    }
    test_bsx();
    test_mul();
    test_jcc();
#ifdef TEST_LOOP
    test_loop();
#endif
#ifdef TEST_FLOATS
    test_floats();
#endif
    test_xchg();
    test_string();
    test_misc();
    test_lea();
    test_enter();
    test_conv();
    
    // write a NUL terminator to output (yay, crutches)
    printf("%c", 0);
    
    // won't do anything on rusty_x86, but will terminate the process on linux (nice)
    exit(0);
    
    return 0;
}
