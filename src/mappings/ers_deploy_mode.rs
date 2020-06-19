use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ErsDeployMode {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Overtake = 4,
    Hotlap = 5,
}

#[non_exhaustive]
pub struct InvalidErsDeployMode(());

impl InvalidErsDeployMode {
    fn new() -> Self {
        InvalidErsDeployMode(())
    }
}

impl TryFrom<u8> for ErsDeployMode {
    type Error = InvalidErsDeployMode;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(ErsDeployMode::None),
            1 => Ok(ErsDeployMode::Low),
            2 => Ok(ErsDeployMode::Medium),
            3 => Ok(ErsDeployMode::High),
            4 => Ok(ErsDeployMode::Overtake),
            5 => Ok(ErsDeployMode::Hotlap),
            _ => Err(InvalidErsDeployMode::new()),
        }
    }
}

impl ErsDeployMode {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |ers_deploy_mode: u8| {
            ErsDeployMode::try_from(ers_deploy_mode)
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
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::None)));

        let packet = 1u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::Low)));

        let packet = 2u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::Medium)));

        let packet = 3u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::High)));

        let packet = 4u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::Overtake)));

        let packet = 5u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], ErsDeployMode::Hotlap)));

        let packet = 6u8.to_le_bytes();
        let result = ErsDeployMode::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
