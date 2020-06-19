use nom::bytes::complete::take;
use nom::combinator::{flat_map, map, map_res};
use nom::error::make_error;
use nom::number::complete::{le_f32, le_u8};
use nom::sequence::tuple;
use nom::Err;

use crate::ParseResult;

#[derive(Debug, Copy, Clone)]
pub enum EventDataDetails {
    SessionStarted,
    SessionEnded,
    FastestLap { vehicle_index: u8, lap_time: f32 },
    Retirement { vehicle_index: u8 },
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits { vehicle_index: u8 },
    ChequeredFlag,
    RaceWinner { vehicle_index: u8 },
}

impl EventDataDetails {
    fn parse(event_string_code: &str) -> impl Fn(&[u8]) -> ParseResult<Self> + '_ {
        move |input| match event_string_code {
            "SSTA" => Ok((input, EventDataDetails::SessionStarted)),
            "SEND" => Ok((input, EventDataDetails::SessionEnded)),
            "FTLP" => map(tuple((le_u8, le_f32)), |(vehicle_index, lap_time)| {
                EventDataDetails::FastestLap {
                    vehicle_index,
                    lap_time,
                }
            })(input),
            "RTMT" => map(le_u8, |vehicle_index| EventDataDetails::Retirement {
                vehicle_index,
            })(input),
            "DRSE" => Ok((input, EventDataDetails::DRSEnabled)),
            "DRSD" => Ok((input, EventDataDetails::DRSDisabled)),
            "TMPT" => map(le_u8, |vehicle_index| EventDataDetails::TeamMateInPits {
                vehicle_index,
            })(input),
            "CHQF" => Ok((input, EventDataDetails::ChequeredFlag)),
            "RCWN" => map(le_u8, |vehicle_index| EventDataDetails::RaceWinner {
                vehicle_index,
            })(input),
            _ => Err(Err::Error(make_error(
                input,
                nom::error::ErrorKind::TagBits,
            ))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EventData<'a> {
    pub event_string_code: &'a str,
    pub event_details: EventDataDetails,
}

impl<'a> EventData<'a> {
    pub fn parse(input: &[u8]) -> ParseResult<EventData> {
        flat_map(
            map_res(take(4usize), std::str::from_utf8),
            |event_string_code| {
                map(
                    EventDataDetails::parse(event_string_code),
                    move |event_details| EventData {
                        event_string_code,
                        event_details,
                    },
                )
            },
        )(input)
    }
}
