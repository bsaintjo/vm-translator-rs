// Push(Segment { segment: Constant, index: 10 })
@10
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(Local), index: 0 })
@LCL
D=M
@0
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Push(Segment { segment: Constant, index: 21 })
@21
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Constant, index: 22 })
@22
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(Argument), index: 2 })
@ARG
D=M
@2
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Pop(Segment { segment: LATT(Argument), index: 1 })
@ARG
D=M
@1
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Push(Segment { segment: Constant, index: 36 })
@36
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(This), index: 6 })
@THIS
D=M
@6
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Push(Segment { segment: Constant, index: 42 })
@42
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Constant, index: 45 })
@45
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(That), index: 5 })
@THAT
D=M
@5
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Pop(Segment { segment: LATT(That), index: 2 })
@THAT
D=M
@2
D=D+A
@R13
M=D
@SP
M=M-1
@SP
A=M
D=M
@R13
A=M
M=D
// Push(Segment { segment: Constant, index: 510 })
@510
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: Temp, index: 6 })
@SP
M=M-1
@SP
A=M
D=M
@11
M=D
// Push(Segment { segment: LATT(Local), index: 0 })
@LCL
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: LATT(That), index: 5 })
@THAT
D=M
@5
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// Add
// addition
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: LATT(Argument), index: 1 })
@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// Subtract
// subtract
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: LATT(This), index: 6 })
@THIS
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: LATT(This), index: 6 })
@THIS
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// Add
// addition
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
// Subtract
// subtract
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Temp, index: 6 })
@11
D=M
@SP
A=M
M=D
@SP
M=M+1
// Add
// addition
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
(END)
@END
0;JMP
