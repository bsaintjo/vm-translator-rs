use std::str::FromStr;

pub mod latt;
pub mod pointer;
pub mod segment;
pub mod statics;
pub mod temp;

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
