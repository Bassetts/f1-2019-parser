use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{le_f32, le_f64, le_u8};
use nom::sequence::tuple;

use crate::mappings::ResultStatus;
use crate::ParseResult;

#[derive(Debug)]
pub struct FinalClassificationData {
  pub position: u8,
  pub number_laps: u8,
  pub grid_position: u8,
  pub points: u8,
  pub number_pit_stops: u8,
  pub result_status: ResultStatus,
  pub best_lap_time: f32,
  pub total_race_time: f64,
  pub penalties_time: u8,
  pub number_penalties: u8,
  pub number_tyre_stints: u8,
  pub tyre_stints_actual: Vec<u8>,
  pub tyre_stints_visual: Vec<u8>,
}

impl FinalClassificationData {
  fn parse(input: &[u8]) -> ParseResult<FinalClassificationData> {
    map(
      tuple((
        le_u8,
        le_u8,
        le_u8,
        le_u8,
        le_u8,
        ResultStatus::parse,
        le_f32,
        le_f64,
        le_u8,
        le_u8,
        le_u8,
        count(le_u8, 8),
        count(le_u8, 8),
      )),
      |(
        position,
        number_laps,
        grid_position,
        points,
        number_pit_stops,
        result_status,
        best_lap_time,
        total_race_time,
        penalties_time,
        number_penalties,
        number_tyre_stints,
        tyre_stints_actual,
        tyre_stints_visual,
      )| FinalClassificationData {
        position,
        number_laps,
        grid_position,
        points,
        number_pit_stops,
        result_status,
        best_lap_time,
        total_race_time,
        penalties_time,
        number_penalties,
        number_tyre_stints,
        tyre_stints_actual,
        tyre_stints_visual,
      },
    )(input)
  }
}

#[derive(Debug)]
pub struct FinalClassificationsData {
  pub number_cars: u8,
  pub classification_data: Vec<FinalClassificationData>,
}

impl FinalClassificationsData {
  pub fn parse(input: &[u8]) -> ParseResult<FinalClassificationsData> {
    let (input, number_cars) = le_u8(input)?;
    map(
      count(FinalClassificationData::parse, number_cars as usize),
      move |classification_data| FinalClassificationsData {
        number_cars,
        classification_data,
      },
    )(input)
  }
}
