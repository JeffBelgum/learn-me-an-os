global start

section .text
bits 32
start:
  mov esp, stack_top

  call check_multiboot
  call check_cpuid
  call check_long_mode

  ; print `OK` to screen
  mov dword [0xb8000], 0x2f4b2f4f
  hlt

; Prints `Err: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
error:
  mov dword [0xb8000], 0x4f524f45
  mov dword [0xb8004], 0x4f3a4f52
  mov dword [0xb8008], 0x4f204f20
  mov byte  [0xb800a], al
  hlt

; test processor feature support
check_multiboot:
  cmp eax, 0x36d76289
  jne .no_multiboot
  ret
.no_multiboot:
  mov al, "0"
  jmp error

check_cpuid:
  ; Copy FLAGS into EAX via stack
  pushfd
  pop eax

  ; Copy ECX as well for comparing later on
  mov ecx, eax

  ; Flip the ID bit
  xor eax, 1 << 21

  ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
  pushfd
  pop eax

  ; Restore FLAGS from the old version stored in ECX
  push ecx
  popfd

  ; Compare EAX and ECX. If they are equal then that means the bit
  ; wasn't flipped, and CPUID isn't supported.
  cmp eax, ecx
  je .no_cpuid
  ret
.no_cpuid:
  mov al, "1"
  jmp error

check_long_mode:
  ; test if extended proc info is available
  mov eax, 0x80000000  ; implicit arg for cpuid
  cpuid                ; get highest supported arg
  cmp eax, 0x80000001  ; it needs to be at least 0x80000001
  jb .no_long_mode     ; if it's less, the CPU is too old for long mode

  ; use extended info to test if long mode is abailable
  mov eax, 0x80000001  ; argument for extended proc info
  cpuid                ; returns various feature bits in exc and edx
  test edx, 1 << 29    ; test if LM-bit is set in the D-register
  jz .no_long_mode     ; If it's not set, there is no long mode
  ret
.no_long_mode:
  mov al, "2"
  jmp error



section .bss
align 4096
p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096

stack_bottom:
  resb 64
stack_top:
