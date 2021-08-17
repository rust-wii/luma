/*
 * Linkscript for Wii
 */

OUTPUT_FORMAT("elf32-powerpc");
OUTPUT_ARCH(powerpc:common);

EXTERN(_start);
ENTRY(_start);

PHDRS
{
  stub PT_LOAD FLAGS(5);
  text PT_LOAD FLAGS(5);
  data PT_LOAD FLAGS(6);
  bss1 PT_LOAD;
  bss2 PT_LOAD;
}

SECTIONS
{
	/* stub is loaded at physical address 0x00003400 (though both 0x80003400 and 0x00003400 are equivalent for IOS) */
	/* This can also be used to load an arbitrary standalone stub at an arbitrary address in memory, for any purpose */
	/* Use -Wl,--section-start,.stub=0xADDRESS to change */
	. = 0x00003400;

	.stub :
	{
		KEEP(*(.stub))
	} :stub = 0

	/* default base address */
	/* use -Wl,--section-start,.init=0xADDRESS to change */
	. = 0x80004000;

	/* Program */
	.init	: {
		KEEP (*crt0.o(*.init))
		KEEP (*(.init))
	} :text = 0
	
	.text	: {
		*(.text)
		*(.text.*)
		. = ALIGN(32);   /* REQUIRED. LD is flaky without it. */
	} = 0

	.rodata   : { *(.rodata) *(.rodata.*) } :data

	.sdata2	: {
		PROVIDE(_SDA2_BASE_ = .);
		*(.sdata2)
		*(.sdata2.*)
	}

	. = ALIGN(32 / 8);
	.data	: {
		*(.data)
		*(.data.*)
		SORT(CONSTRUCTORS)
		. = ALIGN(32);   /* REQUIRED. LD is flaky without it. */
	}

	.sdata	: {
		PROVIDE(_SDA_BASE_ = .);
		*(.sdata)
		*(.sdata.*)
		. = ALIGN(32);   /* REQUIRED. LD is flaky without it. */
	}

	.sbss	: {
		__sbss_start = .;
		PROVIDE (__sbss_start = .);
		PROVIDE (___sbss_start = .);
		*(.sbss)
		*(.sbss.*)
		PROVIDE (__sbss_end = .);
		PROVIDE (___sbss_end = .);
		. = ALIGN(32);   /* REQUIRED. LD is flaky without it. */
		__sbss_end = .;
	} :bss1

	.bss	: {
		__bss_start = .;
		PROVIDE (__bss_start = .);
		*(.bss)
		*(.bss.*)
		*(COMMON)
	/*	Align here to ensure that the .bss section occupies space up to
		_end.  Align after .bss to ensure correct alignment even if the
		.bss section disappears because there are no input sections.  */

		. = ALIGN(32);

		PROVIDE (__bss_end = .);
		__bss_end = .;
	} :bss2
}

__stack_addr = (__bss_start + SIZEOF(.bss) + 0x20000 + 7) & (-8);
__stack_end = (__bss_start + SIZEOF(.bss));

PROVIDE(__stack_addr = __stack_addr);
PROVIDE(__stack_end = __stack_end);