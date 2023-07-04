# baremetal boot code for C program
# To turn off linker relaxation:
#    command line gcc linker switch: -Wl,--no-relax
#    or assembler: .option norelax  , possibly with .option push & .option pop wrapping it
# To get verbatim RV32I code, also requires that 16b code (RV32C) optimization be turned off
#     gcc command line option: -mno-16-bit
#     or assembler: .option norvc

   .global _main   # external

   .option norelax            # turn off linker relaxation
   .option norvc              # only 32b instructions

   .section .text._start,"aw",@progbits
   .global _start
   .type _start, @function
_start:
   .option push
   .option norelax
   la gp, __global_pointer$
   .option pop

   # la sp, __stack_pointer$  # cause warning

   la a0, __stack_pointer$
   mv sp, a0
   # enable I$ and D$
   #csrwi 0x7ca,0x3
   csrsi 0x7d0, 0x10 # enable ACE in mmisc_ctl (custom CSR)
//#ifdef CACHE_ECCEN    
#if 0
   csrr t1, 0x7ca
   or  t1, t1, 0x2b
   csrw 0x7ca, t1 # enable I and D Cache with ECC
#endif

//#ifdef ILM_DLM_ECCEN
#if 0 
   csrr t1, 0x7c0
   or t1, t1, 0x4
   csrw 0x7c0, t1 # enable ILM ecc
   csrr t1, 0x7c1
   or t1, t1, 0x4
   csrw 0x7c1, t1 # enable DLM ecc
#endif

   call _main

exit:
   lui t1, %hi(tohost)
1: sw a0,  %lo(tohost)(t1)
   j 1b


   .section ._stack_fence,"aw",@progbits
   #.rept _STACK_FENCE_SIZE/4
   #.word 0
   #.endr
   .word 0, 0, 0, 0

   /* This is optional */
   #.section ._stack,"aw",@nobits
   #.comm _stack, _STACK_SIZE


   .section .tohost,"aw",@progbits
   .align 6
   .global tohost
tohost:
   .word 0
   .global fromhost
fromhost:
   .word 0

