use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum SurfaceType {
    Tarmac = 0,
    RumbleStrip = 1,
    Concrete = 2,
    Rock = 3,
    Gravel = 4,
    Mud = 5,
    Sand = 6,
    Grass = 7,
    Water = 8,
    Cobblestone = 9,
    Metal = 10,
    Ridged = 11,
}

#[non_exhaustive]
pub struct InvalidSurfaceType(());

impl InvalidSurfaceType {
    fn new() -> Self {
        InvalidSurfaceType(())
    }
}

impl TryFrom<u8> for SurfaceType {
    type Error = InvalidSurfaceType;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(SurfaceType::Tarmac),
            1 => Ok(SurfaceType::RumbleStrip),
            2 => Ok(SurfaceType::Concrete),
            3 => Ok(SurfaceType::Rock),
            4 => Ok(SurfaceType::Gravel),
            5 => Ok(SurfaceType::Mud),
            6 => Ok(SurfaceType::Sand),
            7 => Ok(SurfaceType::Grass),
            8 => Ok(SurfaceType::Water),
            9 => Ok(SurfaceType::Cobblestone),
            10 => Ok(SurfaceType::Metal),
            11 => Ok(SurfaceType::Ridged),
            _ => Err(InvalidSurfaceType::new()),
        }
    }
}

impl SurfaceType {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |surface_type: u8| {
            SurfaceType::try_from(surface_type)
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
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Tarmac)));

        let packet = 1u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::RumbleStrip)));

        let packet = 2u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Concrete)));

        let packet = 3u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Rock)));

        let packet = 4u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Gravel)));

        let packet = 5u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Mud)));

        let packet = 6u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Sand)));

        let packet = 7u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Grass)));

        let packet = 8u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Water)));

        let packet = 9u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Cobblestone)));

        let packet = 10u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Metal)));

        let packet = 11u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SurfaceType::Ridged)));

        let packet = 12u8.to_le_bytes();
        let result = SurfaceType::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
