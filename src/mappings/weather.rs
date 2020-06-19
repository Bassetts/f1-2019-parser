use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Weather {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

#[non_exhaustive]
pub struct InvalidWeather(());

impl InvalidWeather {
    fn new() -> Self {
        InvalidWeather(())
    }
}

impl TryFrom<u8> for Weather {
    type Error = InvalidWeather;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Weather::Clear),
            1 => Ok(Weather::LightCloud),
            2 => Ok(Weather::Overcast),
            3 => Ok(Weather::LightRain),
            4 => Ok(Weather::HeavyRain),
            5 => Ok(Weather::Storm),
            _ => Err(InvalidWeather::new()),
        }
    }
}

impl Weather {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, Weather::try_from)(input)
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
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::Clear)));

        let packet = 1u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::LightCloud)));

        let packet = 2u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::Overcast)));

        let packet = 3u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::LightRain)));

        let packet = 4u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::HeavyRain)));

        let packet = 5u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Weather::Storm)));

        let packet = 6u8.to_le_bytes();
        let result = Weather::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
