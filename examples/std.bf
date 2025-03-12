; ADD
; >x   y
; >x+y 0
#A >[<+>-]< ; Adds p0 and p1 and saves to p0

; SUBSTRACT
; >0 x   y 0 0
; >0 x-y 0 0 0
#S 0>>[<[->]<]>>[[<+>-]>>]<<<

; MULTIPLY
; >x y 0    0
;  0 0 >x*y 0
#M [>[->+>+<< ]>>[-<<+>>]<<<-]>>

; DIVMOD
; >n d     1     0   0 0
; >0 d-n%d n%d+1 n/d 0 0
#D [->-[>+>>]>[[-<+>]+>+>>]<<<<<]

; DUPLICATE
; >n 0 0
; >n n 0
#U [>+>+<<-]>>[<<+>>-]<<

; ZERO OUT 
; >n
; >0
#0 [-]

; SHIFT
; >n *
; >0 n
#S [>+<-]0

; SWAP
: >x >y <0
