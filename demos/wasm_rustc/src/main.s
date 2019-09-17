	.text
	.file	"main.7rcbfp3g-cgu.0"
	.section	.text._ZN3std2rt10lang_start17hac2180291b77cdc4E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17hac2180291b77cdc4E
	.globl	_ZN3std2rt10lang_start17hac2180291b77cdc4E
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17hac2180291b77cdc4E,@function
_ZN3std2rt10lang_start17hac2180291b77cdc4E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	leaq	.L__unnamed_1(%rip), %rax
	movq	%rdi, 32(%rsp)
	leaq	32(%rsp), %rcx
	movq	%rcx, %rdi
	movq	%rsi, 24(%rsp)
	movq	%rax, %rsi
	movq	24(%rsp), %rax
	movq	%rdx, 16(%rsp)
	movq	%rax, %rdx
	movq	16(%rsp), %rcx
	callq	*_ZN3std2rt19lang_start_internal17hb7a7311aaa9a4a4dE@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN3std2rt10lang_start17hac2180291b77cdc4E, .Lfunc_end0-_ZN3std2rt10lang_start17hac2180291b77cdc4E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*(%rdi)
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE
	movl	%eax, 4(%rsp)
	movl	4(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E, .Lfunc_end1-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E
	.cfi_endproc

	.section	.text._ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE,@function
_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE:
	.cfi_startproc
	movzbl	(%rdi), %eax
	retq
.Lfunc_end2:
	.size	_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE, .Lfunc_end2-_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE
	.cfi_endproc

	.section	.text._ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E,@function
_ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E:
	.cfi_startproc
	subq	$16, %rsp
	.cfi_def_cfa_offset 24
	movq	%rdi, %rax
	movq	$0, (%rsp)
	movq	%rsi, (%rdi)
	movq	%rdx, 8(%rdi)
	movq	(%rsp), %rdx
	movq	8(%rsp), %rsi
	movq	%rdx, 16(%rdi)
	movq	%rsi, 24(%rdi)
	movq	%rcx, 32(%rdi)
	movq	%r8, 40(%rdi)
	addq	$16, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E, .Lfunc_end3-_ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E
	movl	%eax, 12(%rsp)
	movl	12(%rsp), %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E,@function
_ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp0:
	leaq	8(%rsp), %rdi
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E
.Ltmp1:
	movl	%eax, 4(%rsp)
	jmp	.LBB5_1
.LBB5_1:
	jmp	.LBB5_2
.LBB5_2:
	movl	4(%rsp), %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB5_3:
	.cfi_def_cfa_offset 48
	jmp	.LBB5_4
.LBB5_4:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
.LBB5_5:
.Ltmp2:
	movq	%rax, 24(%rsp)
	movl	%edx, 32(%rsp)
	jmp	.LBB5_3
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E, .Lfunc_end5-_ZN4core3ops8function6FnOnce9call_once17h180a048e94f66917E
	.cfi_endproc
	.section	.gcc_except_table,"a",@progbits
	.p2align	2
GCC_except_table5:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end5-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2

	.section	.text._ZN4core3ptr18real_drop_in_place17h57ec936c75967012E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr18real_drop_in_place17h57ec936c75967012E,@function
_ZN4core3ptr18real_drop_in_place17h57ec936c75967012E:
	.cfi_startproc
	retq
.Lfunc_end6:
	.size	_ZN4core3ptr18real_drop_in_place17h57ec936c75967012E, .Lfunc_end6-_ZN4core3ptr18real_drop_in_place17h57ec936c75967012E
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	xorl	%edi, %edi
	callq	_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E
	movl	%eax, 4(%rsp)
	movl	4(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end7:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE, .Lfunc_end7-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h606e76703f28fb3dE
	.cfi_endproc

	.section	".text._ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E,@function
_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movb	%dil, 7(%rsp)
	leaq	7(%rsp), %rdi
	callq	_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217hfc470e6be4de7c0aE
	movl	%eax, (%rsp)
	movl	(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E, .Lfunc_end8-_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h196914dab8f41a73E
	.cfi_endproc

	.section	.text._ZN4main4main17h6b07ac6c6f06cdd6E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4main4main17h6b07ac6c6f06cdd6E,@function
_ZN4main4main17h6b07ac6c6f06cdd6E:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	leaq	.L__unnamed_2(%rip), %rax
	xorl	%ecx, %ecx
	movl	%ecx, %r8d
	leaq	8(%rsp), %rdi
	movq	%rax, %rsi
	movl	$1, %edx
	movl	$8, %ecx
	callq	_ZN4core3fmt9Arguments6new_v117h5d1aeee66e959867E
	leaq	8(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h7cb73078f8bf5907E@GOTPCREL(%rip)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_ZN4main4main17h6b07ac6c6f06cdd6E, .Lfunc_end9-_ZN4main4main17h6b07ac6c6f06cdd6E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movslq	%edi, %rax
	leaq	_ZN4main4main17h6b07ac6c6f06cdd6E(%rip), %rdi
	movq	%rsi, (%rsp)
	movq	%rax, %rsi
	movq	(%rsp), %rdx
	callq	_ZN3std2rt10lang_start17hac2180291b77cdc4E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	main, .Lfunc_end10-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	_ZN4core3ptr18real_drop_in_place17h57ec936c75967012E
	.quad	8
	.quad	8
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb0e0b2f0f3401520E
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h90420044647f93a7E
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"Hello, world!\n"
	.size	.L__unnamed_3, 14

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_3
	.asciz	"\016\000\000\000\000\000\000"
	.size	.L__unnamed_2, 16

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality

	.section	".note.GNU-stack","",@progbits
