use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_i8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum TrackId {
    Unknown = -1,
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    Sakhir = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    Baku = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
}

#[non_exhaustive]
pub struct InvalidTrackId(());

impl InvalidTrackId {
    fn new() -> Self {
        InvalidTrackId(())
    }
}

impl TryFrom<i8> for TrackId {
    type Error = InvalidTrackId;

    fn try_from(item: i8) -> Result<Self, Self::Error> {
        match item {
            -1 => Ok(TrackId::Unknown),
            0 => Ok(TrackId::Melbourne),
            1 => Ok(TrackId::PaulRicard),
            2 => Ok(TrackId::Shanghai),
            3 => Ok(TrackId::Sakhir),
            4 => Ok(TrackId::Catalunya),
            5 => Ok(TrackId::Monaco),
            6 => Ok(TrackId::Montreal),
            7 => Ok(TrackId::Silverstone),
            8 => Ok(TrackId::Hockenheim),
            9 => Ok(TrackId::Hungaroring),
            10 => Ok(TrackId::Spa),
            11 => Ok(TrackId::Monza),
            12 => Ok(TrackId::Singapore),
            13 => Ok(TrackId::Suzuka),
            14 => Ok(TrackId::AbuDhabi),
            15 => Ok(TrackId::Texas),
            16 => Ok(TrackId::Brazil),
            17 => Ok(TrackId::Austria),
            18 => Ok(TrackId::Sochi),
            19 => Ok(TrackId::Mexico),
            20 => Ok(TrackId::Baku),
            21 => Ok(TrackId::SakhirShort),
            22 => Ok(TrackId::SilverstoneShort),
            23 => Ok(TrackId::TexasShort),
            24 => Ok(TrackId::SuzukaShort),
            _ => Err(InvalidTrackId::new()),
        }
    }
}

impl TrackId {
    pub(crate) fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_i8, TrackId::try_from)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_parse() {
        let packet = (-1i8).to_le_bytes();
        let result = TrackId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TrackId::Unknown)));

        let packet = 0i8.to_le_bytes();
        let result = TrackId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TrackId::Melbourne)));

        let packet = 12i8.to_le_bytes();
        let result = TrackId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TrackId::Singapore)));

        let packet = 24i8.to_le_bytes();
        let result = TrackId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], TrackId::SuzukaShort)));

        let packet = 25i8.to_le_bytes();
        let result = TrackId::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
