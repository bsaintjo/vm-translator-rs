use std::str::FromStr;

use crate::{assembly::Assembly, commands::ParseError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LATT {
    Local,
    Argument,
    This,
    That,
}

impl LATT {
    pub fn as_asm(self) -> Assembly {
        match self {
            LATT::Local => Assembly::local(),
            LATT::Argument => Assembly::argument(),
            LATT::This => Assembly::this(),
            LATT::That => Assembly::that(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentType {
    LATT(LATT),
    Static,
    Constant,
    Pointer,
    Temp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub(crate) segment: SegmentType,
    pub(crate) index: i32,
}

impl Segment {
    pub fn new(segment: SegmentType, index: i32) -> Self {
        Self { segment, index }
    }
}

impl FromStr for SegmentType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(SegmentType::LATT(LATT::Argument)),
            "local" => Ok(SegmentType::LATT(LATT::Local)),
            "static" => Ok(SegmentType::Static),
            "constant" => Ok(SegmentType::Constant),
            "this" => Ok(SegmentType::LATT(LATT::This)),
            "that" => Ok(SegmentType::LATT(LATT::That)),
            "pointer" => Ok(SegmentType::Pointer),
            "temp" => Ok(SegmentType::Temp),
            _ => Err(ParseError::InvalidSegment(s.to_string())),
        }
    }
}
