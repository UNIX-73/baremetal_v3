.global _start
_start:
    ldr w0, kernel_entry32
    br x0

.ltorg

.org 0xf0
.global stub_magic
stub_magic:
    .word 0x5afe570b

.org 0xf4
.global stub_version
stub_version:
    .word 0

.org 0xfc
.global kernel_entry32
kernel_entry32:
    .word 0x0