.global exit

add r4, #2

bge exit
b Loop


exit:
mov r4, #5
ldr r0,=mes3
bl printf

.data
mes3:
.string "Third Message"