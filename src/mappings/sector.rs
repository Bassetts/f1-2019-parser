use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Sector {
    Sector1 = 0,
    Sector2 = 1,
    Sector3 = 2,
}

#[non_exhaustive]
pub struct InvalidSector(());

impl InvalidSector {
    fn new() -> Self {
        InvalidSector(())
    }
}

impl TryFrom<u8> for Sector {
    type Error = InvalidSector;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Sector::Sector1),
            1 => Ok(Sector::Sector2),
            2 => Ok(Sector::Sector3),
            _ => Err(InvalidSector::new()),
        }
    }
}

impl Sector {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, Sector::try_from)(input)
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
        let result = Sector::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Sector::Sector1)));

        let packet = 1u8.to_le_bytes();
        let result = Sector::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Sector::Sector2)));

        let packet = 2u8.to_le_bytes();
        let result = Sector::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Sector::Sector3)));

        let packet = 3u8.to_le_bytes();
        let result = Sector::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
