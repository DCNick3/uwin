mod hireg {
    test_snippets! {
        mov_shit_ah: (
            ; mov eax, 0x12345678
            ; mov ah, -0x66
        ) [CF ZF SF OF],
        mov_ah_shit: (
            ; mov eax, 0x12345678
            ; mov bl, ah
        ) [CF ZF SF OF],
    }
}

mod mov {
    test_snippets! {

        /* test name */
        mov_eax_42: (
            /* test body */
            ; mov eax, 42

        /* optional list of flags to test */
        ) [CF ZF SF OF],
        mov_ebx_42: (
            ; mov ebx, 42
        ) [CF ZF SF OF],

        mov_al_42: (
            ; mov al, 42
        ) [CF ZF SF OF],

        mov_al_42_dirty: (
            ; mov eax, 0x41424344
            ; mov al, 42
        ) [CF ZF SF OF],

        mov_ax_42_dirty: (
            ; mov eax, 0x41424344
            ; mov ax, 42
        ) [CF ZF SF OF],

        // mov_ah_42_dirty: (
        //     ; mov eax, 0x41424344
        //     ; mov ah, 42
        // ) [CF ZF SF OF],
    }
}

mod movzx {
    test_snippets! {
        movzx_16_0: (
            ; mov ax, 0
            ; movzx eax, ax
        ) [CF ZF SF OF],

        movzx_16_0xffff: (
            ; mov ax, -1
            ; movzx eax, ax
        ) [CF ZF SF OF],

        movzx_16_0x8000: (
            ; mov ax, -0x8000
            ; movzx eax, ax
        ) [CF ZF SF OF],

        movzx_16_0x1337: (
            ; mov ax, 0x1337
            ; movzx eax, ax
        ) [CF ZF SF OF],

        movzx_8_0: (
            ; mov al, 0
            ; movzx eax, al
        ) [CF ZF SF OF],

        movzx_8_0xff: (
            ; mov al, -1
            ; movzx eax, al
        ) [CF ZF SF OF],

        movzx_8_0x80: (
            ; mov al, -0x80
            ; movzx eax, al
        ) [CF ZF SF OF],

        movzx_8_0x37: (
            ; mov al, 0x37
            ; movzx eax, al
        ) [CF ZF SF OF],
    }
}

mod movsx {
    test_snippets! {
        movsx_16_0: (
            ; mov ax, 0
            ; movsx eax, ax
        ) [CF ZF SF OF],

        movsx_16_0xffff: (
            ; mov ax, -1
            ; movsx eax, ax
        ) [CF ZF SF OF],

        movsx_16_0x8000: (
            ; mov ax, -0x8000
            ; movsx eax, ax
        ) [CF ZF SF OF],

        movsx_16_0x1337: (
            ; mov ax, 0x1337
            ; movsx eax, ax
        ) [CF ZF SF OF],

        movsx_8_0: (
            ; mov al, 0
            ; movsx eax, al
        ) [CF ZF SF OF],

        movsx_8_0xff: (
            ; mov al, -1
            ; movsx eax, al
        ) [CF ZF SF OF],

        movsx_8_0x80: (
            ; mov al, -0x80
            ; movsx eax, al
        ) [CF ZF SF OF],

        movsx_8_0x37: (
            ; mov al, 0x37
            ; movsx eax, al
        ) [CF ZF SF OF],
    }
}

mod sub {
    test_snippets! {
        sub_1_2: (
            ; mov eax, 1
            ; sub eax, 2
        ) [CF ZF SF OF],
        sub_2_1: (
            ; mov eax, 2
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_0_1: (
            ; mov eax, 0
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_1_1: (
            ; mov eax, 1
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_neg_1_1: (
            ; mov eax, -1
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_1_neg_1: (
            ; mov eax, 1
            ; sub eax, -1
        ) [CF ZF SF OF],
        sub_0x7fffffff_1: (
            ; mov eax, 0x7fffffff
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_1_0x7fffffff: (
            ; mov eax, 1
            ; sub eax, 0x7fffffff
        ) [CF ZF SF OF],
        sub_neg_0x80000000_1: (
            ; mov eax, -0x80000000
            ; sub eax, 1
        ) [CF ZF SF OF],
        sub_sbb_neg_0x80000000_0: (
            ; mov eax, -0x80000000
            ; sub eax, 0
        ) [CF ZF SF OF],
    }
}

mod stc_clc {
    test_snippets! {
        stc: (
            ; stc
        ) [CF ZF SF OF],
        clc: (
            ; clc
        ) [CF ZF SF OF],
        clc_stc: (
            ; clc
            ; stc
        ) [CF ZF SF OF],
        stc_clc: (
            ; stc
            ; clc
        ) [CF ZF SF OF],
    }
}

mod sbb {
    test_snippets! {
        sbb_1_2: (
            ; mov eax, 1
            ; sbb eax, 2
        ) [CF ZF SF OF],
        sbb_2_1: (
            ; mov eax, 2
            ; sbb eax, 1
        ) [CF ZF SF OF],
        sbb_0_1: (
            ; mov eax, 0
            ; sbb eax, 1
        ) [CF ZF SF OF],
        sbb_1_1: (
            ; mov eax, 1
            ; sbb eax, 1
        ) [CF ZF SF OF],
        sbb_neg_1_1: (
            ; mov eax, -1
            ; sbb eax, 1
        ) [CF ZF SF OF],
        sbb_1_neg_1: (
            ; mov eax, 1
            ; sbb eax, -1
        ) [CF ZF SF OF],
        sbb_0x7fffffff_1: (
            ; mov eax, 0x7fffffff
            ; sbb eax, 1
        ) [CF ZF SF OF],
        sbb_1_0x7fffffff: (
            ; mov eax, 1
            ; sbb eax, 0x7fffffff
        ) [CF ZF SF OF],
        sbb_neg_0x80000000_0: (
            ; mov eax, -0x80000000
            ; sbb eax, 0
        ) [CF ZF SF OF],
        sbb_neg_0x80000000_1: (
            ; mov eax, -0x80000000
            ; sbb eax, 1
        ) [CF ZF SF OF],

        stc_sbb_0x7fffffff_neg_1: (
            ; stc
            ; mov eax, 0x7fffffff
            ; mov ebx, -1
            ; sbb eax, ebx
        ) [CF ZF SF OF],
        stc_sbb_neg_1_neg_1: (
            ; stc
            ; mov eax, -1
            ; mov ebx, -1
            ; sbb eax, ebx
        ) [CF ZF SF OF],

        stc_sbb_1_2: (
            ; stc
            ; mov eax, 1
            ; sbb eax, 2
        ) [CF ZF SF OF],
        stc_sbb_2_1: (
            ; stc
            ; mov eax, 2
            ; sbb eax, 1
        ) [CF ZF SF OF],
        stc_sbb_0_1: (
            ; stc
            ; mov eax, 0
            ; sbb eax, 1
        ) [CF ZF SF OF],
        stc_sbb_1_1: (
            ; stc
            ; mov eax, 1
            ; sbb eax, 1
        ) [CF ZF SF OF],
        stc_sbb_neg_1_1: (
            ; stc
            ; mov eax, -1
            ; sbb eax, 1
        ) [CF ZF SF OF],
        stc_sbb_1_neg_1: (
            ; stc
            ; mov eax, 1
            ; sbb eax, -1
        ) [CF ZF SF OF],
        stc_sbb_0x7fffffff_1: (
            ; stc
            ; mov eax, 0x7fffffff
            ; sbb eax, 1
        ) [CF ZF SF OF],
        stc_sbb_1_0x7fffffff: (
            ; stc
            ; mov eax, 1
            ; sbb eax, 0x7fffffff
        ) [CF ZF SF OF],
        stc_sbb_neg_0x80000000_0: (
            ; stc
            ; mov eax, -0x80000000
            ; sbb eax, 0
        ) [CF ZF SF OF],
        stc_sbb_neg_0x80000000_1: (
            ; stc
            ; mov eax, -0x80000000
            ; sbb eax, 1
        ) [CF ZF SF OF],
    }
}

mod add {
    test_snippets! {
        add_borrow: (
            ; mov eax, 1
            ; add eax, 2
        ) [CF ZF SF OF],
        add_branch_sign: (
            ; mov eax, 1
            ; add eax, 2
            ; js ->L1 // TODO: cmov is more concise?
            ; mov ebx, 1
            ; jmp ->R
            ; ->L1:
            ; mov ebx, 2
            ; ->R:
            ; mov edx, 1 // necessary because of funky control flow at the end of test snippets...
        ) [CF ZF SF OF],
        add_cmov_sign: (
            ; mov eax, 1
            ; add eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
        add_cmov_sign_2: (
            ; mov eax, 3
            ; add eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
    }
}

mod adc {
    test_snippets! {
        adc_1_2: (
            ; mov eax, 1
            ; adc eax, 2
        ) [CF ZF SF OF],
        adc_2_1: (
            ; mov eax, 2
            ; adc eax, 1
        ) [CF ZF SF OF],
        adc_0_1: (
            ; mov eax, 0
            ; adc eax, 1
        ) [CF ZF SF OF],
        adc_1_1: (
            ; mov eax, 1
            ; adc eax, 1
        ) [CF ZF SF OF],
        adc_neg_1_1: (
            ; mov eax, -1
            ; adc eax, 1
        ) [CF ZF SF OF],
        adc_1_neg_1: (
            ; mov eax, 1
            ; adc eax, -1
        ) [CF ZF SF OF],
        adc_0x7fffffff_1: (
            ; mov eax, 0x7fffffff
            ; adc eax, 1
        ) [CF ZF SF OF],
        adc_1_0x7fffffff: (
            ; mov eax, 1
            ; adc eax, 0x7fffffff
        ) [CF ZF SF OF],
        adc_neg_0x80000000_0: (
            ; mov eax, -0x80000000
            ; adc eax, 0
        ) [CF ZF SF OF],
        adc_neg_0x80000000_1: (
            ; mov eax, -0x80000000
            ; adc eax, 1
        ) [CF ZF SF OF],

        stc_adc_0x7fffffff_neg_1: (
            ; stc
            ; mov eax, 0x7fffffff
            ; mov ebx, -1
            ; adc eax, ebx
        ) [CF ZF SF OF],
        stc_adc_neg_1_neg_1: (
            ; stc
            ; mov eax, -1
            ; mov ebx, -1
            ; adc eax, ebx
        ) [CF ZF SF OF],

        stc_adc_1_2: (
            ; stc
            ; mov eax, 1
            ; adc eax, 2
        ) [CF ZF SF OF],
        stc_adc_2_1: (
            ; stc
            ; mov eax, 2
            ; adc eax, 1
        ) [CF ZF SF OF],
        stc_adc_0_1: (
            ; stc
            ; mov eax, 0
            ; adc eax, 1
        ) [CF ZF SF OF],
        stc_adc_1_1: (
            ; stc
            ; mov eax, 1
            ; adc eax, 1
        ) [CF ZF SF OF],
        stc_adc_neg_1_1: (
            ; stc
            ; mov eax, -1
            ; adc eax, 1
        ) [CF ZF SF OF],
        stc_adc_1_neg_1: (
            ; stc
            ; mov eax, 1
            ; adc eax, -1
        ) [CF ZF SF OF],
        stc_adc_0x7fffffff_1: (
            ; stc
            ; mov eax, 0x7fffffff
            ; adc eax, 1
        ) [CF ZF SF OF],
        stc_adc_1_0x7fffffff: (
            ; stc
            ; mov eax, 1
            ; adc eax, 0x7fffffff
        ) [CF ZF SF OF],
        stc_adc_neg_0x80000000_0: (
            ; stc
            ; mov eax, -0x80000000
            ; adc eax, 0
        ) [CF ZF SF OF],
        stc_adc_neg_0x80000000_1: (
            ; stc
            ; mov eax, -0x80000000
            ; adc eax, 1
        ) [CF ZF SF OF],
    }
}

mod cmp {
    test_snippets! {
        cmp_cmov_eq: (
            ; mov eax, 12
            ; cmp eax, 12
            ; mov ecx, 2
            ; cmovz ebx, ecx
        ) [CF ZF SF OF],
        cmp_cmov_eq_2: (
            ; mov eax, 12
            ; cmp eax, 13
            ; mov ecx, 2
            ; cmovz ebx, ecx
        ) [CF ZF SF OF],
        cmp_set_eq: (
            ; mov eax, 12
            ; cmp eax, 12
            ; mov ecx, 2
            ; setz dl
        ) [CF ZF SF OF],
        cmp_set_eq_2: (
            ; mov eax, 12
            ; cmp eax, 13
            ; mov ecx, 2
            ; setz dl
        ) [CF ZF SF OF],
        cmp_less: (
            ; mov eax, 11
            ; cmp eax, 13
        ) [CF ZF SF OF],
        cmp_neg_1: (
            ; mov eax, -1
            ; cmp eax, -2
        ) [CF ZF SF OF],
        cmp_neg_2: (
            ; mov eax, 0
            ; cmp eax, 1
        ) [CF ZF SF OF],
        cmp_neg_3: (
            ; mov eax, -0x80000000
            ; cmp eax, 1
        ) [CF ZF SF OF],
        cmp_rnd_1: (
            ; mov eax, 0x3e9c87ab
            ; cmp eax, 0x47f38608
        ) [CF ZF SF OF],
        cmp_rnd_2: (
            ; mov eax, -0x403f0352
            ; cmp eax, -0x4440a37e
        ) [CF ZF SF OF],
        cmp_rnd_3: (
            ; mov eax, 0x2600bb16
            ; cmp eax, 0x73fc32b6
        ) [CF ZF SF OF],
    }
}

mod lea {
    test_snippets! {
        lea_disp: (
            ; mov eax, 1228
            ; lea ecx, [eax + 7]
        ) [CF ZF SF OF],
        lea_idx: (
            ; mov eax, 1228
            ; mov ebx, 337
            ; lea ecx, [eax + ebx*4]
        ) [CF ZF SF OF],
        lea_idx_disp: (
            ; mov eax, 1228
            ; mov ebx, 337
            ; lea ecx, [eax + ebx*4 + 7]
        ) [CF ZF SF OF],

        lea16_disp: (
            ; mov eax, 91228
            ; lea ecx, [eax + 7]
        ) [CF ZF SF OF],
        lea16_idx: (
            ; mov eax, 91228
            ; mov ebx, 1337
            ; lea cx, [eax + ebx*4]
        ) [CF ZF SF OF],
        lea16_idx_disp: (
            ; mov eax, 91228
            ; mov ebx, 1337
            ; lea cx, [eax + ebx*4 + 7]
        ) [CF ZF SF OF],

        // dynasm won't assemble this for us =(
        // lea_from_16: (
        //     ; mov ax, 12
        //     ; lea ebx, [ax + 6]
        // ) [CF ZF SF OF],
    }
}

mod dec {
    test_snippets! {
        dec_0: (
            ; mov eax, 0
            ; dec eax
        ) [CF ZF SF OF],
        dec_1: (
            ; mov eax, 1
            ; dec eax
        ) [CF ZF SF OF],
        dec_neg_1: (
            ; mov eax, -1
            ; dec eax
        ) [CF ZF SF OF],
        dec_neg_2: (
            ; mov eax, -2
            ; dec eax
        ) [CF ZF SF OF],
        dec_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; dec eax
        ) [CF ZF SF OF],
        dec_0x7fffffff: (
            ; mov eax, 0x7fffffff
            ; dec eax
        ) [CF ZF SF OF],
    }
    test_snippets! {
        dec_16_0: (
            ; mov ax, 0
            ; dec ax
        ) [CF ZF SF OF],
        dec_16_1: (
            ; mov ax, 1
            ; dec ax
        ) [CF ZF SF OF],
        dec_16_neg_1: (
            ; mov ax, -1
            ; dec ax
        ) [CF ZF SF OF],
        dec_16_neg_2: (
            ; mov ax, -2
            ; dec ax
        ) [CF ZF SF OF],
        dec_16_neg_0x8000: (
            ; mov ax, -0x8000
            ; dec ax
        ) [CF ZF SF OF],
        dec_16_0x7fff: (
            ; mov ax, 0x7fff
            ; dec ax
        ) [CF ZF SF OF],
    }
    test_snippets! {
        dec_8_0: (
            ; mov al, 0
            ; dec al
        ) [CF ZF SF OF],
        dec_8_1: (
            ; mov al, 1
            ; dec al
        ) [CF ZF SF OF],
        dec_8_neg_1: (
            ; mov al, -1
            ; dec al
        ) [CF ZF SF OF],
        dec_8_neg_2: (
            ; mov al, -2
            ; dec al
        ) [CF ZF SF OF],
        dec_8_neg_0x80: (
            ; mov al, -0x80
            ; dec al
        ) [CF ZF SF OF],
        dec_8_0x7f: (
            ; mov al, 0x7f
            ; dec al
        ) [CF ZF SF OF],
    }
}

mod inc {
    test_snippets! {
        inc_0: (
            ; mov eax, 0
            ; inc eax
        ) [CF ZF SF OF],
        inc_1: (
            ; mov eax, 1
            ; inc eax
        ) [CF ZF SF OF],
        inc_neg_1: (
            ; mov eax, -1
            ; inc eax
        ) [CF ZF SF OF],
        inc_neg_2: (
            ; mov eax, -2
            ; inc eax
        ) [CF ZF SF OF],
        inc_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; inc eax
        ) [CF ZF SF OF],
        inc_0x7fffffff: (
            ; mov eax, 0x7fffffff
            ; inc eax
        ) [CF ZF SF OF],
    }
    test_snippets! {
        inc_16_0: (
            ; mov ax, 0
            ; inc ax
        ) [CF ZF SF OF],
        inc_16_1: (
            ; mov ax, 1
            ; inc ax
        ) [CF ZF SF OF],
        inc_16_neg_1: (
            ; mov ax, -1
            ; inc ax
        ) [CF ZF SF OF],
        inc_16_neg_2: (
            ; mov ax, -2
            ; inc ax
        ) [CF ZF SF OF],
        inc_16_neg_0x8000: (
            ; mov ax, -0x8000
            ; inc ax
        ) [CF ZF SF OF],
        inc_16_0x7fff: (
            ; mov ax, 0x7fff
            ; inc ax
        ) [CF ZF SF OF],
    }
    test_snippets! {
        inc_8_0: (
            ; mov al, 0
            ; inc al
        ) [CF ZF SF OF],
        inc_8_1: (
            ; mov al, 1
            ; inc al
        ) [CF ZF SF OF],
        inc_8_neg_1: (
            ; mov al, -1
            ; inc al
        ) [CF ZF SF OF],
        inc_8_neg_2: (
            ; mov al, -2
            ; inc al
        ) [CF ZF SF OF],
        inc_8_neg_0x80: (
            ; mov al, -0x80
            ; inc al
        ) [CF ZF SF OF],
        inc_8_0x7f: (
            ; mov al, 0x7f
            ; inc al
        ) [CF ZF SF OF],
    }
}

mod neg {
    test_snippets! {
        neg_0: (
            ; mov eax, 0
            ; neg eax
        ) [CF ZF SF OF],
        neg_neg_1: (
            ; mov eax, -1
            ; neg eax
        ) [CF ZF SF OF],
        neg_228: (
            ; mov eax, 228
            ; neg eax
        ) [CF ZF SF OF],
        neg_neg_228: (
            ; mov eax, -228
            ; neg eax
        ) [CF ZF SF OF],
        neg_8_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; neg eax
        ) [CF ZF SF OF],

        neg_16_0: (
            ; mov ax, 0
            ; neg ax
        ) [CF ZF SF OF],
        neg_16_neg_1: (
            ; mov ax, -1
            ; neg ax
        ) [CF ZF SF OF],
        neg_16_228: (
            ; mov ax, 228
            ; neg ax
        ) [CF ZF SF OF],
        neg_16_neg_228: (
            ; mov ax, -228
            ; neg ax
        ) [CF ZF SF OF],
        neg_8_neg_0x8000: (
            ; mov ax, -0x8000
            ; neg ax
        ) [CF ZF SF OF],

        neg_8_0: (
            ; mov al, 0
            ; neg al
        ) [CF ZF SF OF],
        neg_8_neg_1: (
            ; mov al, -1
            ; neg al
        ) [CF ZF SF OF],
        neg_8_42: (
            ; mov al, 42
            ; neg al
        ) [CF ZF SF OF],
        neg_8_neg_42: (
            ; mov al, -42
            ; neg al
        ) [CF ZF SF OF],
        neg_8_neg_0x80: (
            ; mov al, -0x80
            ; neg al
        ) [CF ZF SF OF],

        neg_rnd: (
            ; mov eax, 0x79f9322a
            ; neg eax
        ) [CF ZF SF OF],
        neg_16_rnd: (
            ; mov eax, 0x79f9322a
            ; neg ax
        ) [CF ZF SF OF],
        neg_8_rnd: (
            ; mov eax, 0x79f9322a
            ; neg al
        ) [CF ZF SF OF],
    }
}

mod cdq {
    test_snippets! {
        cdq_zero: (
            ; mov eax, 0
            ; mov edx, 1337
            ; cdq
        ) [CF ZF SF OF],
        cdq_1: (
            ; mov eax, 1
            ; mov edx, 1337
            ; cdq
        ) [CF ZF SF OF],
        cdq_neg_1: (
            ; mov eax, -1
            ; mov edx, 1337
            ; cdq
        ) [CF ZF SF OF],
        cdq_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; mov edx, 1337
            ; cdq
        ) [CF ZF SF OF],
    }
    test_snippets! {
        cwd_zero: (
            ; mov ax, 0
            ; mov dx, 1337
            ; cwd
        ) [CF ZF SF OF],
        cwd_1: (
            ; mov ax, 1
            ; mov dx, 1337
            ; cwd
        ) [CF ZF SF OF],
        cwd_neg_1: (
            ; mov ax, -1
            ; mov dx, 1337
            ; cwd
        ) [CF ZF SF OF],
        cwd_neg_0x8000: (
            ; mov ax, -0x8000
            ; mov dx, 1337
            ; cwd
        ) [CF ZF SF OF],
    }
}

mod cwde {
    test_snippets! {
        cbw_zero: (
            ; mov al, 0
            ; mov ah, 123
            ; cbw
        ) [CF ZF SF OF],
        cbw_1: (
            ; mov al, 1
            ; mov ah, 123
            ; cbw
        ) [CF ZF SF OF],
        cbw_neg_1: (
            ; mov al, -1
            ; mov ah, 123
            ; cbw
        ) [CF ZF SF OF],
        cbw_neg_0x80: (
            ; mov al, -0x80
            ; mov ah, 123
            ; cbw
        ) [CF ZF SF OF],
    }
    test_snippets! {
        cwde_zero: (
            ; mov eax, 2003123123
            ; mov ax, 1
            ; cwde
        ) [CF ZF SF OF],
        cwde_1: (
            ; mov eax, 2003123123
            ; mov ax, 1
            ; cwde
        ) [CF ZF SF OF],
        cwde_neg_1: (
            ; mov eax, 2003123123
            ; mov ax, -1
            ; cwde
        ) [CF ZF SF OF],
        cwde_neg_0x8000: (
            ; mov eax, 2003123123
            ; mov ax, -0x8000
            ; cbw
        ) [CF ZF SF OF],
    }
}

mod mem {
    test_snippets! {
        mem_basic_rw: (
            ; mov eax, 42
            ; mov eax, [MEM_ADDR as i32]
            ; mov [MEM_ADDR as i32], ebx
        ) [CF ZF SF OF],
    }
}

mod imul {
    test_snippets! {
        imul_1op_eax_eax: (
            ; mov eax, 23
            ; imul eax
        ) [CF OF],
        imul_1op: (
            ; mov eax, 23
            ; mov ebx, 24
            ; imul ebx
        ) [CF OF],
        imul_1op_overflow: (
            ; mov eax, 0x7fffffff
            ; mov ebx, 0x7fffffff
            ; imul ebx
        ) [CF OF],

        imul_2op_eax_eax: (
            ; mov eax, 23
            ; imul eax, eax
        ) [CF OF],
        imul_2op: (
            ; mov eax, 23
            ; mov ebx, 24
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_overflow: (
            ; mov eax, 0x7fffffff
            ; mov ebx, 0x7fffffff
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd1: (
            ; mov eax, -0x2c333634
            ; mov ebx, 0x47ec9023
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd2: (
            ; mov eax, -0x23f11f0a
            ; mov ebx, -0x2073452e
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd3: (
            ; mov eax, 0x4f0e4a0c
            ; mov ebx, -0xefd25f
            ; imul eax, ebx
        ) [CF OF],

        imul_3op_eax_eax: (
            ; mov eax, 23
            ; imul eax, eax, 24
        ) [CF OF],
        imul_3op: (
            ; mov ebx, 24
            ; imul eax, ebx, 23
        ) [CF OF],
        imul_3op_overflow: (
            ; mov ebx, 0x7fffffff
            ; imul eax, ebx, 0x7fffffff
        ) [CF OF],
    }
}

mod mul {
    test_snippets! {
        mul_eax_eax: (
            ; mov eax, 23
            ; mul eax
        ) [CF OF],
        mul: (
            ; mov eax, 23
            ; mov ebx, 24
            ; mul ebx
        ) [CF OF],
        mul_overflow: (
            ; mov eax, 0x7fffffff
            ; mov ebx, 0x7fffffff
            ; mul ebx
        ) [CF OF],
        mul_overflow_more: (
            ; mov eax, -1
            ; mov ebx, -1
            ; mul ebx
        ) [CF OF],
    }
}

mod xor {
    test_snippets! {
        xor_zero_eax: (
            ; mov eax, 228
            ; xor eax, eax
        ) [CF ZF SF OF],
        xor_zero_eax_with_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; xor eax, ebx
        ) [CF ZF SF OF],
        xor_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; xor eax, ebx
        ) [CF ZF SF OF],
        xor_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; xor eax, ebx
        ) [CF ZF SF OF],
    }
}

mod not {
    test_snippets! {
        not_228: (
            ; mov eax, 228
            ; not eax
        ) [CF ZF SF OF],
        not_zero: (
            ; mov eax, 0
            ; not eax
        ) [CF ZF SF OF],
        not_ffffffff: (
            ; mov eax, -1
            ; not eax
        ) [CF ZF SF OF],
        not_rnd: (
            ; mov eax, 0x79f9322a
            ; not eax
        ) [CF ZF SF OF],
        not_16_rnd: (
            ; mov eax, 0x79f9322a
            ; not ax
        ) [CF ZF SF OF],
        not_8_rnd: (
            ; mov eax, 0x79f9322a
            ; not al
        ) [CF ZF SF OF],
    }
}

mod and {
    test_snippets! {
        and_same_eax_eax: (
            ; mov eax, 228
            ; and eax, eax
        ) [CF ZF SF OF],
        and_same_eax_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; and eax, ebx
        ) [CF ZF SF OF],
        and_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; and eax, ebx
        ) [CF ZF SF OF],
        and_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; and eax, ebx
        ) [CF ZF SF OF],
    }
}

mod test {
    test_snippets! {
        test_same_eax_eax: (
            ; mov eax, 228
            ; test eax, eax
        ) [CF ZF SF OF],
        test_same_eax_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; test eax, ebx
        ) [CF ZF SF OF],
        test_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; test eax, ebx
        ) [CF ZF SF OF],
        test_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; test eax, ebx
        ) [CF ZF SF OF],
    }
}

mod or {
    test_snippets! {
        or_same_eax_eax: (
            ; mov eax, 228
            ; or eax, eax
        ) [CF ZF SF OF],
        or_same_eax_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; or eax, ebx
        ) [CF ZF SF OF],
        or_0_0: (
            ; mov eax, 0
            ; or eax, 0
        ) [CF ZF SF OF],
        or_0_1: (
            ; mov eax, 0
            ; or eax, 1
        ) [CF ZF SF OF],
        or_1_0: (
            ; mov eax, 1
            ; or eax, 0
        ) [CF ZF SF OF],
        or_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; or eax, ebx
        ) [CF ZF SF OF],
        or_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; or eax, ebx
        ) [CF ZF SF OF],
    }
}

mod shr {
    test_snippets! {
        shr_zero: (
            ; mov eax, 228
            ; shr eax, 0
        ) [CF ZF SF OF],

        shr_228_one: (
            ; mov eax, 228
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_229_one: (
            ; mov eax, 229
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_neg_228_one: (
            ; mov eax, -228
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_neg_229_one: (
            ; mov eax, -229
            ; shr eax, 1
        ) [CF ZF SF OF],

        shr_228_two: (
            ; mov eax, 228
            ; shr eax, 2
        ) [CF ZF SF],
        shr_229_two: (
            ; mov eax, 229
            ; shr eax, 2
        ) [CF ZF SF],
        shr_neg_228_two: (
            ; mov eax, -228
            ; shr eax, 2
        ) [CF ZF SF],
        shr_neg_229_two: (
            ; mov eax, -229
            ; shr eax, 2
        ) [CF ZF SF],

        shr_228_zero_wrap: (
            ; mov eax, 228
            ; shr eax, 32
        ) [CF ZF SF OF],

        shr_228_one_wrap: (
            ; mov eax, 228
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_229_one_wrap: (
            ; mov eax, 229
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_neg_228_one_wrap: (
            ; mov eax, -228
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_neg_229_one_wrap: (
            ; mov eax, -229
            ; shr eax, 33
        ) [CF ZF SF OF],

        shr_228_two_wrap: (
            ; mov eax, 228
            ; shr eax, 34
        ) [CF ZF SF],
        shr_229_two_wrap: (
            ; mov eax, 229
            ; shr eax, 34
        ) [CF ZF SF],
        shr_neg_228_two_wrap: (
            ; mov eax, -228
            ; shr eax, 34
        ) [CF ZF SF],
        shr_neg_229_two_wrap: (
            ; mov eax, -229
            ; shr eax, 34
        ) [CF ZF SF],
    }
}

mod sar {
    test_snippets! {
        sar_zero: (
            ; mov eax, 228
            ; sar eax, 0
        ) [CF ZF SF OF],

        sar_228_one: (
            ; mov eax, 228
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_229_one: (
            ; mov eax, 229
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_neg_228_one: (
            ; mov eax, -228
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_neg_229_one: (
            ; mov eax, -229
            ; sar eax, 1
        ) [CF ZF SF OF],

        sar_228_two: (
            ; mov eax, 228
            ; sar eax, 2
        ) [CF ZF SF],
        sar_229_two: (
            ; mov eax, 229
            ; sar eax, 2
        ) [CF ZF SF],
        sar_neg_228_two: (
            ; mov eax, -228
            ; sar eax, 2
        ) [CF ZF SF],
        sar_neg_229_two: (
            ; mov eax, -229
            ; sar eax, 2
        ) [CF ZF SF],

        sar_228_zero_wrap: (
            ; mov eax, 228
            ; sar eax, 32
        ) [CF ZF SF OF],

        sar_228_one_wrap: (
            ; mov eax, 228
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_229_one_wrap: (
            ; mov eax, 229
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_neg_228_one_wrap: (
            ; mov eax, -228
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_neg_229_one_wrap: (
            ; mov eax, -229
            ; sar eax, 33
        ) [CF ZF SF OF],

        sar_228_two_wrap: (
            ; mov eax, 228
            ; sar eax, 34
        ) [CF ZF SF],
        sar_229_two_wrap: (
            ; mov eax, 229
            ; sar eax, 34
        ) [CF ZF SF],
        sar_neg_228_two_wrap: (
            ; mov eax, -228
            ; sar eax, 34
        ) [CF ZF SF],
        sar_neg_229_two_wrap: (
            ; mov eax, -229
            ; sar eax, 34
        ) [CF ZF SF],

        // basically https://github.com/nepx/halfix/issues/7
        sar_edge_case_byte: (
            ; mov al, -0x08
            ; sar al, 0x09
        ) [CF ZF SF OF],
        sar_edge_case_word: (
            ; mov ax, -0x0888
            ; sar ax, 0x11
        ) [CF ZF SF OF],
        sar_edge_case_dword: (
            ; mov eax, -0x08888888
            ; sar eax, 0x21
        ) [CF ZF SF OF],
    }
}

mod shl {
    test_snippets! {
        shl_zero: (
            ; mov eax, 228
            ; shl eax, 0
        ) [CF ZF SF OF],

        shl_228_one: (
            ; mov eax, 228
            ; shl eax, 1
        ) [CF ZF SF OF],
        shl_229_one: (
            ; mov eax, 229
            ; shl eax, 1
        ) [CF ZF SF OF],
        shl_neg_228_one: (
            ; mov eax, -228
            ; shl eax, 1
        ) [CF ZF SF OF],
        shl_neg_229_one: (
            ; mov eax, -229
            ; shl eax, 1
        ) [CF ZF SF OF],
        shl_neg_64_one: (
            ; mov eax, -64
            ; shl eax, 1
        ) [CF ZF SF OF],

        shl_228_two: (
            ; mov eax, 228
            ; shl eax, 2
        ) [CF ZF SF],
        shl_229_two: (
            ; mov eax, 229
            ; shl eax, 2
        ) [CF ZF SF],
        shl_neg_228_two: (
            ; mov eax, -228
            ; shl eax, 2
        ) [CF ZF SF],
        shl_neg_229_two: (
            ; mov eax, -229
            ; shl eax, 2
        ) [CF ZF SF],
        shl_neg_64_two: (
            ; mov eax, -64
            ; shl eax, 2
        ) [CF ZF SF],

        shl_228_zero_wrap: (
            ; mov eax, 228
            ; shl eax, 32
        ) [CF ZF SF OF],

        shl_228_one_wrap: (
            ; mov eax, 228
            ; shl eax, 33
        ) [CF ZF SF OF],
        shl_229_one_wrap: (
            ; mov eax, 229
            ; shl eax, 33
        ) [CF ZF SF OF],
        shl_neg_228_one_wrap: (
            ; mov eax, -228
            ; shl eax, 33
        ) [CF ZF SF OF],
        shl_neg_229_one_wrap: (
            ; mov eax, -229
            ; shl eax, 33
        ) [CF ZF SF OF],
        shl_neg_64_one_wrap: (
            ; mov eax, -64
            ; shl eax, 33
        ) [CF ZF SF OF],

        shl_228_two_wrap: (
            ; mov eax, 228
            ; shl eax, 34
        ) [CF ZF SF],
        shl_229_two_wrap: (
            ; mov eax, 229
            ; shl eax, 34
        ) [CF ZF SF],
        shl_neg_228_two_wrap: (
            ; mov eax, -228
            ; shl eax, 34
        ) [CF ZF SF],
        shl_neg_229_two_wrap: (
            ; mov eax, -229
            ; shl eax, 34
        ) [CF ZF SF],
        shl_neg_64_two_wrap: (
            ; mov eax, -64
            ; shl eax, 34
        ) [CF ZF SF],

        shl_edge_case_byte: (
            ; mov al, -0x08
            ; shl al, 0x09
        ) [CF ZF SF OF],
        shl_edge_case_word: (
            ; mov ax, -0x0888
            ; shl ax, 0x11
        ) [CF ZF SF OF],
        shl_edge_case_dword: (
            ; mov eax, -0x08888888
            ; shl eax, 0x21
        ) [CF ZF SF OF],
    }
}

mod rcr {
    test_snippets! {
        rcr_zero: (
            ; mov eax, 228
            ; rcr eax, 0
        ) [CF ZF SF OF],

        rcr_228_one: (
            ; mov eax, 228
            ; rcr eax, 1
        ) [CF ZF SF OF],
        rcr_229_one: (
            ; mov eax, 229
            ; rcr eax, 1
        ) [CF ZF SF OF],
        rcr_neg_228_one: (
            ; mov eax, -228
            ; rcr eax, 1
        ) [CF ZF SF OF],
        rcr_neg_229_one: (
            ; mov eax, -229
            ; rcr eax, 1
        ) [CF ZF SF OF],

        rcr_228_two: (
            ; mov eax, 228
            ; rcr eax, 2
        ) [CF ZF SF],
        rcr_229_two: (
            ; mov eax, 229
            ; rcr eax, 2
        ) [CF ZF SF],
        rcr_neg_228_two: (
            ; mov eax, -228
            ; rcr eax, 2
        ) [CF ZF SF],
        rcr_neg_229_two: (
            ; mov eax, -229
            ; rcr eax, 2
        ) [CF ZF SF],

        rcr_228_zero_wrap: (
            ; mov eax, 228
            ; rcr eax, 32
        ) [CF ZF SF OF],

        rcr_228_one_wrap: (
            ; mov eax, 228
            ; rcr eax, 33
        ) [CF ZF SF OF],
        rcr_229_one_wrap: (
            ; mov eax, 229
            ; rcr eax, 33
        ) [CF ZF SF OF],
        rcr_neg_228_one_wrap: (
            ; mov eax, -228
            ; rcr eax, 33
        ) [CF ZF SF OF],
        rcr_neg_229_one_wrap: (
            ; mov eax, -229
            ; rcr eax, 33
        ) [CF ZF SF OF],

        rcr_228_two_wrap: (
            ; mov eax, 228
            ; rcr eax, 34
        ) [CF ZF SF],
        rcr_229_two_wrap: (
            ; mov eax, 229
            ; rcr eax, 34
        ) [CF ZF SF],
        rcr_neg_228_two_wrap: (
            ; mov eax, -228
            ; rcr eax, 34
        ) [CF ZF SF],
        rcr_neg_229_two_wrap: (
            ; mov eax, -229
            ; rcr eax, 34
        ) [CF ZF SF],

        // from https://github.com/nepx/halfix/issues/8
        rcr_corner_case: (
            ; stc
            ; mov al, 0x00
            ; rcr al, 0x20
        ) [CF ZF SF],
    }
}

mod shld {
    test_snippets! {
        shld_zero: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shld eax, ebx, 0x0
        ) [CF ZF SF OF],
        shld_one: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shld eax, ebx, 0x1
        ) [CF ZF SF OF],
        shld_two: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shld eax, ebx, 0x2
        ) [CF ZF SF],
        shld_one_wrap: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shld eax, ebx, 0x21
        ) [CF ZF SF OF],
        shld_two_wrap: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shld eax, ebx, 0x22
        ) [CF ZF SF],
    }
}
mod shrd {
    test_snippets! {
        shrd_zero: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shrd eax, ebx, 0x0
        ) [CF ZF SF OF],
        shrd_one: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shrd eax, ebx, 0x1
        ) [CF ZF SF OF],
        shrd_two: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shrd eax, ebx, 0x2
        ) [CF ZF SF],
        shrd_one_wrap: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shrd eax, ebx, 0x21
        ) [CF ZF SF OF],
        shrd_two_wrap: (
            ; mov eax, 0x1
            ; mov ebx, 0x2
            ; shrd eax, ebx, 0x22
        ) [CF ZF SF],
    }
}

mod div {
    test_snippets!(
        div_basic1: (
            ; mov eax, 42
            ; mov ebx, 24
            ; div ebx
        ) [],
        div_basic2: (
            ; mov eax, 1
            ; mov ebx, 888
            ; div ebx
        ) [],
        div_basic3: (
            ; mov eax, 888
            ; mov ebx, 1
            ; div ebx
        ) [],
        div_basic4: (
            ; mov eax, 1
            ; mov ebx, 2
            ; div ebx
        ) [],

        div_16_basic1: (
            ; mov ax, 42
            ; mov bx, 24
            ; div bx
        ) [],
        div_16_basic2: (
            ; mov ax, 1
            ; mov bx, 888
            ; div bx
        ) [],
        div_16_basic3: (
            ; mov ax, 888
            ; mov bx, 1
            ; div bx
        ) [],
        div_16_basic4: (
            ; mov ax, 1
            ; mov bx, 2
            ; div bx
        ) [],

        div_8_basic1: (
            ; mov al, 42
            ; mov bl, 24
            ; div bl
        ) [],
        div_8_basic2: (
            ; mov al, 1
            ; mov bl, 88
            ; div bl
        ) [],
        div_8_basic3: (
            ; mov al, 88
            ; mov bl, 1
            ; div bl
        ) [],
        div_8_basic4: (
            ; mov al, 1
            ; mov bl, 2
            ; div bl
        ) [],


        div_rnd1: (
            ; mov eax, -0x57549d35
            ; mov ebx, 0x4003cb02
            ; div ebx
        ) [],
        div_rnd2: (
            ; mov eax, 0x37ab7947
            ; mov ebx, -0x6d61d34
            ; div ebx
        ) [],
        div_rnd3: (
            ; mov eax, 0x3a64b162
            ; mov ebx, -0x502df7b4
            ; div ebx
        ) [],
        div_big1: (
            ; mov eax, 0
            ; mov edx, 1
            ; mov ebx, 2
            ; div ebx
        ) [],
        div_16_big1: (
            ; mov ax, 0
            ; mov dx, 1
            ; mov bx, 2
            ; div bx
        ) [],
        div_8_big1: (
            ; mov al, 0
            ; mov ah, 1
            ; mov bl, 2
            ; div bl
        ) [],
        // this should cause a division error
        // TODO: how can we test this? (it's not how it behaves rn btw)
        // ditto for division by zero
        // div_big2: (
        //     ; mov eax, 0
        //     ; mov edx, 1
        //     ; mov ebx, 1
        //     ; div ebx
        // ),
        div_big_rnd1: (
            ; mov eax, -0x1895c25a
            ; mov edx, 0x6c8300d6
            ; mov ebx, 0x70a45624
            ; div ebx
        ) [],
        div_big_rnd2: (
            ; mov eax, -0x21c0f
            ; mov edx, 0x338001
            ; mov ebx, 0x90ed24d
            ; div ebx
        ) [],
        div_big_rnd3: (
            ; mov eax, 0x74f1d28c
            ; mov edx, 0x7507473a
            ; mov ebx, -0x7d79c77f
            ; div ebx
        ) [],
    );
}

mod idiv {
    test_snippets!(
        idiv_basic1: (
            ; mov eax, 42
            ; mov ebx, 24
            ; idiv ebx
        ) [],
        idiv_basic2: (
            ; mov eax, 1
            ; mov ebx, 888
            ; idiv ebx
        ) [],
        idiv_basic3: (
            ; mov eax, 888
            ; mov ebx, 1
            ; idiv ebx
        ) [],
        idiv_basic4: (
            ; mov eax, 1
            ; mov ebx, 2
            ; idiv ebx
        ) [],

        idiv_basic1_neg: (
            ; mov eax, -42
            ; mov ebx, 24
            ; idiv ebx
        ) [],
        idiv_basic2_neg: (
            ; mov eax, 1
            ; mov ebx, -888
            ; idiv ebx
        ) [],
        idiv_basic3_neg: (
            ; mov eax, -888
            ; mov ebx, 2
            ; idiv ebx
        ) [],
        idiv_basic4_neg: (
            ; mov eax, -1
            ; mov ebx, 2
            ; idiv ebx
        ) [],

        idiv_rnd1: (
            ; mov eax, -0x57549d35
            ; mov ebx, 0x4003cb02
            ; idiv ebx
        ) [],
        idiv_rnd2: (
            ; mov eax, 0x37ab7947
            ; mov ebx, -0x6d61d34
            ; idiv ebx
        ) [],
        idiv_rnd3: (
            ; mov eax, 0x3a64b162
            ; mov ebx, -0x502df7b4
            ; idiv ebx
        ) [],
        idiv_big1: (
            ; mov eax, 0
            ; mov edx, 1
            ; mov ebx, 3
            ; idiv ebx
        ) [],
        // this should cause a idivision error
        // TODO: how can we test this? (it's not how it behaves rn btw)
        // ditto for idivision by zero
        // idiv_big2: (
        //     ; mov eax, 0
        //     ; mov edx, 1
        //     ; mov ebx, 1
        //     ; idiv ebx
        // ),
        idiv_big_rnd1: (
            ; mov eax, -0x1895c25a
            ; mov edx, -0x0c8300d6
            ; mov ebx, 0x70a45624
            ; idiv ebx
        ) [],
        idiv_big_rnd2: (
            ; mov eax, -0x21c0f
            ; mov edx, 0x338001
            ; mov ebx, 0x90ed24d
            ; idiv ebx
        ) [],
        idiv_big_rnd3: (
            ; mov eax, 0x74f1d28c
            ; mov edx, -0x0507473a
            ; mov ebx, -0x7d79c77f
            ; idiv ebx
        ) [],
    );
}

mod xchg {
    test_snippets!(
        xchg_0_0: (
            ; xor eax, eax
            ; xor ebx, ebx
            ; xchg eax, ebx
        ) [CF ZF SF OF],
        xchg_0_1: (
            ; xor eax, eax
            ; mov ebx, 1
            ; xchg eax, ebx
        ) [CF ZF SF OF],
        xchg_eax_eax: (
            ; mov eax, 2281337
            ; xchg eax, eax
        ) [CF ZF SF OF],
        xchg_al: (
            ; mov eax, 2281337
            ; xchg al, bl
        ) [CF ZF SF OF],
    );
}

mod bsx {
    test_snippets! {
        bsr_zero: (
            ; mov eax, 0
            ; bsr eax, ebx
            ; mov ebx, 0 // bsr result is undefined for zero, so define it ;)
            // we only care for flags here
        ) [CF ZF SF OF],
        bsr_one: (
            ; mov eax, 1
            ; bsr eax, ebx
        ) [CF ZF SF OF],
        bsr_16_one: (
            ; mov ax, 1
            ; bsr ax, bx
        ) [CF ZF SF OF],
        bsr_neg_one: (
            ; mov eax, -1
            ; bsr eax, ebx
        ) [CF ZF SF OF],
        bsr_16_neg_one: (
            ; mov ax, -1
            ; bsr ax, bx
        ) [CF ZF SF OF],
    }
}

mod bt {
    // these test register access
    // in this case the search is performed only inside the register - the offset is taken modulo register size
    test_snippets! {
        bt_reg_0: (
            ; mov eax, 0
            ; bt eax, 0
        ) [CF ZF SF OF],
        bt_reg_1: (
            ; mov eax, 1
            ; bt eax, 0
        ) [CF ZF SF OF],
        bt_reg_2: (
            ; mov eax, 2
            ; bt eax, 1
        ) [CF ZF SF OF],
        bt_reg_0xfffffffd: (
            ; mov eax, -3
            ; bt eax, 1
        ) [CF ZF SF OF],
        bt_reg_1_wrap: (
            ; mov eax, 1
            ; mov ebx, 32 // we can't pass these as an immediate!!
            ; bt eax, ebx
        ) [CF ZF SF OF],
        bt_reg_0xfffffffd_wrap: (
            ; mov eax, -3
            ; mov ebx, 33
            ; bt eax, ebx
        ) [CF ZF SF OF],
        bt_reg_word_1_wrap: (
            ; mov ax, 1
            ; mov bx, 16
            ; bt ax, bx
        ) [CF ZF SF OF],
        bt_reg_word_0xfffd_wrap: (
            ; mov ax, -3
            ; mov bx, 17
            ; bt ax, bx
        ) [CF ZF SF OF],
    }
    // test the "memory" case
    // this one allows to index not only bits inside the specified argument, but actually uses the bit offset/8 as byte offset
    // but this apparently is only allowed when offset is stored in the register but not for immediate offset??
    test_snippets! {
        bt_mem_0: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov eax, 0
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_1: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov eax, 0
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_2: (
            ; mov DWORD [MEM_ADDR as i32], 2
            ; mov eax, 1
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_0xfffffffd: (
            ; mov DWORD [MEM_ADDR as i32], -3
            ; mov eax, 1
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov eax, 32
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_0xfffffffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov eax, 33
            ; bt [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bt_mem_word_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov ax, 16
            ; bt [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        bt_mem_word_0xfffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov ax, 17
            ; bt [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        bt_mem_word_1_neg: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -16
            ; bt [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
        bt_mem_word_0x80_neg: (
            ; mov DWORD [MEM_ADDR as i32], -0x80
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -1
            ; bt [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
    }
}
mod bts {
    // these test register access
    // in this case the search is performed only inside the register - the offset is taken modulo register size
    test_snippets! {
        bts_reg_0: (
            ; mov eax, 0
            ; bts eax, 0
        ) [CF ZF SF OF],
        bts_reg_1: (
            ; mov eax, 1
            ; bts eax, 0
        ) [CF ZF SF OF],
        bts_reg_2: (
            ; mov eax, 2
            ; bts eax, 1
        ) [CF ZF SF OF],
        bts_reg_0xfffffffd: (
            ; mov eax, -3
            ; bts eax, 1
        ) [CF ZF SF OF],
        bts_reg_1_wrap: (
            ; mov eax, 1
            ; mov ebx, 32 // we can't pass these as an immediate!!
            ; bts eax, ebx
        ) [CF ZF SF OF],
        bts_reg_0xfffffffd_wrap: (
            ; mov eax, -3
            ; mov ebx, 33
            ; bts eax, ebx
        ) [CF ZF SF OF],
        bts_reg_word_1_wrap: (
            ; mov ax, 1
            ; mov bx, 16
            ; bts ax, bx
        ) [CF ZF SF OF],
        bts_reg_word_0xfffd_wrap: (
            ; mov ax, -3
            ; mov bx, 17
            ; bts ax, bx
        ) [CF ZF SF OF],
    }
    // test the "memory" case
    // this one allows to index not only bits inside the specified argument, but actually uses the bit offset/8 as byte offset
    // but this apparently is only allowed when offset is stored in the register but not for immediate offset??
    test_snippets! {
        bts_mem_0: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov eax, 0
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_1: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov eax, 0
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_2: (
            ; mov DWORD [MEM_ADDR as i32], 2
            ; mov eax, 1
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_0xfffffffd: (
            ; mov DWORD [MEM_ADDR as i32], -3
            ; mov eax, 1
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov eax, 32
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_0xfffffffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov eax, 33
            ; bts [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        bts_mem_word_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov ax, 16
            ; bts [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        bts_mem_word_0xfffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov ax, 17
            ; bts [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        bts_mem_word_1_neg: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -16
            ; bts [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
        bts_mem_word_0x80_neg: (
            ; mov DWORD [MEM_ADDR as i32], -0x80
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -1
            ; bts [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
    }
}
mod btr {
    // these test register access
    // in this case the search is performed only inside the register - the offset is taken modulo register size
    test_snippets! {
        btr_reg_0: (
            ; mov eax, 0
            ; btr eax, 0
        ) [CF ZF SF OF],
        btr_reg_1: (
            ; mov eax, 1
            ; btr eax, 0
        ) [CF ZF SF OF],
        btr_reg_2: (
            ; mov eax, 2
            ; btr eax, 1
        ) [CF ZF SF OF],
        btr_reg_0xfffffffd: (
            ; mov eax, -3
            ; btr eax, 1
        ) [CF ZF SF OF],
        btr_reg_1_wrap: (
            ; mov eax, 1
            ; mov ebx, 32 // we can't pass these as an immediate!!
            ; btr eax, ebx
        ) [CF ZF SF OF],
        btr_reg_0xfffffffd_wrap: (
            ; mov eax, -3
            ; mov ebx, 33
            ; btr eax, ebx
        ) [CF ZF SF OF],
        btr_reg_word_1_wrap: (
            ; mov ax, 1
            ; mov bx, 16
            ; btr ax, bx
        ) [CF ZF SF OF],
        btr_reg_word_0xfffd_wrap: (
            ; mov ax, -3
            ; mov bx, 17
            ; btr ax, bx
        ) [CF ZF SF OF],
    }
    // test the "memory" case
    // this one allows to index not only bits inside the specified argument, but actually uses the bit offset/8 as byte offset
    // but this apparently is only allowed when offset is stored in the register but not for immediate offset??
    test_snippets! {
        btr_mem_0: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov eax, 0
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_1: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov eax, 0
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_2: (
            ; mov DWORD [MEM_ADDR as i32], 2
            ; mov eax, 1
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_0xfffffffd: (
            ; mov DWORD [MEM_ADDR as i32], -3
            ; mov eax, 1
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov eax, 32
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_0xfffffffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov eax, 33
            ; btr [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btr_mem_word_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov ax, 16
            ; btr [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        btr_mem_word_0xfffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov ax, 17
            ; btr [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        btr_mem_word_1_neg: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -16
            ; btr [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
        btr_mem_word_0x80_neg: (
            ; mov DWORD [MEM_ADDR as i32], -0x80
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -1
            ; btr [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
    }
}
mod btc {
    // these test register access
    // in this case the search is performed only inside the register - the offset is taken modulo register size
    test_snippets! {
        btc_reg_0: (
            ; mov eax, 0
            ; btc eax, 0
        ) [CF ZF SF OF],
        btc_reg_1: (
            ; mov eax, 1
            ; btc eax, 0
        ) [CF ZF SF OF],
        btc_reg_2: (
            ; mov eax, 2
            ; btc eax, 1
        ) [CF ZF SF OF],
        btc_reg_0xfffffffd: (
            ; mov eax, -3
            ; btc eax, 1
        ) [CF ZF SF OF],
        btc_reg_1_wrap: (
            ; mov eax, 1
            ; mov ebx, 32 // we can't pass these as an immediate!!
            ; btc eax, ebx
        ) [CF ZF SF OF],
        btc_reg_0xfffffffd_wrap: (
            ; mov eax, -3
            ; mov ebx, 33
            ; btc eax, ebx
        ) [CF ZF SF OF],
        btc_reg_word_1_wrap: (
            ; mov ax, 1
            ; mov bx, 16
            ; btc ax, bx
        ) [CF ZF SF OF],
        btc_reg_word_0xfffd_wrap: (
            ; mov ax, -3
            ; mov bx, 17
            ; btc ax, bx
        ) [CF ZF SF OF],
    }
    // test the "memory" case
    // this one allows to index not only bits inside the specified argument, but actually uses the bit offset/8 as byte offset
    // but this apparently is only allowed when offset is stored in the register but not for immediate offset??
    test_snippets! {
        btc_mem_0: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov eax, 0
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_1: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov eax, 0
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_2: (
            ; mov DWORD [MEM_ADDR as i32], 2
            ; mov eax, 1
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_0xfffffffd: (
            ; mov DWORD [MEM_ADDR as i32], -3
            ; mov eax, 1
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov eax, 32
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_0xfffffffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov eax, 33
            ; btc [MEM_ADDR as i32], eax
        ) [CF ZF SF OF],
        btc_mem_word_1_wrap: (
            ; mov DWORD [MEM_ADDR as i32], 0
            ; mov DWORD [MEM_ADDR as i32 + 4], 1
            ; mov ax, 16
            ; btc [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        btc_mem_word_0xfffd_wrap: (
            ; mov DWORD [MEM_ADDR as i32], -1
            ; mov DWORD [MEM_ADDR as i32 + 4], -3
            ; mov ax, 17
            ; btc [MEM_ADDR as i32], ax
        ) [CF ZF SF OF],
        btc_mem_word_1_neg: (
            ; mov DWORD [MEM_ADDR as i32], 1
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -16
            ; btc [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
        btc_mem_word_0x80_neg: (
            ; mov DWORD [MEM_ADDR as i32], -0x80
            ; mov DWORD [MEM_ADDR as i32 + 4], 0
            ; mov ax, -1
            ; btc [MEM_ADDR as i32 + 4], ax
        ) [CF ZF SF OF],
    }
}

mod stack {
    test_snippets! {
        push_eax_pop_ebx: (
            ; mov eax, 42
            ; push eax
            ; pop ebx
        ) [CF ZF SF OF],
        push_eax_ebx: (
            ; mov eax, 42
            ; push eax
            ; push ebx
            ; add esp, 8
            // want to return the stack back to the initial state, because a ret is added implicitly
        ) [CF ZF SF OF],

        // TODO: test leave instruction
        leave_fixed: (
            ; push DWORD 0x1337
            ; mov ebp, esp
            ; leave
            ; ret
        ) [CF ZF SF OF],

        enter_leave: (
            ; push ebp
            ; mov ebp, esp
            ; sub esp, 0x100

            ; leave
            ; ret
        ) [CF ZF SF OF],

        retn: (
            ; push 1211337
            ; call >funny
            ; ret

            ; funny:
            ;   mov eax, [esp+4]
            ;   retn 4
        ) [CF ZF SF OF],
    }

    test_snippets! {
        pushfd_pop_eax: (
            ; pushf
            ; pop eax
        ) [CF ZF SF OF],
        push_neg_one_popfd: (
            ; mov eax, -1
            ; push eax
            ; popf
        ) [CF ZF SF OF],
    }
}

mod string {
    mod forward {
        mod scas {
            test_snippets! {
                scasb_eq: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; scasb
                ) [CF ZF SF OF],
                scasb_less: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x10
                    ; scasb
                ) [CF ZF SF OF],
                scasb_greater: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x12
                    ; scasb
                ) [CF ZF SF OF],

                scasb_less_signed: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, -1
                    ; scasb
                ) [CF ZF SF OF],

                scasb_greater_signed: (
                    ; mov BYTE [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 2
                    ; scasb
                ) [CF ZF SF OF],


                scasb_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; mov ecx, 0x4
                    ; repe scasb
                ) [CF ZF SF OF],
                scasb_repe_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; mov ecx, 0x1
                    ; repe scasb
                ) [CF ZF SF OF],

                scasb_repne_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11001111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x0
                    ; mov ecx, 0x4
                    ; repne scasb
                ) [CF ZF SF OF],
                scasb_repne_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11001111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x0
                    ; mov ecx, 0x1
                    ; repne scasb
                ) [CF ZF SF OF],
            }
            test_snippets! {
                scasw_eq: (
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x11
                    ; scasw
                ) [CF ZF SF OF],
                scasw_less: (
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x10
                    ; scasw
                ) [CF ZF SF OF],
                scasw_greater: (
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x12
                    ; scasw
                ) [CF ZF SF OF],

                scasw_less_signed: (
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, -1
                    ; scasw
                ) [CF ZF SF OF],

                scasw_greater_signed: (
                    ; mov WORD [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 2
                    ; scasw
                ) [CF ZF SF OF],


                scasw_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x11
                    ; mov ecx, 0x4
                    ; repe scasw
                ) [CF ZF SF OF],
                scasw_repe_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x11
                    ; mov ecx, 0x1
                    ; repe scasw
                ) [CF ZF SF OF],

                scasw_repne_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x0
                    ; mov ecx, 0x4
                    ; repne scasw
                ) [CF ZF SF OF],
                scasw_repne_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x0
                    ; mov ecx, 0x1
                    ; repne scasw
                ) [CF ZF SF OF],
            }
            test_snippets! {
                scasd_eq: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11
                    ; scasd
                ) [CF ZF SF OF],
                scasd_less: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x10
                    ; scasd
                ) [CF ZF SF OF],
                scasd_greater: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x12
                    ; scasd
                ) [CF ZF SF OF],

                scasd_less_signed: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, -1
                    ; scasd
                ) [CF ZF SF OF],

                scasd_greater_signed: (
                    ; mov DWORD [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 2
                    ; scasd
                ) [CF ZF SF OF],


                scasd_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11
                    ; mov ecx, 0x4
                    ; repe scasd
                ) [CF ZF SF OF],
                scasd_repe_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11
                    ; mov ecx, 0x1
                    ; repe scasd
                ) [CF ZF SF OF],

                scasd_repne_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x0
                    ; mov ecx, 0x4
                    ; repne scasd
                ) [CF ZF SF OF],
                scasd_repne_1: (
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x0
                    ; mov ecx, 0x1
                    ; repne scasd
                ) [CF ZF SF OF],
            }
        }

        mod stos {
            test_snippets! {
                stosb_zero: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x0
                    ; stosb
                ) [CF ZF SF OF],
                stosb_0x11: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; stosb
                ) [CF ZF SF OF],
                stosb_0x11_twice: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; stosb
                    ; stosb
                ) [CF ZF SF OF],

                stosb_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x13
                    ; mov ecx, 0x0
                    ; rep stosb
                ) [CF ZF SF OF],
                stosb_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x13
                    ; mov ecx, 0x2
                    ; rep stosb
                ) [CF ZF SF OF],
                stosb_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x13
                    ; mov ecx, 0x4
                    ; rep stosb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                stosw_zero: (
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x0
                    ; stosw
                ) [CF ZF SF OF],
                stosw_0x1112: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1112
                    ; stosw
                ) [CF ZF SF OF],
                stosw_0x1112_twice: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1112
                    ; stosw
                    ; stosw
                ) [CF ZF SF OF],

                stosw_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1314
                    ; mov ecx, 0x0
                    ; rep stosw
                ) [CF ZF SF OF],
                stosw_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1314
                    ; mov ecx, 0x2
                    ; rep stosw
                ) [CF ZF SF OF],
                stosw_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1314
                    ; mov ecx, 0x4
                    ; rep stosw
                ) [CF ZF SF OF],
            }

            test_snippets! {
                stosd_zero: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x0
                    ; stosd
                ) [CF ZF SF OF],
                stosd_0x11121314: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11121314
                    ; stosd
                ) [CF ZF SF OF],
                stosd_0x11121314_twice: (
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11121314
                    ; stosd
                    ; stosd
                ) [CF ZF SF OF],

                stosd_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x0
                    ; rep stosd
                ) [CF ZF SF OF],
                stosd_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x2
                    ; rep stosd
                ) [CF ZF SF OF],
                stosd_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x4
                    ; rep stosd
                ) [CF ZF SF OF],
            }
        }

        mod movs {
            test_snippets! {
                movsb_0x11: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; movsb
                ) [CF ZF SF OF],
                movsb_0x00: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov esi, MEM_ADDR as i32 + 1
                    ; mov edi, MEM_ADDR as i32
                    ; movsb
                ) [CF ZF SF OF],

                movsb_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x0
                    ; rep movsb
                ) [CF ZF SF OF],
                movsb_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x2
                    ; rep movsb
                ) [CF ZF SF OF],
                movsb_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x4
                    ; rep movsb
                ) [CF ZF SF OF],
                // this should start wrapping
                movsb_rep_8: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x4
                    ; rep movsb
                ) [CF ZF SF OF],
                movsb_rep_420: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 420
                    ; rep movsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                movsw_0x11: (
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; movsw
                ) [CF ZF SF OF],
                movsw_0x00: (
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov esi, MEM_ADDR as i32 + 2
                    ; mov edi, MEM_ADDR as i32
                    ; movsw
                ) [CF ZF SF OF],

                movsw_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 8
                    ; mov ecx, 0x0
                    ; rep movsw
                ) [CF ZF SF OF],
                movsw_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 8
                    ; mov ecx, 0x2
                    ; rep movsw
                ) [CF ZF SF OF],
                movsw_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x4
                    ; rep movsw
                ) [CF ZF SF OF],
                // this should start wrapping
                movsw_rep_8: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 8
                    ; mov ecx, 0x4
                    ; rep movsw
                ) [CF ZF SF OF],
                movsw_rep_420: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 8
                    ; mov ecx, 420
                    ; rep movsw
                ) [CF ZF SF OF],
            }

            test_snippets! {
                movsd_0x11: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; movsd
                ) [CF ZF SF OF],
                movsd_0x00: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 4
                    ; mov edi, MEM_ADDR as i32
                    ; movsd
                ) [CF ZF SF OF],

                movsd_rep_0: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; mov ecx, 0x0
                    ; rep movsd
                ) [CF ZF SF OF],
                movsd_rep_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; mov ecx, 0x2
                    ; rep movsd
                ) [CF ZF SF OF],
                movsd_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; mov ecx, 0x4
                    ; rep movsd
                ) [CF ZF SF OF],
                // this should start wrapping
                movsd_rep_8: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; mov ecx, 0x4
                    ; rep movsd
                ) [CF ZF SF OF],
                movsd_rep_420: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; mov ecx, 420
                    ; rep movsd
                ) [CF ZF SF OF],
            }
        }

        mod cmps {
            test_snippets! {
                cmpsb_0x11_0x11: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x11_0x12: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x12
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x12_0x11: (
                    ; mov BYTE [MEM_ADDR as i32], 0x12
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x00_0x00: (
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x00
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],

                cmpsb_repne_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14121311
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x0
                    ; repne cmpsb
                ) [CF ZF SF OF],
                cmpsb_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x2
                    ; repe cmpsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                cmpsw_0x1111_0x1111: (
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1111
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1111_0x1212: (
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1212
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1212_0x1111: (
                    ; mov WORD [MEM_ADDR as i32], 0x1212
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1111
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1111_0x0000: (
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x0000
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],

                cmpsw_repne_2: (
                    ; mov DWORD [MEM_ADDR as i32], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14141313
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x12121112
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141313
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 8
                    ; mov ecx, 0x0
                    ; repne cmpsw
                ) [CF ZF SF OF],
                cmpsw_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14151313
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141313
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x2
                    ; repe cmpsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                cmpsd_0x11_0x11: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x11_0x12: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x12
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x12_0x11: (
                    ; mov DWORD [MEM_ADDR as i32], 0x12
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x00: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x00
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],

                cmpsd_repne_2: (
                    ; mov DWORD [MEM_ADDR as i32],      0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 4],  0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 8],  0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141414

                    ; mov DWORD [MEM_ADDR as i32 + 16], 0x11111112
                    ; mov DWORD [MEM_ADDR as i32 + 20], 0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 24], 0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 28], 0x14141414

                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; repne cmpsd
                ) [CF ZF SF OF],
                cmpsd_repe_4: (
                    ; mov DWORD [MEM_ADDR as i32],      0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 4],  0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 8],  0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141414

                    ; mov DWORD [MEM_ADDR as i32 + 16], 0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 20], 0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 24], 0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 28], 0x14141415

                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 16
                    ; repe cmpsd
                ) [CF ZF SF OF],
            }
        }
    }
    mod backwards {
        mod scas {
            test_snippets! {
                scasb_eq: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; scasb
                ) [CF ZF SF OF],
                scasb_less: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x10
                    ; scasb
                ) [CF ZF SF OF],
                scasb_greater: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x12
                    ; scasb
                ) [CF ZF SF OF],

                scasb_less_signed: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, -1
                    ; scasb
                ) [CF ZF SF OF],

                scasb_greater_signed: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 2
                    ; scasb
                ) [CF ZF SF OF],


                scasb_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x11
                    ; mov ecx, 0x4
                    ; repe scasb
                ) [CF ZF SF OF],
                scasb_repe_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x11
                    ; mov ecx, 0x1
                    ; repe scasb
                ) [CF ZF SF OF],

                scasb_repne_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11001111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x0
                    ; mov ecx, 0x4
                    ; repne scasb
                ) [CF ZF SF OF],
                scasb_repne_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11001111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x0
                    ; mov ecx, 0x1
                    ; repne scasb
                ) [CF ZF SF OF],
            }
            test_snippets! {
                scasw_eq: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x11
                    ; scasw
                ) [CF ZF SF OF],
                scasw_less: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x10
                    ; scasw
                ) [CF ZF SF OF],
                scasw_greater: (
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x12
                    ; scasw
                ) [CF ZF SF OF],

                scasw_less_signed: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, -1
                    ; scasw
                ) [CF ZF SF OF],

                scasw_greater_signed: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 2
                    ; scasw
                ) [CF ZF SF OF],


                scasw_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x11
                    ; mov ecx, 0x4
                    ; repe scasw
                ) [CF ZF SF OF],
                scasw_repe_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x11
                    ; mov ecx, 0x1
                    ; repe scasw
                ) [CF ZF SF OF],

                scasw_repne_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x0
                    ; mov ecx, 0x4
                    ; repne scasw
                ) [CF ZF SF OF],
                scasw_repne_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00110011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x0
                    ; mov ecx, 0x1
                    ; repne scasw
                ) [CF ZF SF OF],
            }
            test_snippets! {
                scasd_eq: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11
                    ; scasd
                ) [CF ZF SF OF],
                scasd_less: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x10
                    ; scasd
                ) [CF ZF SF OF],
                scasd_greater: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x12
                    ; scasd
                ) [CF ZF SF OF],

                scasd_less_signed: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, -1
                    ; scasd
                ) [CF ZF SF OF],

                scasd_greater_signed: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], -1
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 2
                    ; scasd
                ) [CF ZF SF OF],


                scasd_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x11
                    ; mov ecx, 0x4
                    ; repe scasd
                ) [CF ZF SF OF],
                scasd_repe_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x11
                    ; mov ecx, 0x1
                    ; repe scasd
                ) [CF ZF SF OF],

                scasd_repne_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x0
                    ; mov ecx, 0x4
                    ; repne scasd
                ) [CF ZF SF OF],
                scasd_repne_1: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x0
                    ; mov ecx, 0x1
                    ; repne scasd
                ) [CF ZF SF OF],
            }
        }

        mod stos {
            test_snippets! {
                stosb_zero: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x0
                    ; stosb
                ) [CF ZF SF OF],
                stosb_0x11: (
                    ; std
                    ; mov edi, MEM_ADDR as i32
                    ; mov al, 0x11
                    ; stosb
                ) [CF ZF SF OF],
                stosb_0x11_twice: (
                    ; std
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; mov al, 0x11
                    ; stosb
                    ; stosb
                ) [CF ZF SF OF],

                stosb_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x13
                    ; mov ecx, 0x0
                    ; rep stosb
                ) [CF ZF SF OF],
                stosb_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x13
                    ; mov ecx, 0x2
                    ; rep stosb
                ) [CF ZF SF OF],
                stosb_rep_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 3
                    ; mov al, 0x13
                    ; mov ecx, 0x4
                    ; rep stosb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                stosw_zero: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x0
                    ; stosw
                ) [CF ZF SF OF],
                stosw_0x1112: (
                    ; std
                    ; mov edi, MEM_ADDR as i32
                    ; mov ax, 0x1112
                    ; stosw
                ) [CF ZF SF OF],
                stosw_0x1112_twice: (
                    ; std
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; mov ax, 0x1112
                    ; stosw
                    ; stosw
                ) [CF ZF SF OF],

                stosw_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x1314
                    ; mov ecx, 0x0
                    ; rep stosw
                ) [CF ZF SF OF],
                stosw_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x1314
                    ; mov ecx, 0x2
                    ; rep stosw
                ) [CF ZF SF OF],
                stosw_rep_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 6
                    ; mov ax, 0x1314
                    ; mov ecx, 0x4
                    ; rep stosw
                ) [CF ZF SF OF],
            }

            test_snippets! {
                stosd_zero: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x0
                    ; stosd
                ) [CF ZF SF OF],
                stosd_0x11121314: (
                    ; std
                    ; mov edi, MEM_ADDR as i32
                    ; mov eax, 0x11121314
                    ; stosd
                ) [CF ZF SF OF],
                stosd_0x11121314_twice: (
                    ; std
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov eax, 0x11121314
                    ; stosd
                    ; stosd
                ) [CF ZF SF OF],

                stosd_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x0
                    ; rep stosd
                ) [CF ZF SF OF],
                stosd_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x2
                    ; rep stosd
                ) [CF ZF SF OF],
                stosd_rep_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x11121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x11121111
                    ; mov edi, MEM_ADDR as i32 + 12
                    ; mov eax, 0x13141516
                    ; mov ecx, 0x4
                    ; rep stosd
                ) [CF ZF SF OF],
            }
        }

        mod movs {
            test_snippets! {
                movsb_0x11: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; movsb
                ) [CF ZF SF OF],
                movsb_0x00: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov esi, MEM_ADDR as i32 + 1
                    ; mov edi, MEM_ADDR as i32
                    ; movsb
                ) [CF ZF SF OF],

                movsb_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 3
                    ; mov edi, MEM_ADDR as i32 + 7
                    ; mov ecx, 0x0
                    ; rep movsb
                ) [CF ZF SF OF],
                movsb_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 3
                    ; mov edi, MEM_ADDR as i32 + 7
                    ; mov ecx, 0x2
                    ; rep movsb
                ) [CF ZF SF OF],
                movsb_rep_4: (
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; mov ecx, 0x4
                    ; rep movsb
                ) [CF ZF SF OF],
                // this should start wrapping
                movsb_rep_8: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 3
                    ; mov edi, MEM_ADDR as i32 + 7
                    ; mov ecx, 0x4
                    ; rep movsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                movsw_0x11: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; movsw
                ) [CF ZF SF OF],
                movsw_0x00: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1112
                    ; mov esi, MEM_ADDR as i32 + 2
                    ; mov edi, MEM_ADDR as i32
                    ; movsw
                ) [CF ZF SF OF],

                movsw_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x0
                    ; rep movsw
                ) [CF ZF SF OF],
                movsw_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x2
                    ; rep movsw
                ) [CF ZF SF OF],
                movsw_rep_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x4
                    ; rep movsw
                ) [CF ZF SF OF],
                // this should start wrapping
                movsw_rep_8: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15161718
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x4
                    ; rep movsw
                ) [CF ZF SF OF],
            }

            test_snippets! {
                movsd_0x11: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 4
                    ; movsd
                ) [CF ZF SF OF],
                movsd_0x00: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov esi, MEM_ADDR as i32 + 4
                    ; mov edi, MEM_ADDR as i32
                    ; movsd
                ) [CF ZF SF OF],

                movsd_rep_0: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; mov ecx, 0x0
                    ; rep movsd
                ) [CF ZF SF OF],
                movsd_rep_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; mov ecx, 0x2
                    ; rep movsd
                ) [CF ZF SF OF],
                movsd_rep_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; mov ecx, 0x4
                    ; rep movsd
                ) [CF ZF SF OF],
                // this should start wrapping
                movsd_rep_8: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32+4], 0x15161718
                    ; mov DWORD [MEM_ADDR as i32+8], 0x191a1b1c
                    ; mov DWORD [MEM_ADDR as i32+12], 0x1d1e1f20
                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; mov ecx, 0x4
                    ; rep movsd
                ) [CF ZF SF OF],
            }
        }

        mod cmps {
            test_snippets! {
                cmpsb_0x11_0x11: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x11_0x12: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x12
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x12_0x11: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x12
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],
                cmpsb_0x00_0x00: (
                    ; std
                    ; mov BYTE [MEM_ADDR as i32], 0x11
                    ; mov BYTE [MEM_ADDR as i32 + 1], 0x00
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsb
                ) [CF ZF SF OF],

                cmpsb_repne_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14121311
                    ; mov esi, MEM_ADDR as i32 + 3
                    ; mov edi, MEM_ADDR as i32 + 7
                    ; mov ecx, 0x0
                    ; repne cmpsb
                ) [CF ZF SF OF],
                cmpsb_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11121314
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x15121314
                    ; mov esi, MEM_ADDR as i32 + 3
                    ; mov edi, MEM_ADDR as i32 + 7
                    ; mov ecx, 0x2
                    ; repe cmpsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                cmpsw_0x1111_0x1111: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1111
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1111_0x1212: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1212
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1212_0x1111: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1212
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x1111
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],
                cmpsw_0x1111_0x0000: (
                    ; std
                    ; mov WORD [MEM_ADDR as i32], 0x1111
                    ; mov WORD [MEM_ADDR as i32 + 2], 0x0000
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 2
                    ; cmpsw
                ) [CF ZF SF OF],

                cmpsw_repne_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14141313
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x12121112
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141313
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x0
                    ; repne cmpsw
                ) [CF ZF SF OF],
                cmpsw_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 4], 0x14151313
                    ; mov DWORD [MEM_ADDR as i32 + 8], 0x12121111
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141313
                    ; mov esi, MEM_ADDR as i32 + 6
                    ; mov edi, MEM_ADDR as i32 + 14
                    ; mov ecx, 0x2
                    ; repe cmpsb
                ) [CF ZF SF OF],
            }

            test_snippets! {
                cmpsd_0x11_0x11: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x11_0x12: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x12
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x12_0x11: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x12
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x11
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],
                cmpsd_0x00: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32], 0x11
                    ; mov DWORD [MEM_ADDR as i32 + 1], 0x00
                    ; mov esi, MEM_ADDR as i32
                    ; mov edi, MEM_ADDR as i32 + 1
                    ; cmpsd
                ) [CF ZF SF OF],

                cmpsd_repne_2: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32],      0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 4],  0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 8],  0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141414

                    ; mov DWORD [MEM_ADDR as i32 + 16], 0x11111112
                    ; mov DWORD [MEM_ADDR as i32 + 20], 0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 24], 0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 28], 0x14141414

                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; repne cmpsd
                ) [CF ZF SF OF],
                cmpsd_repe_4: (
                    ; std
                    ; mov DWORD [MEM_ADDR as i32],      0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 4],  0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 8],  0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 12], 0x14141414

                    ; mov DWORD [MEM_ADDR as i32 + 16], 0x11111111
                    ; mov DWORD [MEM_ADDR as i32 + 20], 0x12121212
                    ; mov DWORD [MEM_ADDR as i32 + 24], 0x13131313
                    ; mov DWORD [MEM_ADDR as i32 + 28], 0x14141415

                    ; mov esi, MEM_ADDR as i32 + 12
                    ; mov edi, MEM_ADDR as i32 + 28
                    ; repe cmpsd
                ) [CF ZF SF OF],
            }
        }
    }
}
