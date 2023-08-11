.code

GetPEB proc
	mov rax, gs:[60h]
	ret
GetPEB endp

BeingDebugged proc
	xor eax, eax
	call GetPEB
	movzx eax, byte ptr [rax + 2h]		; PEB -> BeingDebugged
	ret
BeingDebugged endp

end
