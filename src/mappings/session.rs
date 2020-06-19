use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum SessionType {
    Unknown = 0,
    Practice1 = 1,
    Practice2 = 2,
    Practice3 = 3,
    ShortPractice = 4,
    Qualifying1 = 5,
    Qualifying2 = 6,
    Qualifying3 = 7,
    ShortQualifying = 8,
    OneShotQualifying = 9,
    Race = 10,
    Race2 = 11,
    TimeTrial = 12,
}

#[non_exhaustive]
pub struct InvalidSessionType(());

impl InvalidSessionType {
    fn new() -> Self {
        InvalidSessionType(())
    }
}

impl TryFrom<u8> for SessionType {
    type Error = InvalidSessionType;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(SessionType::Unknown),
            1 => Ok(SessionType::Practice1),
            2 => Ok(SessionType::Practice2),
            3 => Ok(SessionType::Practice3),
            4 => Ok(SessionType::ShortPractice),
            5 => Ok(SessionType::Qualifying1),
            6 => Ok(SessionType::Qualifying2),
            7 => Ok(SessionType::Qualifying3),
            8 => Ok(SessionType::ShortQualifying),
            9 => Ok(SessionType::OneShotQualifying),
            10 => Ok(SessionType::Race),
            11 => Ok(SessionType::Race2),
            12 => Ok(SessionType::TimeTrial),
            _ => Err(InvalidSessionType::new()),
        }
    }
}

impl SessionType {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, SessionType::try_from)(input)
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
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Unknown)));

        let packet = 1u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Practice1)));

        let packet = 2u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Practice2)));

        let packet = 3u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Practice3)));

        let packet = 4u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::ShortPractice)));

        let packet = 5u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Qualifying1)));

        let packet = 6u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Qualifying2)));

        let packet = 7u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Qualifying3)));

        let packet = 8u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::ShortQualifying)));

        let packet = 9u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::OneShotQualifying)));

        let packet = 10u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Race)));

        let packet = 11u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::Race2)));

        let packet = 12u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], SessionType::TimeTrial)));

        let packet = 13u8.to_le_bytes();
        let result = SessionType::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
