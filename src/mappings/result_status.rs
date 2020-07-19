use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    Disqualified = 4,
    NotClassified = 5,
    Retired = 6,
    Dnf = 7,
}

#[non_exhaustive]
pub struct InvalidResultStatus(());

impl InvalidResultStatus {
    fn new() -> Self {
        InvalidResultStatus(())
    }
}
impl TryFrom<u8> for ResultStatus {
    type Error = InvalidResultStatus;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(ResultStatus::Invalid),
            1 => Ok(ResultStatus::Inactive),
            2 => Ok(ResultStatus::Active),
            3 => Ok(ResultStatus::Finished),
            4 => Ok(ResultStatus::Disqualified),
            5 => Ok(ResultStatus::NotClassified),
            6 => Ok(ResultStatus::Retired),
            7 => Ok(ResultStatus::Dnf),
            _ => Err(InvalidResultStatus::new()),
        }
    }
}

impl ResultStatus {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, ResultStatus::try_from)(input)
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
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Invalid)));

        let packet = 1u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Inactive)));

        let packet = 2u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Active)));

        let packet = 3u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Finished)));

        let packet = 4u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Disqualified)));

        let packet = 5u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::NotClassified)));

        let packet = 6u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Retired)));

        let packet = 7u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ResultStatus::Dnf)));

        let packet = 8u8.to_le_bytes();
        let result = ResultStatus::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
