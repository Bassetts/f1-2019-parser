use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

#[non_exhaustive]
pub struct InvalidDriverStatus(());

impl InvalidDriverStatus {
    fn new() -> Self {
        InvalidDriverStatus(())
    }
}

impl TryFrom<u8> for DriverStatus {
    type Error = InvalidDriverStatus;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(DriverStatus::InGarage),
            1 => Ok(DriverStatus::FlyingLap),
            2 => Ok(DriverStatus::InLap),
            3 => Ok(DriverStatus::OutLap),
            4 => Ok(DriverStatus::OnTrack),
            _ => Err(InvalidDriverStatus::new()),
        }
    }
}

impl DriverStatus {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, DriverStatus::try_from)(input)
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
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverStatus::InGarage)));

        let packet = 1u8.to_le_bytes();
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverStatus::FlyingLap)));

        let packet = 2u8.to_le_bytes();
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverStatus::InLap)));

        let packet = 3u8.to_le_bytes();
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverStatus::OutLap)));

        let packet = 4u8.to_le_bytes();
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverStatus::OnTrack)));

        let packet = 5u8.to_le_bytes();
        let result = DriverStatus::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
