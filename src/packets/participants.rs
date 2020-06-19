use nom::bytes::complete::{take, take_until};
use nom::combinator::{flat_map, map, map_res};
use nom::multi::many1;
use nom::number::complete::le_u8;
use nom::sequence::tuple;

use crate::mappings::{DriverId, Nationality, TeamId, VehicleController};
use crate::ParseResult;

const MAX_NAME_LENGTH: usize = 48;

#[derive(Debug, Copy, Clone)]
pub struct ParticipantData<'a> {
    pub ai_controlled: VehicleController,
    pub driver_id: DriverId,
    pub team_id: TeamId,
    pub race_number: u8,
    pub nationality: Nationality,
    pub name: &'a str,
    pub your_telemetry: u8,
}

impl<'a> ParticipantData<'a> {
    fn parse(input: &[u8]) -> ParseResult<ParticipantData> {
        map(
            tuple((
                VehicleController::parse,
                DriverId::parse,
                TeamId::parse,
                le_u8,
                Nationality::parse,
                flat_map(map_res(take_until("\0"), std::str::from_utf8), |name| {
                    map(take(MAX_NAME_LENGTH - name.len()), move |_| name)
                }),
                le_u8,
            )),
            |(
                ai_controlled,
                driver_id,
                team_id,
                race_number,
                nationality,
                name,
                your_telemetry,
            )| {
                ParticipantData {
                    ai_controlled,
                    driver_id,
                    team_id,
                    race_number,
                    nationality,
                    name,
                    your_telemetry,
                }
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct ParticipantsData<'a> {
    pub number_active_cars: u8,
    pub participants: Vec<ParticipantData<'a>>,
}

impl<'a> ParticipantsData<'a> {
    pub fn parse(input: &[u8]) -> ParseResult<ParticipantsData> {
        map(
            tuple((le_u8, many1(ParticipantData::parse))),
            |(number_active_cars, participants)| ParticipantsData {
                number_active_cars,
                participants,
            },
        )(input)
    }
}
