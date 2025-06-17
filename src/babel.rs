use std::str::FromStr;

use crate::assembly::{Assembly, Comp, Dest, Jump};
use crate::segment::{self, Segment, SegmentType};

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
            Command::Push(segment::Segment {
                segment: segment::SegmentType::Constant,
                index: x,
            }) => {
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
            Command::Push(segment)
                if matches!(
                    segment.segment,
                    SegmentType::Local
                        | SegmentType::Argument
                        | SegmentType::This
                        | SegmentType::That
                ) =>
            {
                translator.comment(cmd);
                translator.with_asm([
                    // @Segment
                    segment.as_asm(),
                    // D = M // Store segment address in D
                    Assembly::assign(Dest::D, Comp::M),
                    // @x
                    Assembly::Address(segment.index as u32),
                    // D = D + A
                    Assembly::assign(Dest::D, Comp::DplusA),
                    // A = D // Go to address of segment + offset
                    Assembly::assign(Dest::A, Comp::D),
                    // Store that value into D register
                    Assembly::assign(Dest::D, Comp::M),
                    // Store D register into area pointed by stack pointer
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
            Command::Pop(
                segment @ Segment {
                    segment: SegmentType::Temp,
                    ..
                },
            ) => {
                // STORE TEMP(5 + i) into D register
                translator.with_asm([
                    Assembly::Address(5),
                    Assembly::assign(Dest::D, Comp::A),
                    Assembly::Address(segment.index as u32),
                    Assembly::assign(Dest::D, Comp::DplusA),
                    Assembly::assign(Dest::A, Comp::D),
                    Assembly::assign(Dest::D, Comp::M),
                ]);
                // Store address into reg13
                translator.store_dreg_in_reg13();
                // Store SP into D register and decrement
                translator.with_asm([
                    // @SP
                    Assembly::sp(),
                    // M = M - 1
                    Assembly::assign(Dest::M, Comp::Mminus1),
                    // D = M
                    Assembly::assign(Dest::D, Comp::M),
                ]);
                // Store D Register into RAM[reg13]
                translator.with_asm([
                    Assembly::reg13(),
                    Assembly::assign(Dest::A, Comp::M),
                    Assembly::assign(Dest::M, Comp::D),
                ]);
            }
            Command::Push(
                segment @ Segment {
                    segment: SegmentType::Temp,
                    ..
                },
            ) => {
                translator.comment(cmd);
                // Store TEMP=(5 + i) into D Register
                translator.with_asm([
                    Assembly::Address(5),
                    Assembly::assign(Dest::D, Comp::A),
                    Assembly::Address(segment.index as u32),
                    Assembly::assign(Dest::D, Comp::DplusA),
                    Assembly::assign(Dest::A, Comp::D),
                    Assembly::assign(Dest::D, Comp::M),
                ]);
                // Store D register into area pointed by stack pointer
                translator.push_dreg_onto_stack_and_increment_sp();
            }
            Command::Push(..) => {
                todo!()
            }
            Command::Pop(segment) => {
                translator.comment(cmd);
                // addr <- segment + offset
                // Store addr in D register
                translator.with_asm([
                    // @Segment
                    segment.as_asm(),
                    // D = M // Store segment address in D
                    Assembly::assign(Dest::D, Comp::M),
                    // @x
                    Assembly::Address(segment.index as u32),
                    // D = D + A
                    Assembly::assign(Dest::D, Comp::DplusA),
                    // A = D // Go to address of segment + offset
                    Assembly::assign(Dest::A, Comp::D),
                    // Store address into D register
                    Assembly::assign(Dest::D, Comp::A),
                ]);
                // Store address into R13 temp
                translator.store_dreg_in_reg13();
                // SP--
                translator.with_asm([
                    // @SP
                    Assembly::sp(),
                    // M = M - 1
                    Assembly::assign(Dest::M, Comp::Mminus1),
                    // D = M
                    Assembly::assign(Dest::D, Comp::M),
                ]);
                // RAM[addr] <- RAM[SP]
                translator.with_asm([
                    // @R13
                    Assembly::reg13(),
                    // Go to address stored in register 13
                    Assembly::assign(Dest::A, Comp::M),
                    // Store D register value into Ram[A]
                    Assembly::assign(Dest::M, Comp::D),
                ]);
            }
            Command::Add => {
                translator.push(Assembly::comment("addition"));
                translator.binary_asm(Comp::DplusM);
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
            } // _ => unreachable!(),
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

    pub fn store_dreg_in_reg13(&mut self) {
        self.with_asm([
            //@R13
            Assembly::reg13(),
            // M = D
            Assembly::assign(Dest::M, Comp::D),
        ]);
    }

    pub fn push_dreg_onto_stack_and_increment_sp(&mut self) {
        self.with_asm([
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
    ///
    /// Counter is used to generate unique jump locations
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
    Push(segment::Segment),
    Pop(segment::Segment),
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
                let segment = ss.next().unwrap().parse::<segment::SegmentType>()?;
                let location = ss.next().unwrap().parse::<i32>().unwrap();
                Ok(Command::Push(segment::Segment::new(segment, location)))
            }
            Some("pop") => {
                let segment = ss.next().unwrap().parse::<segment::SegmentType>()?;
                let location = ss.next().unwrap().parse::<i32>().unwrap();
                Ok(Command::Pop(segment::Segment::new(segment, location)))
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

#[cfg(test)]
mod test {
    use crate::segment::Segment;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "push constant 5".parse::<Command>().unwrap(),
            Command::Push(Segment::new(SegmentType::Constant, 5))
        );
        assert_eq!(
            "pop local 7".parse::<Command>().unwrap(),
            Command::Pop(Segment::new(SegmentType::Local, 7))
        );
        assert_eq!("add".parse::<Command>().unwrap(), Command::Add);
    }
}
