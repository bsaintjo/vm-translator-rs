use std::str::FromStr;

use crate::assembly::{Assembly, Comp, Dest, Jump};

pub struct Babel {
    counter: usize,
}

impl Babel {
    pub fn empty() -> Self {
        Self { counter: 0 }
    }

    pub fn translate(&mut self, cmd: &Command) -> Translation {
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
                    Assembly::comment("addition"),
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
            Command::Subtract => {
                translator.push(Assembly::comment("subtract"));
                translator.binary_asm(Comp::MminusD);
            }
            Command::Equal => {
                translator.push(Assembly::comment("equal"));
                translator.ord_asm(&mut self.counter, Jump::JEQ);
            }
            Command::LessThan => {
                translator.push(Assembly::comment("less than"));
                translator.ord_asm(&mut self.counter, Jump::JGT);
            }
            Command::GreaterThan => {
                translator.push(Assembly::comment("greater than"));
                translator.ord_asm(&mut self.counter, Jump::JLT);
            }
            Command::Negate => {
                translator.push(Assembly::comment("negation"));
                translator.unary_asm(Comp::NegateM);
            }
            Command::Not => {
                translator.push(Assembly::comment("not"));
                translator.unary_asm(Comp::NotM);
            }
            Command::And => {
                translator.push(Assembly::comment("and cmd"));
                translator.binary_asm(Comp::DandM);
            }
            Command::Or => {
                translator.push(Assembly::comment("or cmd"));
                translator.binary_asm(Comp::DorM);
            }
            _ => todo!("{cmd:?}"),
        }
        translator
    }
}

#[derive(Debug, Clone)]
pub struct Translation(Vec<Assembly>);

impl Translation {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn finish() -> Self {
        let mut t = Self::new();
        t.with_asm([
            Assembly::label("END"),
            Assembly::addr_sym("END"),
            Assembly::Command {
                dest: None,
                comp: Comp::Zero,
                jump: Some(Jump::JMP),
            },
        ]);
        t
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
            // M = M - 1 // Decrement to go to next value
            Assembly::assign(Dest::M, Comp::Mminus1),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // M = UnaryOperator(M)
            Assembly::assign(Dest::M, m_comp),
            // @SP
            Assembly::sp(),
            // M = M + 1
            Assembly::assign(Dest::M, Comp::Mplus1),
        ]);
        self
    }

    fn binary_asm(&mut self, dm_comp: Comp) -> &mut Self {
        self.with_asm([
            // Assembly::comment(format!("{cmd:?}")),
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
            Assembly::assign(Dest::D, dm_comp),
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
            // D = D - M
            Assembly::assign(Dest::D, Comp::DminusM),
            // @EQ{counter}
            Assembly::addr_sym(format!("{:?}{}", jump, counter)),
            // D; JEQ/JLT/etc.
            Assembly::Command {
                dest: None,
                comp: Comp::D,
                jump: Some(jump),
            },
            // From here the condition is false
            // @0
            Assembly::Address(0),
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
            Assembly::label(format!("{:?}{}", jump, counter)),
            // @0
            Assembly::Address(0),
            // D = A
            Assembly::assign(Dest::D, Comp::A),
            // @SP
            Assembly::sp(),
            // A = M
            Assembly::assign(Dest::A, Comp::M),
            // M = D
            Assembly::assign(Dest::M, Comp::Dminus1),
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

impl IntoIterator for Translation {
    type Item = Assembly;

    type IntoIter = std::vec::IntoIter<Assembly>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
    #[error("not a valid command: {0}")]
    InvalidCommand(String),
    #[error("not a valid segment: {0}")]
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
            Some("lt") => Ok(Command::LessThan),
            Some("gt") => Ok(Command::GreaterThan),
            Some("neg") => Ok(Command::Negate),
            Some("not") => Ok(Command::Not),
            Some("or") => Ok(Command::Or),
            Some("and") => Ok(Command::And),
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
