use crate::{
    assembly::{Assembly, Comp, Dest, Jump},
    commands::{
        latt::{pop_latt, push_latt},
        pointer::{pop_pointer, push_pointer},
        segment::{Segment, SegmentType},
        statics::{pop_static, push_static},
        temp::{pop_temp, push_temp},
        Command,
    },
};

pub struct Babel {
    counter: usize,
    basename: String,
}

impl Babel {
    pub fn empty<S: Into<String>>(basename: S) -> Self {
        Self {
            counter: 0,
            basename: basename.into(),
        }
    }

    pub fn translate(&mut self, cmd: &Command) -> Translation {
        let mut translator = Translation::new();
        translator.comment(cmd);
        match cmd {
            // Pointer
            Command::Push(Segment {
                segment: SegmentType::Pointer,
                index,
            }) => push_pointer(&mut translator, *index),

            Command::Pop(Segment {
                segment: SegmentType::Pointer,
                index,
            }) => pop_pointer(&mut translator, *index),

            // Constant
            Command::Push(Segment {
                segment: SegmentType::Constant,
                index: x,
            }) => {
                translator.with_asm([
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

            // Local/Argument/This/That
            Command::Push(Segment {
                segment: SegmentType::LATT(latt),
                index,
            }) => push_latt(&mut translator, *latt, *index as u32),
            Command::Pop(Segment {
                segment: SegmentType::LATT(latt),
                index,
            }) => pop_latt(&mut translator, *latt, *index as u32),

            // TEMP
            Command::Pop(Segment {
                segment: SegmentType::Temp,
                index,
            }) => pop_temp(&mut translator, *index as u32),

            Command::Push(Segment {
                segment: SegmentType::Temp,
                index,
            }) => push_temp(&mut translator, *index as u32),

            // Static
            Command::Pop(Segment {
                segment: SegmentType::Static,
                index,
            }) => pop_static(&mut translator, *index as u32, &self.basename),
            Command::Push(Segment {
                segment: SegmentType::Static,
                index,
            }) => push_static(&mut translator, *index as u32, &self.basename),

            // Catch unimplemented
            Command::Pop(..) => {
                todo!()
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
            }
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

    pub fn store_sp_to_dreg(&mut self) {
        self.with_asm([
            Assembly::sp(),
            Assembly::assign(Dest::A, Comp::M),
            Assembly::assign(Dest::D, Comp::M),
        ]);
    }

    pub fn store_dreg_to_sp(&mut self) {
        self.with_asm([
            Assembly::sp(),
            Assembly::assign(Dest::A, Comp::M),
            Assembly::assign(Dest::M, Comp::D),
        ]);
    }

    pub fn increment_sp(&mut self) {
        self.with_asm([Assembly::sp(), Assembly::assign(Dest::M, Comp::Mplus1)]);
    }

    pub fn decrement_sp(&mut self) {
        self.with_asm([Assembly::sp(), Assembly::assign(Dest::M, Comp::Mminus1)]);
    }

    pub fn store_dreg_in_reg13(&mut self) {
        self.with_asm([
            //@R13
            Assembly::reg13(),
            // M = D
            Assembly::assign(Dest::M, Comp::D),
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

    pub fn with_asm<I>(&mut self, iter: I) -> &mut Self
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

#[cfg(test)]
mod test {
    use crate::commands::{segment::LATT, Command};

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "push constant 5".parse::<Command>().unwrap(),
            Command::Push(Segment::new(SegmentType::Constant, 5))
        );
        assert_eq!(
            "pop local 7".parse::<Command>().unwrap(),
            Command::Pop(Segment::new(SegmentType::LATT(LATT::Local), 7))
        );
        assert_eq!("add".parse::<Command>().unwrap(), Command::Add);
    }
}
