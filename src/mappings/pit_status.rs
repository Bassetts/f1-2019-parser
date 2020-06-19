use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

#[non_exhaustive]
pub struct InvalidPitStatus(());

impl InvalidPitStatus {
    fn new() -> Self {
        InvalidPitStatus(())
    }
}

impl TryFrom<u8> for PitStatus {
    type Error = InvalidPitStatus;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(PitStatus::None),
            1 => Ok(PitStatus::Pitting),
            2 => Ok(PitStatus::InPitArea),
            _ => Err(InvalidPitStatus::new()),
        }
    }
}

impl PitStatus {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, PitStatus::try_from)(input)
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
        let result = PitStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], PitStatus::None)));

        let packet = 1u8.to_le_bytes();
        let result = PitStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], PitStatus::Pitting)));

        let packet = 2u8.to_le_bytes();
        let result = PitStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], PitStatus::InPitArea)));

        let packet = 3u8.to_le_bytes();
        let result = PitStatus::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
