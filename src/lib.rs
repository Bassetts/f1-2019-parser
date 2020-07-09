use nom::combinator::map;
use nom::error::ErrorKind;
use nom::number::complete::{le_f32, le_u16, le_u8};
use nom::sequence::tuple;
use nom::IResult;

pub mod error;
pub mod mappings;
mod packets;

use packets::{
    header::{Header, PacketId},
    EventData, MotionData, PacketCarSetupData, PacketCarStatusData, PacketCarTelemetryData,
    PacketLapData, ParticipantsData, SessionData,
};

use error::ParseError;

pub const MAXIMUM_PACKET_SIZE: usize = 1347;

type ParseResult<'a, O, E = (&'a [u8], ErrorKind)> = IResult<&'a [u8], O, E>;

#[derive(Debug, Copy, Clone)]
pub struct WheelData<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

impl WheelData<f32> {
    fn parse_f32(input: &[u8]) -> ParseResult<WheelData<f32>> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32)),
            |(rear_left, rear_right, front_left, front_right)| WheelData {
                rear_left,
                rear_right,
                front_left,
                front_right,
            },
        )(input)
    }
}

impl WheelData<u16> {
    fn parse_u16(input: &[u8]) -> ParseResult<WheelData<u16>> {
        map(
            tuple((le_u16, le_u16, le_u16, le_u16)),
            |(rear_left, rear_right, front_left, front_right)| WheelData {
                rear_left,
                rear_right,
                front_left,
                front_right,
            },
        )(input)
    }
}

impl WheelData<u8> {
    fn parse_u8(input: &[u8]) -> ParseResult<WheelData<u8>> {
        map(
            tuple((le_u8, le_u8, le_u8, le_u8)),
            |(rear_left, rear_right, front_left, front_right)| WheelData {
                rear_left,
                rear_right,
                front_left,
                front_right,
            },
        )(input)
    }
}

#[derive(Debug)]
pub enum TelemetryData<'a> {
    Motion(MotionData),
    Session(SessionData),
    Lap(PacketLapData),
    Event(EventData<'a>),
    Participants(ParticipantsData<'a>),
    CarSetups(PacketCarSetupData),
    CarTelemetry(PacketCarTelemetryData),
    CarStatus(PacketCarStatusData),
}

#[derive(Debug)]
pub struct Telemetry<'a> {
    pub header: Header,
    pub data: TelemetryData<'a>,
}

impl<'a> Telemetry<'a> {
    fn parse(input: &[u8]) -> ParseResult<Telemetry> {
        let (input, header) = Header::parse(input)?;
        let (input, data) = match header.packet_id {
            PacketId::Motion => map(MotionData::parse, TelemetryData::Motion)(input)?,
            PacketId::Session => map(SessionData::parse, TelemetryData::Session)(input)?,
            PacketId::LapData => map(PacketLapData::parse, TelemetryData::Lap)(input)?,
            PacketId::Event => map(EventData::parse, TelemetryData::Event)(input)?,
            PacketId::Participants => {
                map(ParticipantsData::parse, TelemetryData::Participants)(input)?
            }
            PacketId::CarSetups => map(PacketCarSetupData::parse, TelemetryData::CarSetups)(input)?,
            PacketId::CarTelemetry => map(PacketCarTelemetryData::parse, {
                TelemetryData::CarTelemetry
            })(input)?,
            PacketId::CarStatus => {
                map(PacketCarStatusData::parse, TelemetryData::CarStatus)(input)?
            }
        };

        Ok((input, Telemetry { header, data }))
    }
}

pub fn parse_packet(input: &[u8]) -> Result<Telemetry, error::ParseError> {
    match Telemetry::parse(input) {
        Ok((_, result)) => Ok(result),
        Err(error) => Err(ParseError::from(error)),
    }
}
