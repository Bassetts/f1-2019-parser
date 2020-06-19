use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_i8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum FiaFlag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

#[non_exhaustive]
pub struct InvalidFiaFlag(());

impl InvalidFiaFlag {
    fn new() -> Self {
        InvalidFiaFlag(())
    }
}

impl TryFrom<i8> for FiaFlag {
    type Error = InvalidFiaFlag;

    fn try_from(item: i8) -> Result<Self, Self::Error> {
        match item {
            -1 => Ok(FiaFlag::Unknown),
            0 => Ok(FiaFlag::None),
            1 => Ok(FiaFlag::Green),
            2 => Ok(FiaFlag::Blue),
            3 => Ok(FiaFlag::Yellow),
            4 => Ok(FiaFlag::Red),
            _ => Err(InvalidFiaFlag::new()),
        }
    }
}

impl FiaFlag {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_i8, FiaFlag::try_from)(input)
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
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::Unknown)));

        let packet = 0i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::None)));

        let packet = 1i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::Green)));

        let packet = 2i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::Blue)));

        let packet = 3i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::Yellow)));

        let packet = 4i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FiaFlag::Red)));

        let packet = (-2i8).to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));

        let packet = 5i8.to_le_bytes();
        let result = FiaFlag::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
