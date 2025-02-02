.text
.globl example_sum

example_sum:
    xorl    %eax, %eax        # Zero out eax (return value)
    cmpl    $2, %edi          # Compare input (edi) with 2
    jl      .LBB0_2           # If edi < 2, jump to print and return

    # Calculate (n-2)*(n-3)/2 + 2n -3
    leal    -2(%rdi), %eax    # eax = edi - 2
    leal    -3(%rdi), %ecx    # ecx = edi - 3
    imulq   %rax, %rcx        # rcx = (edi-2) * (edi-3)
    shrq    %rcx              # Divide by 2 via right shift
    leal    (%rcx,%rdi,2), %eax # eax = rcx + 2*edi
    addl    $-3, %eax         # eax -= 3

.LBB0_2:
    push    %rax              # Save return value
    sub     $24, %rsp         # Allocate buffer (24 bytes), keeps stack aligned

    mov     %eax, 24(%rsp)    # Retrieve saved value (from push rax)
    lea     23(%rsp), %rdi    # Point to end of buffer (24 bytes: 0-23)
    movb    $0, (%rdi)        # Null terminator
    dec     %rdi              # Move to position 22
    movb    $0x0A, (%rdi)     # Store newline
    mov     $10, %ecx         # Divisor (for conversion)

    test    %eax, %eax        # Check if value is zero
    jz      .Lhandle_zero     # Handle zero case

.Lconvert_loop:
    xor     %edx, %edx        # Clear edx for division
    div     %ecx              # eax = eax/10, edx = remainder
    add     $'0', %dl         # Convert remainder to ASCII
    dec     %rdi              # Move buffer pointer back
    mov     %dl, (%rdi)       # Store digit
    test    %eax, %eax        # Check if quotient is zero
    jnz     .Lconvert_loop    # Continue if not zero
    jmp     .Lprint

.Lhandle_zero:
    dec     %rdi              # Move to position 21
    movb    $'0', (%rdi)      # Store '0' for zero case

.Lprint:
    mov     %rdi, %rsi        # Buffer start address
    lea     22(%rsp), %rdx    # Address of newline (position 22)
    sub     %rsi, %rdx        # Calculate length (newline - buffer start)
    inc     %rdx              # Include newline in length

    mov     $1, %eax          # syscall: write
    mov     $1, %edi          # fd: stdout
    syscall                   # Invoke syscall

    add     $24, %rsp         # Deallocate buffer
    pop     %rax              # Restore return value
    ret
