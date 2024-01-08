.globl main
.data
gbl_i: .quad 0
gbl_j: .quad 0
str0: .asciz "\n"

.text
main:	pushq %rbp
	movq %rsp,	%rbp
	addq $16,	%rbp
	subq $64,	%rsp
	movq $1,	(gbl_i)
	movq (gbl_i),	%rax
	notq %rax
	movq %rax,	-32(%rbp)
	movq -32(%rbp),	%rax
	movq $1,	%rbx
	imulq		%rbx
	movq %rax,	-40(%rbp)
	movq -40(%rbp),	%rax
	movq $2,	%rbx
	idivq		%rbx
	movq %rax,	-48(%rbp)
	movq -48(%rbp),	%rax
	movq $3,	%rbx
	addq %rbx,	%rax
	movq %rax,	-56(%rbp)
	movq $4,	%rax
	movq $5,	%rbx
	imulq		%rbx
	movq %rax,	-64(%rbp)
	movq -56(%rbp),	%rax
	movq -64(%rbp),	%rbx
	subq %rbx,	%rax
	movq %rax,	-72(%rbp)
	movq $6,	%rax
	movq $7,	%rbx
	idivq		%rbx
	movq %rax,	-80(%rbp)
	movq -72(%rbp),	%rax
	movq -80(%rbp),	%rbx
	addq %rbx,	%rax
	movq %rax,	-88(%rbp)
	movq -88(%rbp),	%rax
	movq %rax,	-24(%rbp)
	movq -24(%rbp),	%rdi
	callq printInt
	movq (str0),	%rdi
	callq printString
lbl_0:	addq $64,	%rsp
	popq %rbp
	retq

	movq $60,	%rax
	movq $1,	%rdi
	
	syscall
