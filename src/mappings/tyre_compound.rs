use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TyreCompound {
    F1Modern(F1Modern),
    F1Classic(F1Classic),
    F2(F2),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum F1Modern {
    Intermediate = 7,
    Wet = 8,
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum F1Classic {
    Dry = 9,
    Wet = 10,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum F2 {
    SuperSoft = 11,
    Soft = 12,
    Medium = 13,
    Hard = 14,
    Wet = 15,
}

#[non_exhaustive]
pub struct InvalidTyreCompound(());

impl InvalidTyreCompound {
    fn new() -> Self {
        InvalidTyreCompound(())
    }
}

impl TryFrom<u8> for TyreCompound {
    type Error = InvalidTyreCompound;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            7 => Ok(TyreCompound::F1Modern(F1Modern::Intermediate)),
            8 => Ok(TyreCompound::F1Modern(F1Modern::Wet)),
            16 => Ok(TyreCompound::F1Modern(F1Modern::C5)),
            17 => Ok(TyreCompound::F1Modern(F1Modern::C4)),
            18 => Ok(TyreCompound::F1Modern(F1Modern::C3)),
            19 => Ok(TyreCompound::F1Modern(F1Modern::C2)),
            20 => Ok(TyreCompound::F1Modern(F1Modern::C1)),
            9 => Ok(TyreCompound::F1Classic(F1Classic::Dry)),
            10 => Ok(TyreCompound::F1Classic(F1Classic::Wet)),
            11 => Ok(TyreCompound::F2(F2::SuperSoft)),
            12 => Ok(TyreCompound::F2(F2::Soft)),
            13 => Ok(TyreCompound::F2(F2::Medium)),
            14 => Ok(TyreCompound::F2(F2::Hard)),
            15 => Ok(TyreCompound::F2(F2::Wet)),
            _ => Err(InvalidTyreCompound::new()),
        }
    }
}

impl TyreCompound {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |tyre_compound: u8| {
            TyreCompound::try_from(tyre_compound)
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
        let packet = 7u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], TyreCompound::F1Modern(F1Modern::Intermediate)))
        );

        let packet = 8u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::Wet))));

        let packet = 16u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::C5))));

        let packet = 17u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::C4))));

        let packet = 18u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::C3))));

        let packet = 19u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::C2))));

        let packet = 20u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F1Modern(F1Modern::C1))));

        let packet = 9u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], TyreCompound::F1Classic(F1Classic::Dry)))
        );

        let packet = 10u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], TyreCompound::F1Classic(F1Classic::Wet)))
        );

        let packet = 11u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F2(F2::SuperSoft))));

        let packet = 12u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F2(F2::Soft))));

        let packet = 13u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F2(F2::Medium))));

        let packet = 14u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F2(F2::Hard))));

        let packet = 15u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TyreCompound::F2(F2::Wet))));

        let packet = 0u8.to_le_bytes();
        let result = TyreCompound::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
