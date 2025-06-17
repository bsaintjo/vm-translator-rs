use std::{borrow::Cow, fmt::Display};

type StringLike = Cow<'static, str>;

#[derive(Debug, Clone)]
pub enum Assembly {
    Label(StringLike),
    Comment(StringLike),
    Address(u32),
    AddressSymbol(StringLike),
    Command {
        dest: Option<Dest>,
        comp: Comp,
        jump: Option<Jump>,
    },
}

impl Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assembly::Label(s) => write!(f, "({s})"),
            Assembly::Comment(s) => write!(f, "// {s}"),
            Assembly::Address(a) => write!(f, "@{a}"),
            Assembly::AddressSymbol(a) => write!(f, "@{a}"),
            Assembly::Command { dest, comp, jump } => {
                if let Some(dest) = dest {
                    dest.fmt(f)?;
                }
                comp.fmt(f)?;
                if let Some(jump) = jump {
                    jump.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}

impl Assembly {
    pub fn comment<S>(s: S) -> Assembly
    where
        S: Into<StringLike>,
    {
        Self::Comment(s.into())
    }

    pub fn label<S>(s: S) -> Assembly
    where
        S: Into<StringLike>,
    {
        Self::Label(s.into())
    }

    pub fn addr_sym<S>(s: S) -> Assembly
    where
        S: Into<StringLike>,
    {
        Self::AddressSymbol(s.into())
    }

    pub fn sp() -> Self {
        Self::addr_sym("SP")
    }

    pub fn local() -> Self {
        Self::addr_sym("LCL")
    }

    pub fn argument() -> Self {
        Self::addr_sym("ARG")
    }

    pub fn this() -> Self {
        Self::addr_sym("THIS")
    }

    pub fn that() -> Self {
        Self::addr_sym("THAT")
    }

    pub fn reg13() -> Self {
        Self::addr_sym("R13")
    }

    pub fn reg14() -> Self {
        Self::addr_sym("R14")
    }

    pub fn reg15() -> Self {
        Self::addr_sym("R15")
    }

    pub fn assign(dest: Dest, comp: Comp) -> Self {
        Self::Command {
            dest: Some(dest),
            comp,
            jump: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Dest {
    M,
    D,
    A,
    DM,
    AM,
    AD,
    ADM,
}

impl Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}=")
    }
}

#[derive(Debug, Clone)]
pub enum Comp {
    Zero,
    /// A
    A,
    /// M
    M,
    /// D
    D,
    /// M + 1
    Mplus1,
    /// D + M
    DplusM,
    /// D + A
    DplusA,
    /// D - M
    DminusM,
    /// M - D
    MminusD,
    /// D - 1
    Dminus1,
    /// M - 1
    Mminus1,
    /// -M
    NegateM,
    /// !M
    NotM,
    /// D&M
    DandM,
    /// D|M
    DorM,
}

impl Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comp::Zero => write!(f, "0"),
            Comp::Mplus1 => write!(f, "M+1"),
            Comp::DplusM => write!(f, "D+M"),
            Comp::DplusA => write!(f, "D+A"),
            Comp::DminusM => write!(f, "D-M"),
            Comp::MminusD => write!(f, "M-D"),
            Comp::Mminus1 => write!(f, "M-1"),
            Comp::Dminus1 => write!(f, "D-1"),
            Comp::NotM => write!(f, "!M"),
            Comp::NegateM => write!(f, "-M"),
            Comp::DandM => write!(f, "D&M"),
            Comp::DorM => write!(f, "D|M"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Jump {
    JLE,
    JEQ,
    JGT,
    JLT,
    JMP,
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ";{self:?}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let res = Dest::M;
        let x = format!("{res}");
        assert_eq!(x, "M=");
    }
}
