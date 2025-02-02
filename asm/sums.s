section .data
    msg db 'Sum: %d', 0xA, 0  ; Message to print

section .text
    global _start

_start:
    ; Calculate 2 + 3
    mov rax, 2
    add rax, 3

    ; Print the result
    mov rdi, msg  ; Format string
    mov rsi, rax  ; Value to print
    call printf

    ; Exit the program
    mov rax, 60   ; syscall: exit
    xor rdi, rdi  ; exit code 0
    syscall
