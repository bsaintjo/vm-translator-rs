use std::str::FromStr;

use crate::{assembly::Assembly, babel::ParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentType {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
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

    pub(crate) fn as_asm(&self) -> Assembly {
        match self.segment {
            SegmentType::Pointer => todo!(),
            SegmentType::Constant => Assembly::Address(self.index as u32),
            SegmentType::Local => Assembly::local(),
            SegmentType::Static => todo!(),
            SegmentType::Argument => Assembly::argument(),
            SegmentType::This => Assembly::this(),
            SegmentType::That => Assembly::that(),
            SegmentType::Temp => todo!()
        }
    }
}

impl FromStr for SegmentType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(SegmentType::Argument),
            "local" => Ok(SegmentType::Local),
            "static" => Ok(SegmentType::Static),
            "constant" => Ok(SegmentType::Constant),
            "this" => Ok(SegmentType::This),
            "that" => Ok(SegmentType::That),
            "pointer" => Ok(SegmentType::Pointer),
            "temp" => Ok(SegmentType::Temp),
            _ => Err(ParseError::InvalidSegment(s.to_string())),
        }
    }
}
