use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum VisualCompound {
    F1Modern(F1Modern),
    F1Classic(F1Classic),
    F2(F2),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum F1Modern {
    Intermediate = 7,
    Wet = 8,
    Soft = 16,
    Medium = 17,
    Hard = 18,
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
pub struct InvalidVisualCompound(());

impl InvalidVisualCompound {
    fn new() -> Self {
        InvalidVisualCompound(())
    }
}

impl TryFrom<u8> for VisualCompound {
    type Error = InvalidVisualCompound;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            7 => Ok(VisualCompound::F1Modern(F1Modern::Intermediate)),
            8 => Ok(VisualCompound::F1Modern(F1Modern::Wet)),
            16 => Ok(VisualCompound::F1Modern(F1Modern::Soft)),
            17 => Ok(VisualCompound::F1Modern(F1Modern::Medium)),
            18 => Ok(VisualCompound::F1Modern(F1Modern::Hard)),
            9 => Ok(VisualCompound::F1Classic(F1Classic::Dry)),
            10 => Ok(VisualCompound::F1Classic(F1Classic::Wet)),
            11 => Ok(VisualCompound::F2(F2::SuperSoft)),
            12 => Ok(VisualCompound::F2(F2::Soft)),
            13 => Ok(VisualCompound::F2(F2::Medium)),
            14 => Ok(VisualCompound::F2(F2::Hard)),
            15 => Ok(VisualCompound::F2(F2::Wet)),
            _ => Err(InvalidVisualCompound::new()),
        }
    }
}

impl VisualCompound {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |tyre_compound: u8| {
            VisualCompound::try_from(tyre_compound)
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
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Modern(F1Modern::Intermediate)))
        );

        let packet = 8u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Modern(F1Modern::Wet)))
        );

        let packet = 16u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Modern(F1Modern::Soft)))
        );

        let packet = 17u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Modern(F1Modern::Medium)))
        );

        let packet = 18u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Modern(F1Modern::Hard)))
        );

        let packet = 9u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Classic(F1Classic::Dry)))
        );

        let packet = 10u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], VisualCompound::F1Classic(F1Classic::Wet)))
        );

        let packet = 11u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VisualCompound::F2(F2::SuperSoft))));

        let packet = 12u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VisualCompound::F2(F2::Soft))));

        let packet = 13u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VisualCompound::F2(F2::Medium))));

        let packet = 14u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VisualCompound::F2(F2::Hard))));

        let packet = 15u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VisualCompound::F2(F2::Wet))));

        let packet = 0u8.to_le_bytes();
        let result = VisualCompound::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
