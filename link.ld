OUTPUT_ARCH("riscv")
ENTRY( _start )

SU_INS_BASE = 0x00000000;
SU_DATA_BASE = 0x00080000;

INS_BASE = SU_INS_BASE;
DATA_BASE = SU_DATA_BASE;

DATA_SIZE = 4K;            /* Bytes */

_STACK_SIZE       = 512;  /* Bytes */
_STACK_FENCE_SIZE = 16;   /* Bytes */

__stack_pointer$  = DATA_BASE + DATA_SIZE;

SECTIONS
{
	/* reset vector addr */
	/*. = 0x200; */
	. = INS_BASE;
	.text._start   : { *(.text._start) }
	.text   : { *(.text._main) }

	. = DATA_BASE;
	.sdata   : { *(.sdata) }
	.sbss   : { *(.sbss) }
	.data    : { *(.data) }
	.bss     : { *(.bss) }
	.rodata  : { *(.rodata) }
	.tohost  : { *(.tohost) }

	. = DATA_BASE + DATA_SIZE - _STACK_SIZE - _STACK_FENCE_SIZE;
	._stack_fence : { *(._stack_fence) }
	/* ._stack       : { boot.o(._stack) } */

	_end = .;
}
