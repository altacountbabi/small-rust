SECTIONS
{
  .text : { *(.text) }
  .data : { *(.data) }
  .bss : { *(.bss) }

  .comment : { *(.comment) } :DISCARD
}
