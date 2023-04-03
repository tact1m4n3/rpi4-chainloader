.section .text.boot

.macro ADR_REL register, symbol
    adrp \register, \symbol
    add \register, \register, #:lo12:\symbol
.endm

.macro ADR_ABS register, symbol
    movz \register, #:abs_g2:\symbol
    movk \register, #:abs_g1_nc:\symbol
    movk \register, #:abs_g0_nc:\symbol
.endm

.globl start
start:
    mrs x4, mpidr_el1
    and x4, x4, #3
    cbnz x4, 6f

1:  ADR_ABS x4, __bss_start
    ADR_ABS x5, __bss_end

2:  cmp x4, x5
    beq 3f
    stp xzr, xzr, [x4], #16
    b 2b

3:  ADR_REL x4, __binary_start
    ADR_ABS x5, __binary_start
    ADR_ABS x6, __binary_end

4:  cmp x5, x6
    beq 5f
    ldr x3, [x4], #8
    str x3, [x5], #8
    b 4b

5:  ADR_ABS x4, __boot_stack
    mov sp, x4

    ADR_ABS x5, rust_start
    br x5

6:  wfe
    b 6b
