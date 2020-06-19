use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Nationality {
    American = 1,
    Argentinean = 2,
    Australian = 3,
    Austrian = 4,
    Azerbaijani = 5,
    Bahraini = 6,
    Belgian = 7,
    Bolivian = 8,
    Brazilian = 9,
    British = 10,
    Bulgarian = 11,
    Cameroonian = 12,
    Canadian = 13,
    Chilean = 14,
    Chinese = 15,
    Colombian = 16,
    CostaRican = 17,
    Croatian = 18,
    Cypriot = 19,
    Czech = 20,
    Danish = 21,
    Dutch = 22,
    Ecuadorian = 23,
    English = 24,
    Emirian = 25,
    Estonian = 26,
    Finnish = 27,
    French = 28,
    German = 29,
    Ghanaian = 30,
    Greek = 31,
    Guatemalan = 32,
    Honduran = 33,
    HongKonger = 34,
    Hungarian = 35,
    Icelander = 36,
    Indian = 37,
    Indonesian = 38,
    Irish = 39,
    Israeli = 40,
    Italian = 41,
    Jamaican = 42,
    Japanese = 43,
    Jordanian = 44,
    Kuwaiti = 45,
    Latvian = 46,
    Lebanese = 47,
    Lithuanian = 48,
    Luxembourger = 49,
    Malaysian = 50,
    Maltese = 51,
    Mexican = 52,
    Monegasque = 53,
    NewZealander = 54,
    Nicaraguan = 55,
    NorthKorean = 56,
    NorthernIrish = 57,
    Norwegian = 58,
    Omani = 59,
    Pakistani = 60,
    Panamanian = 61,
    Paraguayan = 62,
    Peruvian = 63,
    Polish = 64,
    Portuguese = 65,
    Qatari = 66,
    Romanian = 67,
    Russian = 68,
    Salvadoran = 69,
    Saudi = 70,
    Scottish = 71,
    Serbian = 72,
    Singaporean = 73,
    Slovakian = 74,
    Slovenian = 75,
    SouthKorean = 76,
    SouthAfrican = 77,
    Spanish = 78,
    Swedish = 79,
    Swiss = 80,
    Thai = 81,
    Turkish = 82,
    Uruguayan = 83,
    Ukrainian = 84,
    Venezuelan = 85,
    Welsh = 86,
}

#[non_exhaustive]
pub struct InvalidNationality(());

impl InvalidNationality {
    fn new() -> Self {
        InvalidNationality(())
    }
}

impl TryFrom<u8> for Nationality {
    type Error = InvalidNationality;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            1 => Ok(Nationality::American),
            2 => Ok(Nationality::Argentinean),
            3 => Ok(Nationality::Australian),
            4 => Ok(Nationality::Austrian),
            5 => Ok(Nationality::Azerbaijani),
            6 => Ok(Nationality::Bahraini),
            7 => Ok(Nationality::Belgian),
            8 => Ok(Nationality::Bolivian),
            9 => Ok(Nationality::Brazilian),
            10 => Ok(Nationality::British),
            11 => Ok(Nationality::Bulgarian),
            12 => Ok(Nationality::Cameroonian),
            13 => Ok(Nationality::Canadian),
            14 => Ok(Nationality::Chilean),
            15 => Ok(Nationality::Chinese),
            16 => Ok(Nationality::Colombian),
            17 => Ok(Nationality::CostaRican),
            18 => Ok(Nationality::Croatian),
            19 => Ok(Nationality::Cypriot),
            20 => Ok(Nationality::Czech),
            21 => Ok(Nationality::Danish),
            22 => Ok(Nationality::Dutch),
            23 => Ok(Nationality::Ecuadorian),
            24 => Ok(Nationality::English),
            25 => Ok(Nationality::Emirian),
            26 => Ok(Nationality::Estonian),
            27 => Ok(Nationality::Finnish),
            28 => Ok(Nationality::French),
            29 => Ok(Nationality::German),
            30 => Ok(Nationality::Ghanaian),
            31 => Ok(Nationality::Greek),
            32 => Ok(Nationality::Guatemalan),
            33 => Ok(Nationality::Honduran),
            34 => Ok(Nationality::HongKonger),
            35 => Ok(Nationality::Hungarian),
            36 => Ok(Nationality::Icelander),
            37 => Ok(Nationality::Indian),
            38 => Ok(Nationality::Indonesian),
            39 => Ok(Nationality::Irish),
            40 => Ok(Nationality::Israeli),
            41 => Ok(Nationality::Italian),
            42 => Ok(Nationality::Jamaican),
            43 => Ok(Nationality::Japanese),
            44 => Ok(Nationality::Jordanian),
            45 => Ok(Nationality::Kuwaiti),
            46 => Ok(Nationality::Latvian),
            47 => Ok(Nationality::Lebanese),
            48 => Ok(Nationality::Lithuanian),
            49 => Ok(Nationality::Luxembourger),
            50 => Ok(Nationality::Malaysian),
            51 => Ok(Nationality::Maltese),
            52 => Ok(Nationality::Mexican),
            53 => Ok(Nationality::Monegasque),
            54 => Ok(Nationality::NewZealander),
            55 => Ok(Nationality::Nicaraguan),
            56 => Ok(Nationality::NorthKorean),
            57 => Ok(Nationality::NorthernIrish),
            58 => Ok(Nationality::Norwegian),
            59 => Ok(Nationality::Omani),
            60 => Ok(Nationality::Pakistani),
            61 => Ok(Nationality::Panamanian),
            62 => Ok(Nationality::Paraguayan),
            63 => Ok(Nationality::Peruvian),
            64 => Ok(Nationality::Polish),
            65 => Ok(Nationality::Portuguese),
            66 => Ok(Nationality::Qatari),
            67 => Ok(Nationality::Romanian),
            68 => Ok(Nationality::Russian),
            69 => Ok(Nationality::Salvadoran),
            70 => Ok(Nationality::Saudi),
            71 => Ok(Nationality::Scottish),
            72 => Ok(Nationality::Serbian),
            73 => Ok(Nationality::Singaporean),
            74 => Ok(Nationality::Slovakian),
            75 => Ok(Nationality::Slovenian),
            76 => Ok(Nationality::SouthKorean),
            77 => Ok(Nationality::SouthAfrican),
            78 => Ok(Nationality::Spanish),
            79 => Ok(Nationality::Swedish),
            80 => Ok(Nationality::Swiss),
            81 => Ok(Nationality::Thai),
            82 => Ok(Nationality::Turkish),
            83 => Ok(Nationality::Uruguayan),
            84 => Ok(Nationality::Ukrainian),
            85 => Ok(Nationality::Venezuelan),
            86 => Ok(Nationality::Welsh),
            _ => Err(InvalidNationality::new()),
        }
    }
}

impl Nationality {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, Nationality::try_from)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_parse() {
        let packet = 1u8.to_le_bytes();
        let result = Nationality::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Nationality::American)));

        let packet = 43u8.to_le_bytes();
        let result = Nationality::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Nationality::Japanese)));

        let packet = 86u8.to_le_bytes();
        let result = Nationality::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], Nationality::Welsh)));

        let packet = 0u8.to_le_bytes();
        let result = Nationality::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));

        let packet = 87u8.to_le_bytes();
        let result = Nationality::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
