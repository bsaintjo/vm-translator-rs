use std::str::FromStr;

use crate::assembly::{Assembly, Comp, Dest, Jump};

pub struct Babel {
    counter: usize,
}

impl Babel {
    pub fn empty() -> Self {
        Self { counter: 0 }
    }

    pub fn translate(&mut self, cmd: &Command) -> Vec<Assembly> {
        let mut translator = Translation::new();
        match cmd {
            Command::Push(Segment::Constant, x) => {
                translator.with_asm([
                    Assembly::comment(format!("{cmd:?}")),
                    // @x // where x is a constant
                    Assembly::Address(*x as u32),
                    // D = A
                    Assembly::assign(Dest::D, Comp::A),
                    // @SP
                    Assembly::sp(),
                    // A = M // Go to location SP was pointing to
                    Assembly::assign(Dest::A, Comp::M),
                    // M = D
                    Assembly::assign(Dest::M, Comp::D),
                    // @SP
                    Assembly::sp(),
                    // M = M + 1
                    Assembly::assign(Dest::M, Comp::Mplus1),
                ]);
            }
            Command::Add => {
                translator.with_asm([
                    Assembly::comment(format!("{cmd:?}")),
                    // Pop 1st value, put into D
                    // @SP
                    Assembly::sp(),
                    // M = M - 1 // Decrement to go to next value
                    Assembly::assign(Dest::M, Comp::Mminus1),
                    // A = M
                    Assembly::assign(Dest::A, Comp::M),
                    // D = M
                    Assembly::assign(Dest::D, Comp::M),
                    // @SP
                    Assembly::sp(),
                    // M = M - 1
                    Assembly::assign(Dest::M, Comp::Mminus1),
                    // Pop 2nd value, add to D
                    // A = M
                    Assembly::assign(Dest::A, Comp::M),
                    // D = D + M
                    Assembly::assign(Dest::D, Comp::DplusM),
                    // Add value to stack
                    // @SP
                    Assembly::sp(),
                    // A = M
                    Assembly::assign(Dest::A, Comp::M),
                    // M = D // Addition on stack
                    Assembly::assign(Dest::M, Comp::D),
                    // @SP
                    Assembly::sp(),
                    // M = M + 1
                    Assembly::assign(Dest::M, Comp::Mplus1),
                ]);
            }
            Command::Equal => {
                translator.ord_asm(&mut self.counter, Jump::JEQ);
                // self.counter += 1;
                // translator.with_asm([
                //     // @SP
                //     Assembly::sp(),
                //     // M = M - 1 // Decrement to go to next value
                //     Assembly::assign(Dest::M, Comp::Mminus1),
                //     // A = M
                //     Assembly::assign(Dest::A, Comp::M),
                //     // D = M
                //     Assembly::assign(Dest::D, Comp::M),
                //     // @SP
                //     Assembly::sp(),
                //     // M = M - 1
                //     Assembly::assign(Dest::M, Comp::Mminus1),
                //     // A = M
                //     Assembly::assign(Dest::A, Comp::M),
                //     // @EQ{counter}
                //     Assembly::addr_sym(format!("@EQ{}", self.counter)),
                //     // D = D - M; JEQ
                //     Assembly::Command {
                //         dest: Some(Dest::D),
                //         comp: Comp::DminusM,
                //         jump: Some(Jump::JEQ),
                //     },
                //     // @32767 // -1
                //     Assembly::Address(32767),
                //     // D = A
                //     Assembly::assign(Dest::D, Comp::A),
                //     // @SP
                //     Assembly::sp(),
                //     // A = M
                //     Assembly::assign(Dest::A, Comp::M),
                //     // M = D
                //     Assembly::assign(Dest::M, Comp::D),
                //     // @AFTER{counter}
                //     Assembly::addr_sym(format!("AFTER{}", self.counter)),
                //     // 0;JMP
                //     Assembly::Command {
                //         dest: None,
                //         comp: Comp::Zero,
                //         jump: Some(Jump::JMP),
                //     },
                //     // (EQ{counter}) // D = 0 here
                //     Assembly::label(format!("EQ{}", self.counter)),
                //     // @SP
                //     Assembly::sp(),
                //     // A = M
                //     Assembly::assign(Dest::A, Comp::M),
                //     // M = D
                //     Assembly::assign(Dest::M, Comp::D),
                //     // (AFTER{counter})
                //     Assembly::label(format!("AFTER{}", self.counter)),
                //     // @SP
                //     Assembly::sp(),
                //     // M = M + 1
                //     Assembly::assign(Dest::M, Comp::Mplus1),
                // ]);
            }
            Command::LessThan => {
                translator.ord_asm(&mut self.counter, Jump::JLT);
            }
            Command::GreaterThan => {
                translator.ord_asm(&mut self.counter, Jump::JGT);
            }
            Command::Negate => {
                todo!()
            }
            Command::Not => {
                todo!()
            }
            _ => todo!(),
        }
        translator.0
    }
}

struct Translation(Vec<Assembly>);

impl Translation {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, asm: Assembly) -> &mut Self {
        self.0.push(asm);
        self
    }

    fn comment(&mut self, cmd: &Command) -> &mut Self {
        self.push(Assembly::comment(format!("{cmd:?}")))
    }

    fn with_asm<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Assembly>,
    {
        self.0.extend(iter);
        self
    }

    fn unary_asm(&mut self, m_comp: Comp) -> &mut Self {
        self.with_asm([
            Assembly::comment(format!("Unary {}", m_comp)),
            // @SP
            Assembly::sp(),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // M = UnaryOperator(M)
            Assembly::assign(Dest::M, m_comp),
        ]);
        self
    }

    /// Generate assembly for Ordinal functions like equal, less than, greater than
    fn ord_asm(&mut self, counter: &mut usize, jump: Jump) -> &mut Self {
        *counter += 1;
        self.with_asm([
            // @SP
            Assembly::sp(),
            // M = M - 1 // Decrement to go to next value
            Assembly::assign(Dest::M, Comp::Mminus1),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // D = M
            Assembly::assign(Dest::D, Comp::M),
            // @SP
            Assembly::sp(),
            // M = M - 1
            Assembly::assign(Dest::M, Comp::Mminus1),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // @EQ{counter}
            Assembly::addr_sym(format!("@{}{}", jump, counter)),
            // D = D - M; JEQ
            Assembly::Command {
                dest: Some(Dest::D),
                comp: Comp::DminusM,
                jump: Some(jump),
            },
            // @32767 // -1
            Assembly::Address(32767),
            // D = A
            Assembly::assign(Dest::D, Comp::A),
            // @SP
            Assembly::sp(),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // M = D
            Assembly::assign(Dest::M, Comp::D),
            // @AFTER{counter}
            Assembly::addr_sym(format!("AFTER{}", counter)),
            // 0;JMP
            Assembly::Command {
                dest: None,
                comp: Comp::Zero,
                jump: Some(Jump::JMP),
            },
            // (EQ{counter}) // D = 0 here
            Assembly::label(format!("{}{}", jump, counter)),
            // @SP
            Assembly::sp(),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // M = D
            Assembly::assign(Dest::M, Comp::D),
            // (AFTER{counter})
            Assembly::label(format!("AFTER{}", counter)),
            // @SP
            Assembly::sp(),
            // M = M + 1
            Assembly::assign(Dest::M, Comp::Mplus1),
        ]);
        self
    }

    fn goto_sp(&mut self) -> &mut Self {
        self.with_asm([
            // @SP
            Assembly::sp(),
            // A = M // Go to location SP was pointing to
            Assembly::assign(Dest::A, Comp::M),
        ])
    }

    fn inc_sp(&mut self) -> &mut Self {
        self.with_asm([
            // @SP
            Assembly::sp(),
            // M = M + 1
            Assembly::assign(Dest::M, Comp::Mplus1),
        ])
    }
    fn dec_sp(&mut self) -> &mut Self {
        self.with_asm([
            // @SP
            Assembly::sp(),
            // M = M - 1
            Assembly::assign(Dest::M, Comp::Mminus1),
        ])
    }

    /// Go to value pointed by
    // fn with_push<I>(&mut self, iter: I) -> &mut Self
    // where
    //     I: Iterator<Item = Assembly>,
    // {
    //     self.goto_sp().with_asm(iter).inc_sp()
    // }

    fn pop_d(&mut self) -> &mut Self {
        self.with_asm([
            // @SP
            Assembly::sp(),
            // M = M - 1
            Assembly::assign(Dest::M, Comp::Mminus1),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // D = M
            Assembly::assign(Dest::D, Comp::M),
        ])
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Push(Segment, i32),
    Pop(Segment, i32),
    Add,
    Subtract,
    Negate,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("not a valid command")]
    InvalidCommand(String),
    #[error("not a valid segment")]
    InvalidSegment(String),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split_ascii_whitespace();
        match ss.next() {
            Some("push") => {
                let segment = ss.next().unwrap().parse::<Segment>()?;
                let location = ss.next().unwrap().parse::<i32>().unwrap();
                Ok(Command::Push(segment, location))
            }
            Some("pop") => {
                let segment = ss.next().unwrap().parse::<Segment>()?;
                let location = ss.next().unwrap().parse::<i32>().unwrap();
                Ok(Command::Pop(segment, location))
            }
            Some("add") => Ok(Command::Add),
            Some("sub") => Ok(Command::Subtract),
            Some("eq") => Ok(Command::Equal),
            _ => Err(ParseError::InvalidCommand(format!("Not valid: {}", s))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Segment {
    Stack,
    Pointer,
    Constant,
    Local,
    Static,
    Argument,
    This,
    That,
    Temp,
    R13,
    R14,
    R15,
}

impl FromStr for Segment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "constant" => Ok(Segment::Constant),
            "local" => Ok(Segment::Local),
            _ => Err(ParseError::InvalidSegment(s.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "push constant 5".parse::<Command>().unwrap(),
            Command::Push(Segment::Constant, 5)
        );
        assert_eq!(
            "pop local 7".parse::<Command>().unwrap(),
            Command::Pop(Segment::Local, 7)
        );
        assert_eq!("add".parse::<Command>().unwrap(), Command::Add);
    }
}
