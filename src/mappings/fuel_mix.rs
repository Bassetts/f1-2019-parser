use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

#[non_exhaustive]
pub struct InvalidFuelMix(());

impl InvalidFuelMix {
    fn new() -> Self {
        InvalidFuelMix(())
    }
}

impl TryFrom<u8> for FuelMix {
    type Error = InvalidFuelMix;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(FuelMix::Lean),
            1 => Ok(FuelMix::Standard),
            2 => Ok(FuelMix::Rich),
            3 => Ok(FuelMix::Max),
            _ => Err(InvalidFuelMix::new()),
        }
    }
}

impl FuelMix {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, FuelMix::try_from)(input)
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
        let result = FuelMix::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FuelMix::Lean)));

        let packet = 1u8.to_le_bytes();
        let result = FuelMix::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FuelMix::Standard)));

        let packet = 2u8.to_le_bytes();
        let result = FuelMix::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FuelMix::Rich)));

        let packet = 3u8.to_le_bytes();
        let result = FuelMix::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], FuelMix::Max)));

        let packet = 4u8.to_le_bytes();
        let result = FuelMix::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
