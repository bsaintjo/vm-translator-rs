// Push(Segment { segment: Constant, index: 3030 })
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: Pointer, index: 0 })
@SP
M=M-1
@SP
A=M
D=M
@THIS
M=D
// Push(Segment { segment: Constant, index: 3040 })
@3040
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: Pointer, index: 1 })
@SP
M=M-1
@SP
A=M
D=M
@THAT
M=D
// Push(Segment { segment: Constant, index: 32 })
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(This), index: 2 })
@THIS
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
// Push(Segment { segment: Constant, index: 46 })
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: LATT(That), index: 6 })
@THAT
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
// Push(Segment { segment: Pointer, index: 0 })
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Pointer, index: 1 })
@THAT
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
// Push(Segment { segment: LATT(This), index: 2 })
@THIS
D=M
@2
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
// Push(Segment { segment: LATT(That), index: 6 })
@THAT
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
(END)
@END
0;JMP
