.code

GetTEB proc
	mov rax, qword ptr gs:[30h]
	ret
GetTEB endp

CustomError proc
	xor eax, eax
	call getTEB
	mov eax, dword ptr [rax+68h] ; LastErrorValue
	ret
CustomError endp

end
