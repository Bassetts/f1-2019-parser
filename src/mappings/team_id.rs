use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TeamId {
    Mercedes = 0,
    Ferrari = 1,
    RedBullRacing = 2,
    Williams = 3,
    RacingPoint = 4,
    Renault = 5,
    ToroRosso = 6,
    Haas = 7,
    McLaren = 8,
    AlfaRomeo = 9,
    McLaren1988 = 10,
    McLaren1991 = 11,
    Williams1992 = 12,
    Ferrari1995 = 13,
    Williams1996 = 14,
    McLaren1998 = 15,
    Ferrari2002 = 16,
    Ferrari2004 = 17,
    Renault2006 = 18,
    Ferrari2007 = 19,
    RedBull2010 = 21,
    Ferrari1976 = 22,
    ARTGrandPrix = 23,
    CamposVexatecRacing = 24,
    Carlin = 25,
    CharouzRacingSystem = 26,
    DAMS = 27,
    RussianTime = 28,
    MPMotorsport = 29,
    Pertamina = 30,
    McLaren1990 = 31,
    Trident = 32,
    BWTArden = 33,
    McLaren1976 = 34,
    Lotus1972 = 35,
    Ferrari1979 = 36,
    McLaren1982 = 37,
    Williams2003 = 38,
    Brawn2009 = 39,
    Lotus1978 = 40,
    ArtGP19 = 42,
    Campos19 = 43,
    Carlin19 = 44,
    SauberJuniorCharouz19 = 45,
    Dams19 = 46,
    UniVirtuosi19 = 47,
    MPMotorsport19 = 48,
    Prema19 = 49,
    Trident19 = 50,
    Arden19 = 51,
    Ferrari1990 = 63,
    McLaren2010 = 64,
    Ferrari2010 = 65,
}

#[non_exhaustive]
pub struct InvalidTeamId(());

impl InvalidTeamId {
    fn new() -> Self {
        InvalidTeamId(())
    }
}

impl TryFrom<u8> for TeamId {
    type Error = InvalidTeamId;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(TeamId::Mercedes),
            1 => Ok(TeamId::Ferrari),
            2 => Ok(TeamId::RedBullRacing),
            3 => Ok(TeamId::Williams),
            4 => Ok(TeamId::RacingPoint),
            5 => Ok(TeamId::Renault),
            6 => Ok(TeamId::ToroRosso),
            7 => Ok(TeamId::Haas),
            8 => Ok(TeamId::McLaren),
            9 => Ok(TeamId::AlfaRomeo),
            10 => Ok(TeamId::McLaren1988),
            11 => Ok(TeamId::McLaren1991),
            12 => Ok(TeamId::Williams1992),
            13 => Ok(TeamId::Ferrari1995),
            14 => Ok(TeamId::Williams1996),
            15 => Ok(TeamId::McLaren1998),
            16 => Ok(TeamId::Ferrari2002),
            17 => Ok(TeamId::Ferrari2004),
            18 => Ok(TeamId::Renault2006),
            19 => Ok(TeamId::Ferrari2007),
            21 => Ok(TeamId::RedBull2010),
            22 => Ok(TeamId::Ferrari1976),
            23 => Ok(TeamId::ARTGrandPrix),
            24 => Ok(TeamId::CamposVexatecRacing),
            25 => Ok(TeamId::Carlin),
            26 => Ok(TeamId::CharouzRacingSystem),
            27 => Ok(TeamId::DAMS),
            28 => Ok(TeamId::RussianTime),
            29 => Ok(TeamId::MPMotorsport),
            30 => Ok(TeamId::Pertamina),
            31 => Ok(TeamId::McLaren1990),
            32 => Ok(TeamId::Trident),
            33 => Ok(TeamId::BWTArden),
            34 => Ok(TeamId::McLaren1976),
            35 => Ok(TeamId::Lotus1972),
            36 => Ok(TeamId::Ferrari1979),
            37 => Ok(TeamId::McLaren1982),
            38 => Ok(TeamId::Williams2003),
            39 => Ok(TeamId::Brawn2009),
            40 => Ok(TeamId::Lotus1978),
            42 => Ok(TeamId::ArtGP19),
            43 => Ok(TeamId::Campos19),
            44 => Ok(TeamId::Carlin19),
            45 => Ok(TeamId::SauberJuniorCharouz19),
            46 => Ok(TeamId::Dams19),
            47 => Ok(TeamId::UniVirtuosi19),
            48 => Ok(TeamId::MPMotorsport19),
            49 => Ok(TeamId::Prema19),
            50 => Ok(TeamId::Trident19),
            51 => Ok(TeamId::Arden19),
            63 => Ok(TeamId::Ferrari1990),
            64 => Ok(TeamId::McLaren2010),
            65 => Ok(TeamId::Ferrari2010),
            _ => Err(InvalidTeamId::new()),
        }
    }
}

impl TeamId {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, TeamId::try_from)(input)
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
        let result = TeamId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TeamId::Mercedes)));

        let packet = 32u8.to_le_bytes();
        let result = TeamId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TeamId::Trident)));

        let packet = 65u8.to_le_bytes();
        let result = TeamId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TeamId::Ferrari2010)));

        let packet = 66u8.to_le_bytes();
        let result = TeamId::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
