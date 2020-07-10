use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum DriverId {
    Known(KnownDriverId),
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum KnownDriverId {
    CarlosSainz = 0,
    DaniilKvyat = 1,
    DanielRicciardo = 2,
    KimiRaikkonen = 6,
    LewisHamilton = 7,
    MaxVerstappen = 9,
    NicoHulkenburg = 10,
    KevinMagnussen = 11,
    RomainGrosjean = 12,
    SebastianVettel = 13,
    SergioPerez = 14,
    ValtteriBottas = 15,
    EstebanOcon = 17,
    LanceStroll = 19,
    ArronBarnes = 20,
    MartinGiles = 21,
    AlexMurray = 22,
    LucasRoth = 23,
    IgorCorreia = 24,
    SophieLevasseur = 25,
    JonasSchiffer = 26,
    AlainForest = 27,
    JayLetourneau = 28,
    EstoSaari = 29,
    YasarAtiyeh = 30,
    CallistoCalabresi = 31,
    NaotaIzum = 32,
    HowardClarke = 33,
    WilhelmKaufmann = 34,
    MarieLaursen = 35,
    FlavioNieves = 36,
    PeterBelousov = 37,
    KlimekMichalski = 38,
    SantiagoMoreno = 39,
    BenjaminCoppens = 40,
    NoahVisser = 41,
    GertWaldmuller = 42,
    JulianQuesada = 43,
    DanielJones = 44,
    ArtemMarkelov = 45,
    TadasukeMakino = 46,
    SeanGelael = 47,
    NyckDeVries = 48,
    JackAitken = 49,
    GeorgeRussell = 50,
    MaximilianGunther = 51,
    NireiFukuzumi = 52,
    LucaGhiotto = 53,
    LandoNorris = 54,
    SergioSetteCamara = 55,
    LouisDeletraz = 56,
    AntonioFuoco = 57,
    CharlesLeclerc = 58,
    PierreGasly = 59,
    AlexanderAlbon = 62,
    NicholasLatifi = 63,
    DorianBoccolacci = 64,
    NikoKari = 65,
    RobertoMerhi = 66,
    ArjunMaini = 67,
    AlessioLorandi = 68,
    RubenMeijer = 69,
    RashidNair = 70,
    JackTremblay = 71,
    AntonioGiovinazzi = 74,
    RobertKubica = 75,
    NobuharuMatsushita = 78,
    NikitaMazepin = 79,
    GuanyaZhou = 80,
    MickSchumacher = 81,
    CallumIlott = 82,
    JuanManuelCorrea = 83,
    JordanKing = 84,
    MahaveerRaghunathan = 85,
    TatianaCalderon = 86,
    AnthoineHubert = 87,
    GuilianoAlesi = 88,
    RalphBoschung = 89,
}

#[non_exhaustive]
pub struct InvalidDriverId(());

impl InvalidDriverId {
    fn new() -> Self {
        InvalidDriverId(())
    }
}

impl TryFrom<u8> for DriverId {
    type Error = InvalidDriverId;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(DriverId::Known(KnownDriverId::CarlosSainz)),
            1 => Ok(DriverId::Known(KnownDriverId::DaniilKvyat)),
            2 => Ok(DriverId::Known(KnownDriverId::DanielRicciardo)),
            6 => Ok(DriverId::Known(KnownDriverId::KimiRaikkonen)),
            7 => Ok(DriverId::Known(KnownDriverId::LewisHamilton)),
            9 => Ok(DriverId::Known(KnownDriverId::MaxVerstappen)),
            10 => Ok(DriverId::Known(KnownDriverId::NicoHulkenburg)),
            11 => Ok(DriverId::Known(KnownDriverId::KevinMagnussen)),
            12 => Ok(DriverId::Known(KnownDriverId::RomainGrosjean)),
            13 => Ok(DriverId::Known(KnownDriverId::SebastianVettel)),
            14 => Ok(DriverId::Known(KnownDriverId::SergioPerez)),
            15 => Ok(DriverId::Known(KnownDriverId::ValtteriBottas)),
            17 => Ok(DriverId::Known(KnownDriverId::EstebanOcon)),
            19 => Ok(DriverId::Known(KnownDriverId::LanceStroll)),
            20 => Ok(DriverId::Known(KnownDriverId::ArronBarnes)),
            21 => Ok(DriverId::Known(KnownDriverId::MartinGiles)),
            22 => Ok(DriverId::Known(KnownDriverId::AlexMurray)),
            23 => Ok(DriverId::Known(KnownDriverId::LucasRoth)),
            24 => Ok(DriverId::Known(KnownDriverId::IgorCorreia)),
            25 => Ok(DriverId::Known(KnownDriverId::SophieLevasseur)),
            26 => Ok(DriverId::Known(KnownDriverId::JonasSchiffer)),
            27 => Ok(DriverId::Known(KnownDriverId::AlainForest)),
            28 => Ok(DriverId::Known(KnownDriverId::JayLetourneau)),
            29 => Ok(DriverId::Known(KnownDriverId::EstoSaari)),
            30 => Ok(DriverId::Known(KnownDriverId::YasarAtiyeh)),
            31 => Ok(DriverId::Known(KnownDriverId::CallistoCalabresi)),
            32 => Ok(DriverId::Known(KnownDriverId::NaotaIzum)),
            33 => Ok(DriverId::Known(KnownDriverId::HowardClarke)),
            34 => Ok(DriverId::Known(KnownDriverId::WilhelmKaufmann)),
            35 => Ok(DriverId::Known(KnownDriverId::MarieLaursen)),
            36 => Ok(DriverId::Known(KnownDriverId::FlavioNieves)),
            37 => Ok(DriverId::Known(KnownDriverId::PeterBelousov)),
            38 => Ok(DriverId::Known(KnownDriverId::KlimekMichalski)),
            39 => Ok(DriverId::Known(KnownDriverId::SantiagoMoreno)),
            40 => Ok(DriverId::Known(KnownDriverId::BenjaminCoppens)),
            41 => Ok(DriverId::Known(KnownDriverId::NoahVisser)),
            42 => Ok(DriverId::Known(KnownDriverId::GertWaldmuller)),
            43 => Ok(DriverId::Known(KnownDriverId::JulianQuesada)),
            44 => Ok(DriverId::Known(KnownDriverId::DanielJones)),
            45 => Ok(DriverId::Known(KnownDriverId::ArtemMarkelov)),
            46 => Ok(DriverId::Known(KnownDriverId::TadasukeMakino)),
            47 => Ok(DriverId::Known(KnownDriverId::SeanGelael)),
            48 => Ok(DriverId::Known(KnownDriverId::NyckDeVries)),
            49 => Ok(DriverId::Known(KnownDriverId::JackAitken)),
            50 => Ok(DriverId::Known(KnownDriverId::GeorgeRussell)),
            51 => Ok(DriverId::Known(KnownDriverId::MaximilianGunther)),
            52 => Ok(DriverId::Known(KnownDriverId::NireiFukuzumi)),
            53 => Ok(DriverId::Known(KnownDriverId::LucaGhiotto)),
            54 => Ok(DriverId::Known(KnownDriverId::LandoNorris)),
            55 => Ok(DriverId::Known(KnownDriverId::SergioSetteCamara)),
            56 => Ok(DriverId::Known(KnownDriverId::LouisDeletraz)),
            57 => Ok(DriverId::Known(KnownDriverId::AntonioFuoco)),
            58 => Ok(DriverId::Known(KnownDriverId::CharlesLeclerc)),
            59 => Ok(DriverId::Known(KnownDriverId::PierreGasly)),
            62 => Ok(DriverId::Known(KnownDriverId::AlexanderAlbon)),
            63 => Ok(DriverId::Known(KnownDriverId::NicholasLatifi)),
            64 => Ok(DriverId::Known(KnownDriverId::DorianBoccolacci)),
            65 => Ok(DriverId::Known(KnownDriverId::NikoKari)),
            66 => Ok(DriverId::Known(KnownDriverId::RobertoMerhi)),
            67 => Ok(DriverId::Known(KnownDriverId::ArjunMaini)),
            68 => Ok(DriverId::Known(KnownDriverId::AlessioLorandi)),
            69 => Ok(DriverId::Known(KnownDriverId::RubenMeijer)),
            70 => Ok(DriverId::Known(KnownDriverId::RashidNair)),
            71 => Ok(DriverId::Known(KnownDriverId::JackTremblay)),
            74 => Ok(DriverId::Known(KnownDriverId::AntonioGiovinazzi)),
            75 => Ok(DriverId::Known(KnownDriverId::RobertKubica)),
            78 => Ok(DriverId::Known(KnownDriverId::NobuharuMatsushita)),
            79 => Ok(DriverId::Known(KnownDriverId::NikitaMazepin)),
            80 => Ok(DriverId::Known(KnownDriverId::GuanyaZhou)),
            81 => Ok(DriverId::Known(KnownDriverId::MickSchumacher)),
            82 => Ok(DriverId::Known(KnownDriverId::CallumIlott)),
            83 => Ok(DriverId::Known(KnownDriverId::JuanManuelCorrea)),
            84 => Ok(DriverId::Known(KnownDriverId::JordanKing)),
            85 => Ok(DriverId::Known(KnownDriverId::MahaveerRaghunathan)),
            86 => Ok(DriverId::Known(KnownDriverId::TatianaCalderon)),
            87 => Ok(DriverId::Known(KnownDriverId::AnthoineHubert)),
            88 => Ok(DriverId::Known(KnownDriverId::GuilianoAlesi)),
            89 => Ok(DriverId::Known(KnownDriverId::RalphBoschung)),
            driver_id if driver_id >= 100 => Ok(DriverId::Unknown(driver_id)),
            _ => Err(InvalidDriverId::new()),
        }
    }
}

impl DriverId {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, DriverId::try_from)(input)
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
        let result = DriverId::parse(&packet[..]);
        assert_eq!(
            result,
            Ok((&[][..], DriverId::Known(KnownDriverId::CarlosSainz)))
        );

        let packet = 16u8.to_le_bytes();
        let result = DriverId::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));

        let packet = 100u8.to_le_bytes();
        let result = DriverId::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], DriverId::Unknown(100))));
    }
}
