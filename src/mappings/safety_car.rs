use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum SafetyCarStatus {
    None = 0,
    Full = 1,
    Virtual = 2,
}

#[non_exhaustive]
pub struct InvalidSafetyCar(());

impl InvalidSafetyCar {
    fn new() -> Self {
        InvalidSafetyCar(())
    }
}

impl TryFrom<u8> for SafetyCarStatus {
    type Error = InvalidSafetyCar;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(SafetyCarStatus::None),
            1 => Ok(SafetyCarStatus::Full),
            2 => Ok(SafetyCarStatus::Virtual),
            _ => Err(InvalidSafetyCar::new()),
        }
    }
}

impl SafetyCarStatus {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, SafetyCarStatus::try_from)(input)
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
        let result = SafetyCarStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SafetyCarStatus::None)));

        let packet = 1u8.to_le_bytes();
        let result = SafetyCarStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SafetyCarStatus::Full)));

        let packet = 2u8.to_le_bytes();
        let result = SafetyCarStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SafetyCarStatus::Virtual)));

        let packet = 3u8.to_le_bytes();
        let result = SafetyCarStatus::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
