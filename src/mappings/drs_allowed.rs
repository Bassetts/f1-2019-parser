use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_i8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum DrsAllowed {
    Unknown = -1,
    NotAllowed = 0,
    Allowed = 1,
}

#[non_exhaustive]
pub struct InvalidDrsAllowed(());

impl InvalidDrsAllowed {
    fn new() -> Self {
        InvalidDrsAllowed(())
    }
}

impl TryFrom<i8> for DrsAllowed {
    type Error = InvalidDrsAllowed;

    fn try_from(item: i8) -> Result<Self, Self::Error> {
        match item {
            -1 => Ok(DrsAllowed::Unknown),
            0 => Ok(DrsAllowed::NotAllowed),
            1 => Ok(DrsAllowed::Allowed),
            _ => Err(InvalidDrsAllowed::new()),
        }
    }
}

impl DrsAllowed {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_i8, DrsAllowed::try_from)(input)
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
        let result = DrsAllowed::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DrsAllowed::Unknown)));

        let packet = 0i8.to_le_bytes();
        let result = DrsAllowed::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DrsAllowed::NotAllowed)));

        let packet = 1i8.to_le_bytes();
        let result = DrsAllowed::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DrsAllowed::Allowed)));

        let packet = (-2i8).to_le_bytes();
        let result = DrsAllowed::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));

        let packet = 2i8.to_le_bytes();
        let result = DrsAllowed::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
