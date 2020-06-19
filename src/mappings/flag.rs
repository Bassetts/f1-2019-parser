use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_i8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum Flag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

#[non_exhaustive]
pub struct InvalidFlag(());

impl InvalidFlag {
    fn new() -> Self {
        InvalidFlag(())
    }
}

impl TryFrom<i8> for Flag {
    type Error = InvalidFlag;

    fn try_from(item: i8) -> Result<Self, Self::Error> {
        match item {
            -1 => Ok(Flag::Unknown),
            0 => Ok(Flag::None),
            1 => Ok(Flag::Green),
            2 => Ok(Flag::Blue),
            3 => Ok(Flag::Yellow),
            4 => Ok(Flag::Red),
            _ => Err(InvalidFlag::new()),
        }
    }
}

impl Flag {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_i8, Flag::try_from)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_parse() {
        let packet = (-1i8).to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::Unknown)));

        let packet = 0i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::None)));

        let packet = 1i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::Green)));

        let packet = 2i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::Blue)));

        let packet = 3i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::Yellow)));

        let packet = 4i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Flag::Red)));

        let packet = (-2i8).to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));

        let packet = 5i8.to_le_bytes();
        let result = Flag::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
