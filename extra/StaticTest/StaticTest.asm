// Push(Segment { segment: Constant, index: 111 })
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Constant, index: 333 })
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Constant, index: 888 })
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop(Segment { segment: Static, index: 8 })
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.8
M=D
// Pop(Segment { segment: Static, index: 3 })
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.3
M=D
// Pop(Segment { segment: Static, index: 1 })
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.1
M=D
// Push(Segment { segment: Static, index: 3 })
@StaticTest.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push(Segment { segment: Static, index: 1 })
@StaticTest.1
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
// Push(Segment { segment: Static, index: 8 })
@StaticTest.8
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
