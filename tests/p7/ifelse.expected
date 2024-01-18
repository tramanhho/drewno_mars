.globl main

.text
main:	pushq %rbp
	movq %rsp,	%rbp
	addq $16,	%rbp
	subq $0,	%rsp

	callq magic
	movq %rdi,	%rax
	movq %rax,	-24(%rbp)
	movb -24(%rbp),	%al
	cmpb $0,	%al
	je		lbl_2
	movq $1,	%rdi
	callq printInt
	jmp		lbl_1

lbl_2:	nop
	movq $0,	%rdi
	callq printInt

lbl_1:	nop

	jmp		lbl_0
lbl_0:	addq $0,	%rsp
	popq %rbp
	retq

	movq $60,	%rax
	movq $1,	%rdi
	
	syscall
