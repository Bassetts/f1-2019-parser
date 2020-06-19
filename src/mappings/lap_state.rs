use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum LapState {
    Valid = 0,
    Invalid = 1,
}

#[non_exhaustive]
pub struct InvalidState(());

impl InvalidState {
    fn new() -> Self {
        InvalidState(())
    }
}

impl TryFrom<u8> for LapState {
    type Error = InvalidState;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(LapState::Valid),
            1 => Ok(LapState::Invalid),
            _ => Err(InvalidState::new()),
        }
    }
}

impl LapState {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, LapState::try_from)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_parse() {
        let packet = 0u8.to_le_bytes();
        let result = LapState::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], LapState::Valid)));

        let packet = 1u8.to_le_bytes();
        let result = LapState::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], LapState::Invalid)));

        let packet = 2u8.to_le_bytes();
        let result = LapState::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
