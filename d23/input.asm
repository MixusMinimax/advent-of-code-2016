cpy a b   ; a=12, b=12
dec b     ; b=11
cpy a d   ; d=12
cpy 0 a   ; a=0
cpy b c   ; c=11
inc a     ; a=11
dec c     ; c=0
jnz c -2  ;
dec d     ; d=11
jnz d -5  ; a = 11*11
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 96 c
jnz 91 d
inc a
inc d
jnz d -2
inc c
jnz c -5
