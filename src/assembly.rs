use std::{
    borrow::Cow,
    fmt::Display,
};

type StringLike = Cow<'static, str>;

#[derive(Debug)]
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

    pub fn assign(dest: Dest, comp: Comp) -> Self {
        Self::Command {
            dest: Some(dest),
            comp,
            jump: None,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    /// D - M
    DminusM,
    /// M - 1
    Mminus1,
}

impl Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comp::Zero => write!(f, "0"),
            Comp::Mplus1 => write!(f, "M+1"),
            Comp::DplusM => write!(f, "D+M"),
            Comp::DminusM => write!(f, "D-M"),
            Comp::Mminus1 => write!(f, "M-1"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug)]
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
