use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum NetworkGame {
    Offline = 0,
    Online = 1,
}

#[non_exhaustive]
pub struct InvalidNetworkGame(());

impl InvalidNetworkGame {
    fn new() -> Self {
        InvalidNetworkGame(())
    }
}

impl TryFrom<u8> for NetworkGame {
    type Error = InvalidNetworkGame;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(NetworkGame::Offline),
            1 => Ok(NetworkGame::Online),
            _ => Err(InvalidNetworkGame::new()),
        }
    }
}

impl NetworkGame {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, NetworkGame::try_from)(input)
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
        let result = NetworkGame::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], NetworkGame::Offline)));

        let packet = 1u8.to_le_bytes();
        let result = NetworkGame::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], NetworkGame::Online)));

        let packet = 2u8.to_le_bytes();
        let result = NetworkGame::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
