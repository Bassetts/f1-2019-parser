use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Formula {
    Formula1Modern = 0,
    Formula1Classic = 1,
    Formula2 = 2,
    Formula1Generic = 3,
}

#[non_exhaustive]
pub struct InvalidFormula(());

impl InvalidFormula {
    fn new() -> Self {
        InvalidFormula(())
    }
}

impl TryFrom<u8> for Formula {
    type Error = InvalidFormula;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Formula::Formula1Modern),
            1 => Ok(Formula::Formula1Classic),
            2 => Ok(Formula::Formula2),
            3 => Ok(Formula::Formula1Generic),
            _ => Err(InvalidFormula::new()),
        }
    }
}

impl Formula {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, Formula::try_from)(input)
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
        let result = Formula::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Formula::Formula1Modern)));

        let packet = 1u8.to_le_bytes();
        let result = Formula::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Formula::Formula1Classic)));

        let packet = 2u8.to_le_bytes();
        let result = Formula::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Formula::Formula2)));

        let packet = 3u8.to_le_bytes();
        let result = Formula::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Formula::Formula1Generic)));

        let packet = 4u8.to_le_bytes();
        let result = Formula::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
