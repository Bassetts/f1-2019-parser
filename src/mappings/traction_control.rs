use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TractionControl {
    Off = 0,
    Medium = 1,
    Full = 2,
}

#[non_exhaustive]
pub struct InvalidTractionControl(());

impl InvalidTractionControl {
    fn new() -> Self {
        InvalidTractionControl(())
    }
}

impl TryFrom<u8> for TractionControl {
    type Error = InvalidTractionControl;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(TractionControl::Off),
            1 => Ok(TractionControl::Medium),
            2 => Ok(TractionControl::Full),
            _ => Err(InvalidTractionControl::new()),
        }
    }
}

impl TractionControl {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |traction_control: u8| {
            TractionControl::try_from(traction_control)
        })(input)
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
        let result = TractionControl::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TractionControl::Off)));

        let packet = 1u8.to_le_bytes();
        let result = TractionControl::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TractionControl::Medium)));

        let packet = 2u8.to_le_bytes();
        let result = TractionControl::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TractionControl::Full)));

        let packet = 3u8.to_le_bytes();
        let result = TractionControl::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
