.global Loop

mov r0, r1
mov r1, #1
mov r2, #0xaf
mov r3, #0xe2

mov r7, #0x3f0

strb r0, [r7], #1
strb r1, [r7], #1
strb r2, [r7], #1
strb r3, [r7]
//ldr r6 ,= #0xffff0ab0

mov r7, #0x3f0
ldr r5, [r7]


mes0:
.string "Other message"
mes2:
Mes0:
.string "Enter a number."

ldr r0 ,= Mes0
bl printf
bl cr
bl getnumber
bl value
bl cr

Loop:
add r0, #4
cmp r0, #50




