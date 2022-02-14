
section .data
; TODO

DoubleFormat:   dd 400h                 
                dd 0FFFFFC01h
                dd 53
                dd 11
                dd 64
                dd 3FFh

FloatFormat:    dd 80h                  ; maximum base 2 exponent (reserved for special values)
                dd 0FFFFFF81h           ; minimum base 2 exponent (reserved for denormals)
                dd 24                   ; bits of precision carried in the mantissa
                dd 8                    ; number of bits for exponent
                dd 32                   ; format width in bits
                dd 7Fh                  ; exponent bias

align 4
                
strgtold12_ret:     dd 0
double_ret:         dd 0
float_ret:          dd 0
long_double_ret:    dd 0
               
strgtold12_end:     dd 0

strgtold12_buf:     db 12 dup(0)
double_buf:         db 8  dup(0)
float_buf:          db 4  dup(0)
long_double_buf:    db 10 dup(0)

section .text
global _start
_start:         
;                 push    DWORD 0
;                 push    DWORD '3'
;                 push    DWORD '.'
;                 push    DWORD '1'
;                 push    DWORD '1'
;                 push    DWORD 5
;                 call fun
;                 jmp $
;                 fun:

                push    ebp
                mov     ebp, esp
                push    esi
                push    ebx
                mov     ebx, DWORD [ebp+8]
                lea     eax, [ebx+27]
                and     eax, -16
                sub     esp, eax
                lea     esi, [esp+15]
                and     esi, -16
                test    ebx, ebx
                je      .L8
                lea     edx, [ebp+12]
                mov     eax, esi
                add     ebx, esi
        .L9:
                mov     ecx, DWORD [edx]
                add     eax, 1
                add     edx, 4
                mov     BYTE [eax-1], cl
                cmp     eax, ebx
                jne     .L9
        .L8:
                sub     esp, 12
                
                push    DWORD 0
                push    DWORD 0
                push    DWORD 0
                push    DWORD 0
                push    esi
                push    strgtold12_end
                push    strgtold12_buf
                
                call    ___strgtold12
                mov     eax, strgtold12_ret
                
                ; right?
                add     esp, 28
                
                push    esi
                
                push    double_buf
                call    __atodbl
                mov     eax, double_ret

                ; this seems to always produce zero as a results
                ; TODO: why?
                pop     eax
                push    float_buf
                call    __atoflt
                mov     eax, float_ret
                
                ; TODO: find & include the long double conversion
                ; pop     eax
                ; push    long_double_buf
                ; call    __atoldbl
                ; mov     eax, long_double_ret
                            
                add     esp, 16
                lea     esp, [ebp-8]
                pop     ebx
                pop     esi
                pop     ebp
                ret

cbMultiByte dd 1
__decimal_point dd '.'

__isctype:
                int     3

___strgtold12:

                push    ebp
                mov     ebp, esp
                sub     esp, 5Ch
                push    ebx
                push    esi
                push    edi
                mov     edi, [ebp+16]
                lea     eax, [ebp-92]
                push    1
                mov     [ebp-12], eax
                xor     eax, eax
                pop     edx
                mov     [ebp-40], eax
                mov     [ebp-24], edx
                mov     [ebp-4], eax
                mov     [ebp-16], eax
                mov     [ebp-36], eax
                mov     [ebp-32], eax
                mov     [ebp-44], eax
                mov     [ebp-30h], eax
                mov     [ebp-28], eax
                mov     [ebp-8], eax
                mov     [ebp-20], eax
                mov     [ebp+16], edi

loc_405662:                             ; CODE XREF: ___strgtold12+52\u2193j
                mov     cl, [edi]
                cmp     cl, 20h ; ' '
                jz      loc_405678
                cmp     cl, 9
                jz      loc_405678
                cmp     cl, 0Ah
                jz      loc_405678
                cmp     cl, 0Dh
                jnz     loc_40567B

loc_405678:                             ; CODE XREF: ___strgtold12+40\u2191j
                                        ; ___strgtold12+45\u2191j ...
                inc     edi
                jmp     loc_405662
; ---------------------------------------------------------------------------

loc_40567B:                             ; CODE XREF: ___strgtold12+4F\u2191j
                push    4
                pop     esi

loc_40567E:                             ; CODE XREF: ___strgtold12+AE\u2193j
                                        ; ___strgtold12+B7\u2193j ...
                mov     bl, [edi]
                inc     edi
                cmp     eax, 0Bh        ; switch 12 cases
                ja      def_40568A      ; jumptable 0040568A default case, case 10
                jmp     [jpt_40568A+eax*4] ; switch jump
; ---------------------------------------------------------------------------

loc_405691:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     bl, 31h ; '1'   ; jumptable 0040568A case 0
                jl      loc_4056A2
                cmp     bl, 39h ; '9'
                jg      loc_4056A2

loc_40569B:                             ; CODE XREF: ___strgtold12+C4\u2193j
                                        ; ___strgtold12+118\u2193j
                push    3
                jmp     loc_4058BF
; ---------------------------------------------------------------------------

loc_4056A2:                             ; CODE XREF: ___strgtold12+6D\u2191j
                                        ; ___strgtold12+72\u2191j
                cmp     bl, [__decimal_point]
                jnz     loc_4056B1

loc_4056AA:                             ; CODE XREF: ___strgtold12+124\u2193j
                push    5
                jmp     loc_4058F7
; ---------------------------------------------------------------------------

loc_4056B1:                             ; CODE XREF: ___strgtold12+81\u2191j
                movsx   eax, bl
                sub     eax, 2Bh ; '+'
                jz      loc_4056D7
                dec     eax
                dec     eax
                jz      loc_4056CB
                sub     eax, 3
                jnz     loc_40599A
                jmp     loc_40575A
; ---------------------------------------------------------------------------

loc_4056CB:                             ; CODE XREF: ___strgtold12+94\u2191j
                push    2
                mov     dword [ebp-40], 8000h
                pop     eax
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4056D7:                             ; CODE XREF: ___strgtold12+90\u2191j
                and     dword [ebp-40], 0
                push    2
                pop     eax
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4056E0:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     bl, 31h ; '1'   ; jumptable 0040568A case 1
                mov     [ebp-16], edx
                jl      loc_4056ED
                cmp     bl, 39h ; '9'
                jle     loc_40569B

loc_4056ED:                             ; CODE XREF: ___strgtold12+BF\u2191j
                cmp     bl, [__decimal_point]
                jz      loc_4057B5
                cmp     bl, 2Bh ; '+'
                jz      loc_40572F
                cmp     bl, 2Dh ; '-'
                jz      loc_40572F
                cmp     bl, 30h ; '0'
                jz      loc_40575A

loc_405708:                             ; CODE XREF: ___strgtold12+207\u2193j
                cmp     bl, 43h ; 'C'
                jle     loc_40599A
                cmp     bl, 45h ; 'E'
                jle     loc_405728
                cmp     bl, 63h ; 'c'
                jle     loc_40599A
                cmp     bl, 65h ; 'e'
                jg      loc_40599A

loc_405728:                             ; CODE XREF: ___strgtold12+ED\u2191j
                push    6
                jmp     loc_4058F7
; ---------------------------------------------------------------------------

loc_40572F:                             ; CODE XREF: ___strgtold12+D5\u2191j
                                        ; ___strgtold12+DA\u2191j ...
                dec     edi
                push    0Bh
                jmp     loc_4058F7
; ---------------------------------------------------------------------------

loc_405737:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     bl, 31h ; '1'   ; jumptable 0040568A case 2
                jl      loc_405745
                cmp     bl, 39h ; '9'
                jle     loc_40569B

loc_405745:                             ; CODE XREF: ___strgtold12+113\u2191j
                cmp     bl, [__decimal_point]
                jz      loc_4056AA
                cmp     bl, 30h ; '0'
                jnz     loc_40590F

loc_40575A:                             ; CODE XREF: ___strgtold12+9F\u2191j
                                        ; ___strgtold12+DF\u2191j
                mov     eax, edx
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_405761:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                mov     [ebp-16], edx   ; jumptable 0040568A case 3

loc_405764:                             ; CODE XREF: ___strgtold12+184\u2193j
                cmp     DWORD [cbMultiByte], edx
                jle     loc_40577D
                movzx   eax, bl
                push    esi             ; int
                push    eax             ; int
                call    __isctype
                pop     ecx
                pop     ecx
                push    1
                pop     edx
                jmp     loc_40578B
; ---------------------------------------------------------------------------

loc_40577D:                             ; CODE XREF: ___strgtold12+143\u2191j
                mov     ecx, __pctype
                movzx   eax, bl
                mov     al, [ecx+eax*2]
                and     eax, esi

loc_40578B:                             ; CODE XREF: ___strgtold12+154\u2191j
                test    eax, eax
                jz      loc_4057AD
                cmp     dword [ebp-4], 19h
                jnb     loc_4057A5
                mov     eax, [ebp-12]
                inc     dword [ebp-4]
                sub     bl, 30h ; '0'
                inc     dword [ebp-12]
                mov     [eax], bl
                jmp     loc_4057A8
; ---------------------------------------------------------------------------

loc_4057A5:                             ; CODE XREF: ___strgtold12+16C\u2191j
                inc     dword [ebp-8]

loc_4057A8:                             ; CODE XREF: ___strgtold12+17C\u2191j
                mov     bl, [edi]
                inc     edi
                jmp     loc_405764
; ---------------------------------------------------------------------------

loc_4057AD:                             ; CODE XREF: ___strgtold12+166\u2191j
                cmp     bl, [__decimal_point]
                jnz     loc_40581C

loc_4057B5:                             ; CODE XREF: ___strgtold12+CC\u2191j
                mov     eax, esi
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4057BC:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     dword [ebp-4], 0 ; jumptable 0040568A case 4
                mov     [ebp-16], edx
                mov     [ebp-36], edx
                jnz     loc_4057D5

loc_4057C8:                             ; CODE XREF: ___strgtold12+1AC\u2193j
                cmp     bl, 30h ; '0'
                jnz     loc_4057D5
                dec     dword [ebp-8]
                mov     bl, [edi]
                inc     edi
                jmp     loc_4057C8
; ---------------------------------------------------------------------------

loc_4057D5:                             ; CODE XREF: ___strgtold12+19F\u2191j
                                        ; ___strgtold12+1A4\u2191j ...
                cmp     DWORD [cbMultiByte], edx
                jle     loc_4057EE
                movzx   eax, bl
                push    esi             ; int
                push    eax             ; int
                call    __isctype
                pop     ecx
                pop     ecx
                push    1
                pop     edx
                jmp     loc_4057FC
; ---------------------------------------------------------------------------

loc_4057EE:                             ; CODE XREF: ___strgtold12+1B4\u2191j
                mov     ecx, __pctype
                movzx   eax, bl
                mov     al, [ecx+eax*2]
                and     eax, esi

loc_4057FC:                             ; CODE XREF: ___strgtold12+1C5\u2191j
                test    eax, eax
                jz      loc_40581C
                cmp     dword [ebp-4], 19h
                jnb     loc_405817
                mov     eax, [ebp-12]
                inc     dword [ebp-4]
                sub     bl, 30h ; '0'
                inc     dword [ebp-12]
                dec     dword [ebp-8]
                mov     [eax], bl

loc_405817:                             ; CODE XREF: ___strgtold12+1DD\u2191j
                mov     bl, [edi]
                inc     edi
                jmp     loc_4057D5
; ---------------------------------------------------------------------------

loc_40581C:                             ; CODE XREF: ___strgtold12+18C\u2191j
                                        ; ___strgtold12+1D7\u2191j
                cmp     bl, 2Bh ; '+'
                jz      loc_40572F
                cmp     bl, 2Dh ; '-'
                jz      loc_40572F
                jmp     loc_405708
; ---------------------------------------------------------------------------

loc_405833:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     DWORD [cbMultiByte], edx ; jumptable 0040568A case 5
                mov     [ebp-36], edx
                jle     loc_40584F
                movzx   eax, bl
                push    esi             ; int
                push    eax             ; int
                call    __isctype
                pop     ecx
                pop     ecx
                push    1
                pop     edx
                jmp     loc_40585D
; ---------------------------------------------------------------------------

loc_40584F:                             ; CODE XREF: ___strgtold12+215\u2191j
                mov     ecx, __pctype
                movzx   eax, bl
                mov     al, [ecx+eax*2]
                and     eax, esi

loc_40585D:                             ; CODE XREF: ___strgtold12+226\u2191j
                test    eax, eax
                jz      loc_40590F
                mov     eax, esi
                jmp     loc_4058C0
; ---------------------------------------------------------------------------

loc_405869:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                lea     ecx, [edi-2]    ; jumptable 0040568A case 6
                cmp     bl, 31h ; '1'
                mov     [ebp+16], ecx
                jl      loc_405879
                cmp     bl, 39h ; '9'
                jle     loc_4058BD

loc_405879:                             ; CODE XREF: ___strgtold12+24B\u2191j
                movsx   eax, bl
                sub     eax, 2Bh ; '+'
                jz      loc_4058F5
                dec     eax
                dec     eax
                jz      loc_4058E9
                sub     eax, 3
                jnz     loc_40599D

loc_40588E:                             ; CODE XREF: ___strgtold12+2A4\u2193j
                push    8
                jmp     loc_4058F7
; ---------------------------------------------------------------------------

loc_405892:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                mov     [ebp-32], edx   ; jumptable 0040568A case 8

loc_405895:                             ; CODE XREF: ___strgtold12+276\u2193j
                cmp     bl, 30h ; '0'
                jnz     loc_40589F
                mov     bl, [edi]
                inc     edi
                jmp     loc_405895
; ---------------------------------------------------------------------------

loc_40589F:                             ; CODE XREF: ___strgtold12+271\u2191j
                cmp     bl, 31h ; '1'
                jl      loc_40599A
                cmp     bl, 39h ; '9'
                jg      loc_40599A
                jmp     loc_4058BD
; ---------------------------------------------------------------------------

loc_4058B3:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     bl, 31h ; '1'   ; jumptable 0040568A case 7
                jl      loc_4058C6
                cmp     bl, 39h ; '9'
                jg      loc_4058C6

loc_4058BD:                             ; CODE XREF: ___strgtold12+250\u2191j
                                        ; ___strgtold12+28A\u2191j
                push    9

loc_4058BF:                             ; CODE XREF: ___strgtold12+76\u2191j
                pop     eax

loc_4058C0:                             ; CODE XREF: ___strgtold12+240\u2191j
                dec     edi
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4058C6:                             ; CODE XREF: ___strgtold12+28F\u2191j
                                        ; ___strgtold12+294\u2191j
                cmp     bl, 30h ; '0'
                jnz     loc_40590F
                jmp     loc_40588E
; ---------------------------------------------------------------------------

loc_4058CD:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                cmp     dword [ebp+32], 0 ; jumptable 0040568A case 11
                jz      loc_4058FD
                movsx   eax, bl
                lea     ecx, [edi-1]
                sub     eax, 2Bh ; '+'
                mov     [ebp+16], ecx
                jz      loc_4058F5
                dec     eax
                dec     eax
                jnz     loc_40599D

loc_4058E9:                             ; CODE XREF: ___strgtold12+25C\u2191j
                or      dword [ebp-24], 0FFFFFFFFh
                push    7
                pop     eax
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4058F5:                             ; CODE XREF: ___strgtold12+258\u2191j
                                        ; ___strgtold12+2B8\u2191j
                push    7

loc_4058F7:                             ; CODE XREF: ___strgtold12+85\u2191j
                                        ; ___strgtold12+103\u2191j ...
                pop     eax
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_4058FD:                             ; CODE XREF: ___strgtold12+2AA\u2191j
                push    0Ah
                dec     edi
                pop     eax

def_40568A:                             ; CODE XREF: ___strgtold12+5D\u2191j
                                        ; ___strgtold12+63\u2191j
                                        ; DATA XREF: ...
                cmp     eax, 0Ah        ; jumptable 0040568A default case, case 10
                jz      loc_40599F
                jmp     loc_40567E
; ---------------------------------------------------------------------------

loc_40590F:                             ; CODE XREF: ___strgtold12+12D\u2191j
                                        ; ___strgtold12+238\u2191j ...
                mov     edi, [ebp+16]
                jmp     loc_40599F
; ---------------------------------------------------------------------------

loc_405917:                             ; CODE XREF: ___strgtold12+63\u2191j
                                        ; DATA XREF: .text:jpt_40568A\u2193o
                mov     dword [ebp-32], 1 ; jumptable 0040568A case 9
                xor     esi, esi

loc_405920:                             ; CODE XREF: ___strgtold12+339\u2193j
                cmp     DWORD [cbMultiByte], 1
                jle     loc_405938
                movzx   eax, bl
                push    4               ; int
                push    eax             ; int
                call    __isctype
                pop     ecx
                pop     ecx
                jmp     loc_405947
; ---------------------------------------------------------------------------

loc_405938:                             ; CODE XREF: ___strgtold12+300\u2191j
                mov     ecx, __pctype
                movzx   eax, bl
                mov     al, [ecx+eax*2]
                and     eax, 4

loc_405947:                             ; CODE XREF: ___strgtold12+30F\u2191j
                test    eax, eax
                jz      loc_405967
                movsx   ecx, bl
                lea     eax, [esi+esi*4]
                lea     esi, [ecx+eax*2-30h]
                cmp     esi, 1450h
                jg      loc_405962
                mov     bl, [edi]
                inc     edi
                jmp     loc_405920
; ---------------------------------------------------------------------------

loc_405962:                             ; CODE XREF: ___strgtold12+334\u2191j
                mov     esi, 1451h

loc_405967:                             ; CODE XREF: ___strgtold12+322\u2191j
                mov     [ebp-28], esi

loc_40596A:                             ; CODE XREF: ___strgtold12+371\u2193j
                cmp     DWORD [cbMultiByte], 1
                jle     loc_405982
                movzx   eax, bl
                push    4               ; int
                push    eax             ; int
                call    __isctype
                pop     ecx
                pop     ecx
                jmp     loc_405991
; ---------------------------------------------------------------------------

loc_405982:                             ; CODE XREF: ___strgtold12+34A\u2191j
                mov     ecx, __pctype
                movzx   eax, bl
                mov     al, [ecx+eax*2]
                and     eax, 4

loc_405991:                             ; CODE XREF: ___strgtold12+359\u2191j
                test    eax, eax
                jz      loc_40599A
                mov     bl, [edi]
                inc     edi
                jmp     loc_40596A
; ---------------------------------------------------------------------------

loc_40599A:                             ; CODE XREF: ___strgtold12+99\u2191j
                                        ; ___strgtold12+E4\u2191j ...
                dec     edi
                jmp     loc_40599F
; ---------------------------------------------------------------------------

loc_40599D:                             ; CODE XREF: ___strgtold12+261\u2191j
                                        ; ___strgtold12+2BC\u2191j
                mov     edi, ecx

loc_40599F:                             ; CODE XREF: ___strgtold12+2DD\u2191j
                                        ; ___strgtold12+2EB\u2191j ...
                mov     eax, [ebp+12]
                cmp     dword [ebp-16], 0
                mov     [eax], edi
                jz      loc_405A87
                push    18h
                pop     eax
                cmp     [ebp-4], eax
                jbe     loc_4059CB
                cmp     byte [ebp-69], 5
                jl      loc_4059BF
                inc     byte [ebp-69]

loc_4059BF:                             ; CODE XREF: ___strgtold12+393\u2191j
                mov     [ebp-4], eax
                mov     eax, [ebp-12]
                dec     eax
                inc     dword [ebp-8]
                jmp     loc_4059CE
; ---------------------------------------------------------------------------

loc_4059CB:                             ; CODE XREF: ___strgtold12+38D\u2191j
                mov     eax, [ebp-12]

loc_4059CE:                             ; CODE XREF: ___strgtold12+3A2\u2191j
                cmp     dword [ebp-4], 0
                jbe     loc_405A7D

loc_4059D8:                             ; CODE XREF: ___strgtold12+3BD\u2193j
                dec     eax
                cmp     byte [eax], 0
                jnz     loc_4059E6
                dec     dword [ebp-4]
                inc     dword [ebp-8]
                jmp     loc_4059D8
; ---------------------------------------------------------------------------

loc_4059E6:                             ; CODE XREF: ___strgtold12+3B5\u2191j
                lea     eax, [ebp-64]
                push    eax
                lea     eax, [ebp-92]
                push    dword [ebp-4]
                push    eax
                call    ___mtold12
                mov     eax, [ebp-28]
                xor     ecx, ecx
                add     esp, 0Ch
                cmp     [ebp-24], ecx
                jge     loc_405A05
                neg     eax

loc_405A05:                             ; CODE XREF: ___strgtold12+3DA\u2191j
                add     eax, [ebp-8]
                cmp     [ebp-32], ecx
                jnz     loc_405A10
                add     eax, [ebp+24]

loc_405A10:                             ; CODE XREF: ___strgtold12+3E4\u2191j
                cmp     [ebp-36], ecx
                jnz     loc_405A18
                sub     eax, [ebp+28]

loc_405A18:                             ; CODE XREF: ___strgtold12+3EC\u2191j
                cmp     eax, 1450h
                jle     loc_405A4F
                mov     dword [ebp-44], 1

loc_405A26:                             ; CODE XREF: ___strgtold12+436\u2193j
                mov     ebx, [ebp+16]
                mov     esi, [ebp+16]
                mov     eax, [ebp+16]
                mov     edx, [ebp+16]

loc_405A32:                             ; CODE XREF: ___strgtold12+454\u2193j
                                        ; ___strgtold12+45E\u2193j
                cmp     dword [ebp-44], 0
                jz      loc_405A98
                xor     ebx, ebx
                mov     eax, 7FFFh
                mov     esi, 80000000h
                xor     edx, edx
                mov     dword [ebp-20], 2
                jmp     loc_405AAD
; ---------------------------------------------------------------------------

loc_405A4F:                             ; CODE XREF: ___strgtold12+3F6\u2191j
                cmp     eax, 0FFFFEBB0h
                jge     loc_405A5F
                mov     dword [ebp-48], 1
                jmp     loc_405A26
; ---------------------------------------------------------------------------

loc_405A5F:                             ; CODE XREF: ___strgtold12+42D\u2191j
                push    dword [ebp+20]
                push    eax
                lea     eax, [ebp-64]
                push    eax
                call    ___multtenpow12
                mov     edx, [ebp-64]
                mov     ebx, [ebp-62]
                mov     esi, [ebp-58]
                mov     eax, [ebp-54]
                add     esp, 0Ch
                jmp     loc_405A32
; ---------------------------------------------------------------------------

loc_405A7D:                             ; CODE XREF: ___strgtold12+3AB\u2191j
                xor     edx, edx
                xor     eax, eax
                xor     esi, esi
                xor     ebx, ebx
                jmp     loc_405A32
; ---------------------------------------------------------------------------

loc_405A87:                             ; CODE XREF: ___strgtold12+381\u2191j
                xor     edx, edx
                xor     eax, eax
                xor     esi, esi
                xor     ebx, ebx
                mov     dword [ebp-20], 4
                jmp     loc_405AAD
; ---------------------------------------------------------------------------

loc_405A98:                             ; CODE XREF: ___strgtold12+40F\u2191j
                cmp     dword [ebp-48], 0
                jz      loc_405AAD
                xor     edx, edx
                xor     eax, eax
                xor     esi, esi
                xor     ebx, ebx
                mov     dword [ebp-20], 1

loc_405AAD:                             ; CODE XREF: ___strgtold12+426\u2191j
                                        ; ___strgtold12+46F\u2191j ...
                mov     ecx, [ebp+8]
                or      eax, [ebp-40]
                pop     edi
                mov     [ecx+6], esi
                mov     [ecx+2], ebx
                mov     [ecx+0Ah], ax
                mov     eax, [ebp-20]
                pop     esi
                mov     [ecx], dx
                pop     ebx
                leave
                retn

jpt_40568A      dd loc_405691    ; DATA XREF: ___strgtold12+63\u2191r
                dd loc_4056E0    ; jump table for switch statement
                dd loc_405737
                dd loc_405761
                dd loc_4057BC
                dd loc_405833
                dd loc_405869
                dd loc_4058B3
                dd loc_405892
                dd loc_405917
                dd def_40568A
                dd loc_4058CD

___mtold12:
                push    ebp
                mov     ebp, esp
                sub     esp, 10h
                mov     eax, [ebp+12]
                push    ebx
                mov     ebx, [ebp+16]
                xor     edx, edx
                cmp     eax, edx
                push    esi
                mov     dword [ebp-4], 404Eh
                mov     [ebx], edx
                mov     [ebx+4], edx
                mov     [ebx+8], edx
                jbe     short loc_4055D4
                push    edi
                mov     [ebp+16], eax

loc_405587:                             ; CODE XREF: ___mtold12+6F\u2193j
                mov     esi, ebx
                lea     edi, [ebp-16]
                movsd
                movsd
                push    ebx
                movsd
                call    ___shl_12
                push    ebx
                call    ___shl_12
                lea     eax, [ebp-16]
                push    eax
                push    ebx
                call    ___add_12
                push    ebx
                call    ___shl_12
                mov     eax, [ebp+8]
                and     dword [ebp-12], 0
                and     dword [ebp-8], 0
                movsx   eax, byte [eax]
                mov     [ebp-16], eax
                lea     eax, [ebp-16]
                push    eax
                push    ebx
                call    ___add_12
                add     esp, 1Ch
                inc     dword [ebp+8]
                dec     dword [ebp+16]
                jnz     short loc_405587
                xor     edx, edx
                pop     edi

loc_4055D4:                             ; CODE XREF: ___mtold12+21\u2191j
                                        ; ___mtold12+9F\u2193j
                cmp     [ebx+8], edx
                jnz     short loc_405601
                mov     ecx, [ebx+4]
                mov     eax, ecx
                shr     eax, 10h
                mov     [ebx+8], eax
                mov     eax, [ebx]
                mov     esi, eax
                shr     esi, 10h
                shl     ecx, 10h
                or      esi, ecx
                shl     eax, 10h
                add     dword [ebp-4], 0FFF0h
                mov     [ebx+4], esi
                mov     [ebx], eax
                jmp     short loc_4055D4
; ---------------------------------------------------------------------------

loc_405601:                             ; CODE XREF: ___mtold12+77\u2191j
                mov     esi, 8000h

loc_405606:                             ; CODE XREF: ___mtold12+B9\u2193j
                test    [ebx+8], esi
                jnz     short loc_40561B
                push    ebx
                call    ___shl_12
                add     dword [ebp-4], 0FFFFh
                pop     ecx
                jmp     short loc_405606
; ---------------------------------------------------------------------------

loc_40561B:                             ; CODE XREF: ___mtold12+A9\u2191j
                mov     ax, [ebp-4]
                pop     esi
                mov     [ebx+0Ah], ax
                pop     ebx
                leave
                retn


___shl_12:
                mov     eax, [esp+4]
                push    esi
                push    edi
                mov     esi, [eax]
                mov     edi, [eax+4]
                mov     ecx, esi
                add     esi, esi
                mov     [eax], esi
                lea     esi, [edi+edi]
                shr     ecx, 1Fh
                or      esi, ecx
                mov     ecx, [eax+8]
                mov     edx, edi
                mov     [eax+4], esi
                shr     edx, 1Fh
                shl     ecx, 1
                or      ecx, edx
                pop     edi
                mov     [eax+8], ecx
                pop     esi
                retn
                
__pctype        equ __ctype+2

; unsigned __int16 _ctype[]
__ctype         dw      0,   20h,   20h,   20h,   20h,   20h,   20h,   20h
                                        ; DATA XREF: _x_ismbbtype+18\u2191r
                                        ; .data:__pctype\u2191o ...
                dw    20h,   20h,   28h,   28h,   28h,   28h,   28h,   20h
                dw    20h,   20h,   20h,   20h,   20h,   20h,   20h,   20h
                dw    20h,   20h,   20h,   20h,   20h,   20h,   20h,   20h
                dw    20h,   48h,   10h,   10h,   10h,   10h,   10h,   10h
                dw    10h,   10h,   10h,   10h,   10h,   10h,   10h,   10h
                dw    10h,   84h,   84h,   84h,   84h,   84h,   84h,   84h
                dw    84h,   84h,   84h,   10h,   10h,   10h,   10h,   10h
                dw    10h,   10h,   81h,   81h,   81h,   81h,   81h,   81h
                dw      1,     1,     1,     1,     1,     1,     1,     1
                dw      1,     1,     1,     1,     1,     1,     1,     1
                dw      1,     1,     1,     1,   10h,   10h,   10h,   10h
                dw    10h,   10h,   82h,   82h,   82h,   82h,   82h,   82h
                dw      2,     2,     2,     2,     2,     2,     2,     2
                dw      2,     2,     2,     2,     2,     2,     2,     2
                dw      2,     2,     2,     2,   10h,   10h,   10h,   10h
                dw    20h,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0,     0,     0,     0,     0,     0,     0,     0
                dw      0
                db    0
                db    0

__atodbl:
                push    ebp
                mov     ebp, esp
                sub     esp, 0x0C
                xor     eax, eax
                push    eax
                push    eax
                push    eax
                push    eax
                push    DWORD [ebp+12]
                lea     eax, [ebp+12]
                push    eax
                lea     eax, [ebp-12]
                push    eax
                call    ___strgtold12
                push    DWORD [ebp+8]
                lea     eax, [ebp-12]
                push    eax
                call    __ld12tod
                add     esp, 0x24
                leave
                retn

__atoflt:
                push    ebp
                mov     ebp, esp
                sub     esp, 0x0C
                xor     eax, eax
                push    eax
                push    eax
                push    eax
                push    eax
                push    DWORD [ebp+12]
                lea     eax, [ebp+12]
                push    eax
                lea     eax, [ebp-12]
                push    eax
                call    ___strgtold12
                push    DWORD [ebp+8]
                lea     eax, [ebp-12]
                push    eax
                call    __ld12tof
                add     esp, 0x24
                leave
                retn

__ld12tod:
                push    DWORD DoubleFormat
                push    DWORD [esp+12]
                push    DWORD [esp+12]
                call    __ld12cvt
                add     esp, 0x0C
                retn

__ld12tof:
                push    DWORD FloatFormat
                push    DWORD [esp+12]
                push    DWORD [esp+12]
                call    __ld12cvt
                add     esp, 0x0C
                retn


__ld12cvt:
                push    ebp
                mov     ebp, esp
                sub     esp, 0x18
                mov     eax, [ebp+8]
                push    ebx
                push    esi
                push    edi
                movzx   ecx, WORD [eax+0x0A]
                mov     ebx, ecx
                and     ecx, 0x8000
                mov     [ebp+8], ecx
                mov     ecx, [eax+6]
                mov     [ebp-12], ecx
                mov     ecx, [eax+2]
                movzx   eax, WORD [eax]
                mov     edi, [ebp+16]
                and     ebx, 0x7FFF
                sub     ebx, 0x3FFF
                mov     [ebp-8], ecx
                shl     eax, 0x10
                cmp     ebx, -0x3FFF
                mov     [ebp-4], eax
                jnz     loc_403215
                lea     eax, [ebp-12]
                xor     esi, esi
                push    eax
                call    __IsZeroMan
                test    eax, eax
                pop     ecx
                jnz     loc_4032D4
                lea     eax, [ebp-12]
                push    eax
                call    __FillZeroMan
                pop     ecx
      loc_40320D:
                push    2
      loc_40320F:
                pop     eax
                jmp     loc_4032D6
      loc_403215:
                lea     eax, [ebp-12]
                push    eax
                lea     eax, [ebp-24]
                push    eax
                call    __CopyMan
                push    DWORD [edi+8]
                lea     eax, [ebp-12]
                push    eax
                call    __RoundMan
                add     esp, 0x10
                test    eax, eax
                jz      loc_403236
                inc     ebx
      loc_403236:
                mov     eax, [edi+4]
                mov     ecx, eax
                sub     ecx, [edi+8]
                cmp     ebx, ecx
                jge     loc_40324E
                lea     eax, [ebp-12]
                push    eax
                call    __FillZeroMan
                pop     ecx
                jmp     loc_40328A
      loc_40324E:
                cmp     ebx, eax
                jg      loc_403291
                sub     eax, ebx
                mov     esi, eax
                lea     eax, [ebp-24]
                push    eax
                lea     eax, [ebp-12]
                push    eax
                call    __CopyMan
                lea     eax, [ebp-12]
                push    esi
                push    eax
                call    __ShrMan
                push    DWORD [edi+8]
                lea     eax, [ebp-12]
                push    eax
                call    __RoundMan
                mov     eax, [edi+12]
                inc     eax
                push    eax
                lea     eax, [ebp-12]
                push    eax
                call    __ShrMan
                add     esp, 0x20
      loc_40328A:
                xor     esi, esi
                jmp     loc_40320D
      loc_403291:
                cmp     ebx, [edi]
                jl      loc_4032BD
                lea     eax, [ebp-12]
                push    eax
                call    __FillZeroMan
                push    DWORD [edi+0x0C]
                or      BYTE [ebp-9], -0x80
                lea     eax, [ebp-12]
                push    eax
                call    __ShrMan
                mov     esi, [edi+0x14]
                add     esp, 0x0C
                add     esi, [edi]
                push    1
                jmp     loc_40320F
      loc_4032BD:
                push    DWORD [edi+0x0C]
                mov     esi, [edi+0x14]
                and     BYTE [ebp-9], 0x7F
                lea     eax, [ebp-12]
                push    eax
                add     esi, ebx
                call    __ShrMan
                pop     ecx
                pop     ecx
      loc_4032D4:
                xor     eax, eax
      loc_4032D6:
                push    0x1F
                pop     ecx
                sub     ecx, [edi+12]
                mov     edi, [edi+16]
                shl     esi, cl
                mov     ecx, [ebp+8]
                neg     ecx
                sbb     ecx, ecx
                and     ecx, -0x80000000
                or      esi, ecx
                or      esi, [ebp-12]
                cmp     edi, 64
                jnz     loc_403305
                mov     ecx, [ebp+12]
                mov     edx, [ebp-8]
                mov     [ecx+4], esi
                mov     [ecx], edx
                jmp     loc_40330F
      loc_403305:
                cmp     edi, 0x20
                jnz     loc_40330F
                mov     ecx, [ebp+12]
                mov     [ecx], esi
      loc_40330F:
                pop     edi
                pop     esi
                pop     ebx
                leave
                retn
 
__ZeroTail:
                mov     eax, [esp+8]
                push    esi
                push    0x20
                cdq
                pop     ecx
                idiv    ecx
                push    0x1F
                mov     esi, eax
                mov     eax, [esp+8+8]
                cdq
                idiv    ecx
                pop     ecx
                mov     eax, [esp+4+4]
                sub     ecx, edx
                or      edx, -1
                shl     edx, cl
                not     edx
                test    [eax+esi*4], edx
                jnz     loc_402FF3
                inc     esi
                cmp     esi, 3
                jge     loc_402FEE
                lea     eax, [eax+esi*4]
      loc_402FE0:
                cmp     DWORD [eax], 0
                jnz     loc_402FF3
                inc     esi
                add     eax, 4
                cmp     esi, 3
                jl      loc_402FE0
      loc_402FEE:
                push    1
                pop     eax
                pop     esi
                retn
      loc_402FF3:
                xor     eax, eax
                pop     esi
                retn

__CopyMan:
                mov     eax, [esp+8]
                mov     ecx, [esp+4]
                push    esi
                push    3
                sub     ecx, eax
                pop     edx
      loc_4030E7:
                mov     esi, [eax]
                mov     [ecx+eax], esi
                add     eax, 4
                dec     edx
                jnz     loc_4030E7
                pop     esi
                retn

__RoundMan:
                push    ebp
                mov     ebp, esp
                push    ecx
                push    ecx
                mov     eax, [ebp+12]
                push    ebx
                push    esi
                push    edi
                lea     edi, [eax-1]
                push    0x20
                pop     ecx
                and     DWORD [ebp+12], 0
                lea     ebx, [edi+1]
                push    0x20
                mov     eax, ebx
                pop     esi
                cdq
                idiv    ecx
                push    0x1F
                mov     ecx, eax
                mov     eax, ebx
                cdq
                idiv    esi
                mov     eax, [ebp+8]
                pop     esi
                push    1
                mov     [ebp-8], ecx
                lea     eax, [eax+ecx*4]
                mov     [ebp+12], eax
                sub     esi, edx
                pop     edx
                mov     ecx, esi
                shl     edx, cl
                test    [eax], edx
                jz      loc_4030B1
                inc     ebx
                push    ebx
                push    DWORD [ebp+8]
                call    __ZeroTail
                pop     ecx
                test    eax, eax
                pop     ecx
                jnz     loc_4030AE
                push    edi
                push    DWORD [ebp+8]
                call    __IncMan
                pop     ecx
                mov     [ebp-4], eax
                pop     ecx
      loc_4030AE:
                mov     eax, [ebp+12]
      loc_4030B1:
                or      edx, -1
                mov     ecx, esi
                shl     edx, cl
                push    3
                pop     ecx
                and     [eax], edx
                mov     eax, [ebp-8]
                inc     eax
                cmp     eax, ecx
                jge     loc_4030D1
                mov     edx, [ebp+8]
                sub     ecx, eax
                lea     edi, [edx+eax*4]
                xor     eax, eax
                rep stosd
      loc_4030D1:
                mov     eax, [ebp-4]
                pop     edi
                pop     esi
                pop     ebx
                leave
                retn

__IncMan:
                mov     eax, [esp+8]
                push    ebx
                push    esi
                push    edi
                push    0x20
                mov     ebx, [esp+0x10+4]
                cdq
                pop     ecx
                idiv    ecx
                mov     esi, eax
                mov     eax, [esp+0x0C+8]
                cdq
                idiv    ecx
                lea     edi, [ebx+esi*4]
                push    edi
                push    0x1F
                pop     ecx
                push    1
                pop     eax
                sub     ecx, edx
                shl     eax, cl
                push    eax
                push    DWORD [edi]
                call    ___addl
                add     esp, 0x0C
                dec     esi
                js      loc_403049
                lea     edi, [ebx+esi*4]
      loc_403030:
                test    eax, eax
                jz      loc_403049
                push    edi
                push    1
                push    DWORD [edi]
                call    ___addl
                add     esp, 0x0C
                dec     esi
                sub     edi, 4
                test    esi, esi
                jge     loc_403030
      loc_403049:
                pop     edi
                pop     esi
                pop     ebx
                retn


___add_12:
                push    esi
                mov     esi, [esp+4+4]
                push    edi
                mov     edi, [esp+8+8]
                push    esi
                push    dword [edi]
                push    dword [esi]
                call    ___addl
                add     esp, 0Ch
                test    eax, eax
                jz      loc_4054D9
                lea     eax, [esi+4]
                push    eax
                push    1
                push    dword [eax]
                call    ___addl
                add     esp, 0Ch
                test    eax, eax
                jz      loc_4054D9
                inc     dword [esi+8]
loc_4054D9:                             
                lea     eax, [esi+4]
                push    eax
                push    dword [edi+4]
                push    dword [eax]
                call    ___addl
                add     esp, 0Ch
                test    eax, eax
                jz      loc_4054F1
                inc     dword [esi+8]
loc_4054F1:                             ; CODE XREF: ___add_12+45↑j
                lea     eax, [esi+8]
                push    eax
                push    dword [edi+8]
                push    dword [eax]
                call    ___addl
                add     esp, 0Ch
                pop     edi
                pop     esi
                retn

___addl:
                mov     edx, [esp+4]
                push    esi
                mov     esi, [esp+12]
                xor     eax, eax
                lea     ecx, [edx+esi]
                cmp     ecx, edx
                jb      loc_40549C
                cmp     ecx, esi
                jnb     loc_40549F
loc_40549C:
                push    1
                pop     eax
loc_40549F:
                mov     edx, [esp+16]
                pop     esi
                mov     [edx], ecx
                retn
___shr_12:
                mov     eax, [esp+4]
                push    esi
                push    edi
                mov     edx, [eax+8]
                mov     ecx, [eax+4]
                mov     esi, edx
                mov     edi, ecx
                shl     esi, 1Fh
                shr     ecx, 1
                or      ecx, esi
                mov     [eax+4], ecx
                mov     ecx, [eax]
                shl     edi, 1Fh
                shr     ecx, 1
                shr     edx, 1
                or      ecx, edi
                pop     edi
                mov     [eax+8], edx
                mov     [eax], ecx
                pop     esi
                retn
; 
___ld12mul:
                push    ebp
                mov     ebp, esp
                sub     esp, 24h
                push    ebx
                mov     ebx, [ebp+12]
                push    esi
                mov     esi, [ebp+8]
                mov     cx, [ebx+0Ah]
                xor     eax, eax
                push    edi
                mov     [ebp-20], eax
                mov     [ebp-36], eax
                mov     [ebp-32], eax
                mov     [ebp-28], eax
                mov     ax, [esi+0Ah]
                mov     edi, ecx
                mov     edx, 7FFFh
                xor     edi, eax
                and     eax, edx
                and     ecx, edx
                and     edi, 8000h
                cmp     ax, 7FFFh
                lea     edx, [ecx+eax]
                mov     [ebp+8], edx
                jnb     loc_4062DF
                cmp     cx, 7FFFh
                jnb     loc_4062DF
                cmp     dx, 0BFFDh
                ja      loc_4062DF
                cmp     dx, 3FBFh
                ja      loc_406148
                xor     eax, eax
                jmp     loc_406182
loc_406148:                             ; CODE XREF: ___ld12mul+63↑j
                test    ax, ax
                mov     edx, 7FFFFFFFh
                jnz     loc_40616A
                inc     dword [ebp+8]
                test    [esi+8], edx
                jnz     loc_40616A
                xor     eax, eax
                cmp     [esi+4], eax
                jnz     loc_40616C
                cmp     [esi], eax
                jnz     loc_40616C
                jmp     loc_4062D9
loc_40616A:                             ; CODE XREF: ___ld12mul+71↑j
                xor     eax, eax
loc_40616C:                             ; CODE XREF: ___ld12mul+80↑j
                cmp     cx, ax
                jnz     loc_40618F
                inc     dword [ebp+8]
                test    [ebx+8], edx
                jnz     loc_40618F
                cmp     [ebx+4], eax
                jnz     loc_40618F
                cmp     [ebx], eax
                jnz     loc_40618F
loc_406182:                             ; CODE XREF: ___ld12mul+67↑j
                mov     [esi+8], eax
                mov     [esi+4], eax
                mov     [esi], eax
                jmp     loc_4062FA
loc_40618F:                             ; CODE XREF: ___ld12mul+90↑j
                mov     [ebp-16], eax
                lea     eax, [ebp-32]
                mov     [ebp-4], eax
                mov     dword [ebp+12], 5
loc_40619F:                             ; CODE XREF: ___ld12mul+122↓j
                mov     eax, [ebp-16]
                add     eax, eax
                cmp     dword [ebp+12], 0
                jle     loc_4061F3
                add     eax, esi
                lea     ecx, [ebx+8]
                mov     [ebp-8], eax
                mov     eax, [ebp+12]
                mov     [ebp-12], ecx
                mov     [ebp-24], eax
loc_4061BB:                             ; CODE XREF: ___ld12mul+112↓j
                mov     eax, [ebp-8]
                mov     ecx, [ebp-12]
                movzx   eax, word [eax]
                movzx   ecx, word [ecx]
                imul    eax, ecx
                mov     ecx, [ebp-4]
                add     ecx, 0FFFFFFFCh
                push    ecx
                push    eax
                push    dword [ecx]
                call    ___addl
                add     esp, 0Ch
                test    eax, eax
                jz      loc_4061E6
                mov     eax, [ebp-4]
                inc     word [eax]
loc_4061E6:                             ; CODE XREF: ___ld12mul+FF↑j
                add     dword [ebp-8], 2
                sub     dword [ebp-12], 2
                dec     dword [ebp-24]
                jnz     loc_4061BB
loc_4061F3:                             ; CODE XREF: ___ld12mul+C9↑j
                add     dword [ebp-4], 2
                inc     dword [ebp-16]
                dec     dword [ebp+12]
                cmp     dword [ebp+12], 0
                jg      loc_40619F
                add     dword [ebp+8], 0C002h
                cmp     word [ebp+8], 0
                jle     loc_406236
loc_406211:                             ; CODE XREF: ___ld12mul+14E↓j
                test    byte [ebp-25], 80h
                jnz     loc_40622F
                lea     eax, [ebp-36]
                push    eax
                call    ___shl_12
                add     dword [ebp+8], 0FFFFh
                pop     ecx
                cmp     word [ebp+8], 0
                jg      loc_406211
loc_40622F:                             ; CODE XREF: ___ld12mul+136↑j
                cmp     word [ebp+8], 0
                jg      loc_40626F
loc_406236:                             ; CODE XREF: ___ld12mul+130↑j
                add     dword [ebp+8], 0FFFFh
                cmp     word [ebp+8], 0
                jge     loc_40626F
                movsx   eax, word [ebp+8]
                neg     eax
                add     [ebp+8], eax
                mov     ebx, eax
loc_40624F:                             ; CODE XREF: ___ld12mul+184↓j
                test    byte [ebp-36], 1
                jz      loc_406258
                inc     dword [ebp-20]
loc_406258:                             ; CODE XREF: ___ld12mul+174↑j
                lea     eax, [ebp-36]
                push    eax
                call    ___shr_12
                dec     ebx
                pop     ecx
                jnz     loc_40624F
                cmp     dword [ebp-20], 0
                jz      loc_40626F
                or      byte [ebp-36], 1
loc_40626F:                             ; CODE XREF: ___ld12mul+155↑j
                cmp     word [ebp-36], 8000h
                ja      loc_406286
                mov     eax, [ebp-36]
                and     eax, 1FFFFh
                cmp     eax, 18000h
                jnz     loc_4062BB
loc_406286:                             ; CODE XREF: ___ld12mul+196↑j
                cmp     dword [ebp-34], 0FFFFFFFFh
                jnz     loc_4062B8
                and     dword [ebp-34], 0
                cmp     dword [ebp-30], 0FFFFFFFFh
                jnz     loc_4062B3
                and     dword [ebp-30], 0
                cmp     word [ebp-26], 0FFFFh
                jnz     loc_4062AD
                inc     dword [ebp+8]
                mov     word [ebp-26], 8000h
                jmp     loc_4062BB
loc_4062AD:                             ; CODE XREF: ___ld12mul+1C1↑j
                inc     word [ebp-26]
                jmp     loc_4062BB
loc_4062B3:                             ; CODE XREF: ___ld12mul+1B5↑j
                inc     dword [ebp-30]
                jmp     loc_4062BB
loc_4062B8:                             ; CODE XREF: ___ld12mul+1AB↑j
                inc     dword [ebp-34]
loc_4062BB:                             ; CODE XREF: ___ld12mul+1A5↑j
                mov     eax, [ebp+8]
                cmp     ax, 7FFFh
                jnb     loc_4062DF
                mov     cx, [ebp-34]
                or      eax, edi
                mov     [esi], cx
                mov     ecx, [ebp-32]
                mov     [esi+2], ecx
                mov     ecx, [ebp-28]
                mov     [esi+6], ecx
loc_4062D9:                             ; CODE XREF: ___ld12mul+86↑j
                mov     [esi+10], ax
                jmp     loc_4062FA
loc_4062DF:                             ; CODE XREF: ___ld12mul+42↑j
                neg     di
                sbb     edi, edi
                and     dword [esi+4], 0
                and     edi, 80000000h
                add     edi, 7FFF8000h
                and     dword [esi], 0
                mov     [esi+8], edi
loc_4062FA:                             ; CODE XREF: ___ld12mul+AB↑j
                pop     edi
                pop     esi
                pop     ebx
                leave
                retn

__IsZeroMan:
                mov     eax, [esp+4]
                xor     ecx, ecx
      loc_403106:
                cmp     DWORD [eax], 0
                jnz     loc_403118
                inc     ecx
                add     eax, 4
                cmp     ecx, 3
                jl      loc_403106
                push    1
                pop     eax
                retn
      loc_403118:
                xor     eax, eax
                retn

__FillZeroMan:
                push    edi
                mov     edi, [esp+8]
                xor     eax, eax
                stosd
                stosd
                stosd
                pop     edi
                retn

__ShrMan:
                push    ebp
                mov     ebp, esp
                sub     esp, 12
                mov     eax, [ebp+12]
                push    ebx
                push    esi
                push    edi
                push    32
                mov     edi, [ebp+8]
                pop     ebx
                or      esi, -1
                cdq
                mov     ecx, ebx
                mov     DWORD [ebp-4], 3
                idiv    ecx
                mov     [ebp-12], eax
                mov     eax, [ebp+12]
                cdq
                idiv    ecx
                and     DWORD [ebp+12], 0
                mov     ecx, edx
                shl     esi, cl
                sub     ebx, edx
                not     esi
      loc_403151:
                mov     eax, [edi]
                mov     ecx, eax
                and     ecx, esi
                mov     [ebp-8], ecx
                mov     ecx, edx
                shr     eax, cl
                or      eax, [ebp+12]
                mov     [edi], eax
                mov     eax, [ebp-8]
                mov     ecx, ebx
                add     edi, 4
                shl     eax, cl
                dec     DWORD [ebp-4]
                mov     [ebp+12], eax
                jnz     loc_403151
                mov     edi, [ebp-12]
                push    2
                pop     ebx
                mov     esi, edi
                push    8
                pop     ecx
                shl     esi, 2
      loc_403183:
                cmp     ebx, edi
                jl      loc_403196
                mov     edx, [ebp+8]
                mov     eax, ecx
                sub     eax, esi
                mov     eax, [eax+edx]
                mov     [ecx+edx], eax
                jmp     loc_40319D
      loc_403196:
                mov     eax, [ebp+8]
                and     DWORD [ecx+eax], 0
      loc_40319D:
                dec     ebx
                sub     ecx, 4
                jns     loc_403183
                pop     edi
                pop     esi
                pop     ebx
                leave
                retn

___multtenpow12:
                push    ebp
                mov     ebp, esp
                sub     esp, 0Ch
                push    ebx
                mov     ebx, _pow10pos
                xor     ecx, ecx
                sub     ebx, 60h ; '`'
                cmp     [ebp+12], ecx
                jz      loc_406378
                jge     loc_406327
                mov     eax, [ebp+12]
                mov     ebx, _pow10neg
                neg     eax
                mov     [ebp+12], eax
                sub     ebx, 60h ; '`'
loc_406327:                             ; CODE XREF: ___multtenpow12+16↑j
                cmp     [ebp+16], ecx
                jnz     loc_406332
                mov     eax, [ebp+8]
                mov     [eax], cx
loc_406332:                             ; CODE XREF: ___multtenpow12+2B↑j
                cmp     [ebp+12], ecx
                jz      loc_406378
                push    esi
                push    edi
loc_406339:                             ; CODE XREF: ___multtenpow12+75↓j
                mov     eax, [ebp+12]
                add     ebx, 54h ; 'T'
                sar     dword [ebp+12], 3
                and     eax, 7
                cmp     eax, ecx
                jz      loc_406371
                lea     eax, [eax+eax*2]
                cmp     word [ebx+eax*4], 8000h
                lea     esi, [ebx+eax*4]
                jb      loc_406364
                lea     edi, [ebp-12]
                movsd
                movsd
                movsd
                dec     dword [ebp-10]
                lea     esi, [ebp-12]
loc_406364:                             ; CODE XREF: ___multtenpow12+57↑j
                push    esi
                push    dword [ebp+8]
                call    ___ld12mul
                pop     ecx
                pop     ecx
                xor     ecx, ecx
loc_406371:                             ; CODE XREF: ___multtenpow12+49↑j
                cmp     [ebp+12], ecx
                jnz     loc_406339
                pop     edi
                pop     esi
loc_406378:                             
                pop     ebx
                leave
                retn

_pow10pos:      db 9 dup(0), 0A0h, 2, 40h, 9 dup(0), 0C8h, 5, 40h, 9 dup(0)
                db 0FAh, 8, 40h, 8 dup(0), 40h, 9Ch, 0Ch, 40h, 8 dup(0)
                db 50h, 0C3h, 0Fh, 40h, 8 dup(0), 24h, 0F4h, 12h, 40h
                db 7 dup(0), 80h, 96h, 98h, 16h, 40h, 7 dup(0), 20h, 0BCh
                db 0BEh, 19h, 40h, 5 dup(0), 4, 0BFh, 0C9h, 1Bh, 8Eh, 34h
                db 40h, 3 dup(0), 0A1h, 0EDh, 0CCh, 0CEh, 1Bh, 0C2h, 0D3h
                db 4Eh, 40h, 20h, 0F0h, 9Eh, 0B5h, 70h, 2Bh, 0A8h, 0ADh
                db 0C5h, 9Dh, 69h, 40h, 0D0h, 5Dh, 0FDh, 25h, 0E5h, 1Ah
                db 8Eh, 4Fh, 19h, 0EBh, 83h, 40h, 71h, 96h, 0D7h, 95h
                db 43h, 0Eh, 5, 8Dh, 29h, 0AFh, 9Eh, 40h, 0F9h, 0BFh, 0A0h
                db 44h, 0EDh, 81h, 12h, 8Fh, 81h, 82h, 0B9h, 40h, 0BFh
                db 3Ch, 0D5h, 0A6h, 0CFh, 0FFh, 49h, 1Fh, 78h, 0C2h, 0D3h
                db 40h, 6Fh, 0C6h, 0E0h, 8Ch, 0E9h, 80h, 0C9h, 47h, 0BAh
                db 93h, 0A8h, 41h, 0BCh, 85h, 6Bh, 55h, 27h, 39h, 8Dh
                db 0F7h, 70h, 0E0h, 7Ch, 42h, 0BCh, 0DDh, 8Eh, 0DEh, 0F9h
                db 9Dh, 0FBh, 0EBh, 7Eh, 0AAh, 51h, 43h, 0A1h, 0E6h, 76h
                db 0E3h, 0CCh, 0F2h, 29h, 2Fh, 84h, 81h, 26h, 44h, 28h
                db 10h, 17h, 0AAh, 0F8h, 0AEh, 10h, 0E3h, 0C5h, 0C4h, 0FAh
                db 44h, 0EBh, 0A7h, 0D4h, 0F3h, 0F7h, 0EBh, 0E1h, 4Ah
                db 7Ah, 95h, 0CFh, 45h, 65h, 0CCh, 0C7h, 91h, 0Eh, 0A6h
                db 0AEh, 0A0h, 19h, 0E3h, 0A3h, 46h, 0Dh, 65h, 17h, 0Ch
                db 75h, 81h, 86h, 75h, 76h, 0C9h, 48h, 4Dh, 58h, 42h, 0E4h
                db 0A7h, 93h, 39h, 3Bh, 35h, 0B8h, 0B2h, 0EDh, 53h, 4Dh
                db 0A7h, 0E5h, 5Dh, 3Dh, 0C5h, 5Dh, 3Bh, 8Bh, 9Eh, 92h
                db 5Ah, 0FFh, 5Dh, 0A6h, 0F0h, 0A1h, 20h, 0C0h, 54h, 0A5h
                db 8Ch, 37h, 61h, 0D1h, 0FDh, 8Bh, 5Ah, 8Bh, 0D8h, 25h
                db 5Dh, 89h, 0F9h, 0DBh, 67h, 0AAh, 95h, 0F8h, 0F3h, 27h
                db 0BFh, 0A2h, 0C8h, 5Dh, 0DDh, 80h, 6Eh, 4Ch, 0C9h, 9Bh
                db 97h, 20h, 8Ah, 2, 52h, 60h, 0C4h, 25h, 75h, 4 dup(0)

_pow10neg:      db 0CDh, 0CCh, 0CDh, 7 dup(0CCh), 0FBh, 3Fh, 71h, 3Dh
                db 0Ah, 0D7h, 0A3h, 70h, 3Dh, 0Ah, 0D7h, 0A3h, 0F8h, 3Fh
                db 5Ah, 64h, 3Bh, 0DFh, 4Fh, 8Dh, 97h, 6Eh, 12h, 83h, 0F5h
                db 3Fh, 0C3h, 0D3h, 2Ch, 65h, 19h, 0E2h, 58h, 17h, 0B7h
                db 0D1h, 0F1h, 3Fh, 0D0h, 0Fh, 23h, 84h, 47h, 1Bh, 47h
                db 0ACh, 0C5h, 0A7h, 0EEh, 3Fh, 40h, 0A6h, 0B6h, 69h, 6Ch
                db 0AFh, 5, 0BDh, 37h, 86h, 0EBh, 3Fh, 33h, 3Dh, 0BCh
                db 42h, 7Ah, 0E5h, 0D5h, 94h, 0BFh, 0D6h, 0E7h, 3Fh, 0C2h
                db 2 dup(0FDh), 0CEh, 61h, 84h, 11h, 77h, 0CCh, 0ABh, 0E4h
                db 3Fh, 2Fh, 4Ch, 5Bh, 0E1h, 4Dh, 0C4h, 0BEh, 94h, 95h
                db 0E6h, 0C9h, 3Fh, 92h, 0C4h, 53h, 3Bh, 75h, 44h, 0CDh
                db 14h, 0BEh, 9Ah, 0AFh, 3Fh, 0DEh, 67h, 0BAh, 94h, 39h
                db 45h, 0ADh, 1Eh, 0B1h, 0CFh, 94h, 3Fh, 24h, 23h, 0C6h
                db 0E2h, 0BCh, 0BAh, 3Bh, 31h, 61h, 8Bh, 7Ah, 3Fh, 61h
                db 55h, 59h, 0C1h, 7Eh, 0B1h, 53h, 7Ch, 12h, 0BBh, 5Fh
                db 3Fh, 0D7h, 0EEh, 2Fh, 8Dh, 6, 0BEh, 92h, 85h, 15h, 0FBh
                db 44h, 3Fh, 24h, 3Fh, 0A5h, 0E9h, 39h, 0A5h, 27h, 0EAh
                db 7Fh, 0A8h, 2Ah, 3Fh, 7Dh, 0ACh, 0A1h, 0E4h, 0BCh, 64h
                db 7Ch, 46h, 0D0h, 0DDh, 55h, 3Eh, 63h, 7Bh, 6, 0CCh, 23h
                db 54h, 77h, 83h, 0FFh, 91h, 81h, 3Dh, 91h, 0FAh, 3Ah
                db 19h, 7Ah, 63h, 25h, 43h, 31h, 0C0h, 0ACh, 3Ch, 21h
                db 89h, 0D1h, 38h, 82h, 47h, 97h, 0B8h, 0, 0FDh, 0D7h
                db 3Bh, 0DCh, 88h, 58h, 8, 1Bh, 0B1h, 0E8h, 0E3h, 86h
                db 0A6h, 3, 3Bh, 0C6h, 84h, 45h, 42h, 7, 0B6h, 99h, 75h
                db 37h, 0DBh, 2Eh, 3Ah, 33h, 71h, 1Ch, 0D2h, 23h, 0DBh
                db 32h, 0EEh, 49h, 90h, 5Ah, 39h, 0A6h, 87h, 0BEh, 0C0h
                db 57h, 0DAh, 0A5h, 82h, 0A6h, 0A2h, 0B5h, 32h, 0E2h, 68h
                db 0B2h, 11h, 0A7h, 52h, 9Fh, 44h, 59h, 0B7h, 10h, 2Ch
                db 25h, 49h, 0E4h, 2Dh, 36h, 34h, 4Fh, 53h, 0AEh, 0CEh
                db 6Bh, 25h, 8Fh, 59h, 4, 0A4h, 0C0h, 0DEh, 0C2h, 7Dh
                db 0FBh, 0E8h, 0C6h, 1Eh, 9Eh, 0E7h, 88h, 5Ah, 57h, 91h
                db 3Ch, 0BFh, 50h, 83h, 22h, 18h, 4Eh, 4Bh, 65h, 62h, 0FDh
                db 83h, 8Fh, 0AFh, 6, 94h, 7Dh, 11h, 0E4h, 2Dh, 0DEh, 9Fh
                db 0CEh, 0D2h, 0C8h, 4, 0DDh, 0A6h, 0D8h, 0Ah, 4 dup(0)

