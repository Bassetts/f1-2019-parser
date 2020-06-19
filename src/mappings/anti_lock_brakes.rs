use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum AntiLockBrakes {
    Off = 0,
    On = 1,
}

#[non_exhaustive]
pub struct InvalidAntiLockBrakes(());

impl InvalidAntiLockBrakes {
    fn new() -> Self {
        InvalidAntiLockBrakes(())
    }
}

impl TryFrom<u8> for AntiLockBrakes {
    type Error = InvalidAntiLockBrakes;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(AntiLockBrakes::Off),
            1 => Ok(AntiLockBrakes::On),
            _ => Err(InvalidAntiLockBrakes::new()),
        }
    }
}

impl AntiLockBrakes {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |anti_lock_brakes: u8| {
            AntiLockBrakes::try_from(anti_lock_brakes)
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
        let result = AntiLockBrakes::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], AntiLockBrakes::Off)));

        let packet = 1u8.to_le_bytes();
        let result = AntiLockBrakes::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], AntiLockBrakes::On)));

        let packet = 2u8.to_le_bytes();
        let result = AntiLockBrakes::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
