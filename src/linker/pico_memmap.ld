
MEMORY 
{
  BOOT2 (rx)    : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH (rx)    : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM   (xrw)   : ORIGIN = 0x20000000, LENGTH = 248K 
  RAM4  (xrw)   : ORIGIN = 0x2003E000, LENGTH = 4K
  RAM5  (xrw)   : ORIGIN = 0x2003F000, LENGTH = 4K 
}

SECTIONS {
  .boot2 ORIGIN(BOOT2) :
  {
    __boot2_start__ = .;
    KEEP (*(.boot2));
    __boot2_end__ = .;
  } > BOOT2
} INSERT BEFORE .text;

EXTERN(__reset_handler_c0);
ENTRY(__reset_handler_c0);

EXTERN(default_handler);
EXTERN(HardFault);

SECTIONS 
{
  .vector_table ORIGIN(FLASH) :
  {
    __vector_table = .;
    KEEP (*(.vector_table));
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  .text _stext : 
  {
    __stext = .;
    *(.__reset_handler_c0);
    *(.text  .text.*);
    . = ALIGN(4);
    __etext = .;
  } > FLASH

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*); 
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
  } > RAM AT >FLASH
  . = ALIGN(4);
  __edata = .;

  __sidata = LOADADDR(.data);

  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
  } > FLASH

  . = ALIGN(32);
  __veneer_limit = .;

  .bss (NOLOAD) : ALIGN(4) 
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
  } > RAM

  . = ALIGN(4);
  __ebss = .;
  
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __uninit_start = .;
    *(.uninit .uninit.*);
    __uninit_end = .;
  } > RAM

  PROVIDE(__heap_start = __uninit_end);

  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  .vector_table_c0 (NOLOAD) : {
    KEEP (*(.vector_table_c0));
  } > RAM4

  .vector_table_c1 (NOLOAD) : {
    KEEP (*(.vector_table_c1));
  } > RAM5

  /DISCARD/ :
  {
     *(.ARM.exidx);
     *(.ARM.exidx.*);
     *(.ARM.extab.*);
  }
}


