use std::convert::TryFrom;
use std::convert::TryInto;

use nom::combinator::map;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8};
use nom::sequence::tuple;

use crate::ParseResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PacketId {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
    FinalClassification = 8,
    LobbyInfo = 9,
}

impl PacketId {
    pub fn parse(input: &[u8]) -> ParseResult<PacketId> {
        map(le_u8, |packet_id: u8| packet_id.try_into().unwrap())(input)
    }
}

impl TryFrom<u8> for PacketId {
    type Error = String;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(PacketId::Motion),
            1 => Ok(PacketId::Session),
            2 => Ok(PacketId::LapData),
            3 => Ok(PacketId::Event),
            4 => Ok(PacketId::Participants),
            5 => Ok(PacketId::CarSetups),
            6 => Ok(PacketId::CarTelemetry),
            7 => Ok(PacketId::CarStatus),
            8 => Ok(PacketId::FinalClassification),
            9 => Ok(PacketId::LobbyInfo),
            x => Err(format!("Invalid packet id: {}", x)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: PacketId,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

impl Header {
    pub fn parse(input: &[u8]) -> ParseResult<Header> {
        map(
            tuple((
                le_u16,
                le_u8,
                le_u8,
                le_u8,
                PacketId::parse,
                le_u64,
                le_f32,
                le_u32,
                le_u8,
                le_u8,
            )),
            |(
                packet_format,
                game_major_version,
                game_minor_version,
                packet_version,
                packet_id,
                session_uid,
                session_time,
                frame_identifier,
                player_car_index,
                secondary_player_car_index,
            )| Header {
                packet_format,
                game_major_version,
                game_minor_version,
                packet_version,
                packet_id,
                session_uid,
                session_time,
                frame_identifier,
                player_car_index,
                secondary_player_car_index,
            },
        )(input)
    }
}
