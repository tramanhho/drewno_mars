.globl main
.data
gbl_a: .quad 0

.text
main:	pushq %rbp
	movq %rsp,	%rbp
	addq $16,	%rbp
	subq $0,	%rsp
	movq $1,	(gbl_a)
	movq (gbl_a),	%rdi
	callq printInt
lbl_0:	addq $0,	%rsp
	popq %rbp
	retq

	movq $60,	%rax
	movq $1,	%rdi
	
	syscall
