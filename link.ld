OUTPUT_ARCH("riscv")
ENTRY( _start )
/*
ILM_BASE = 0;
DLM_BASE = 2M;
EM_BASE = 3M;*/

SU_ILM_BASE = 0x00000000;
SU_DLM_BASE = 0x00080000;
/*DEV_REGION_BASE  = 10000000h;*/
DEV_REGION_BASE  = 0x00400000;
/*DEV_REGION_BASE  = 0x04000000;*/

ILM_BASE = SU_ILM_BASE;
DLM_BASE = SU_DLM_BASE;
DDR_BASE = 0x80000000;

DLM_SIZE = 4K;            /* Bytes */

_STACK_SIZE       = 512;  /* Bytes */
_STACK_FENCE_SIZE = 16;   /* Bytes */

__stack_pointer$  = DLM_BASE + DLM_SIZE;

SECTIONS
{
	/* reset vector addr */
	/*. = 0x200; */
	. = ILM_BASE;
	.text._start   : { *(.text._start) }
	.text   : { *(.text._main) }

	. = DLM_BASE;
	/* PROVIDE ( __global_pointer$ = . + 4K/2 ); */
	/*__global_pointer$ = (DLM_BASE + 4K/2); */
	__ddr_global_pointer = 0x80000000;
	.sdata   : { *(.sdata) }
	.sbss   : { *(.sbss) }
	.data    : { *(.data) }
	.bss     : { *(.bss) }
	.rodata  : { *(.rodata) }
	.tohost  : { *(.tohost) }

	. = DLM_BASE + DLM_SIZE - _STACK_SIZE - _STACK_FENCE_SIZE;
	._stack_fence : { *(._stack_fence) }
	/* ._stack       : { boot.o(._stack) } */

	_end = .;
}
