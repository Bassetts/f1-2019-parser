use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u8};
use nom::sequence::tuple;

use crate::mappings::{DriverStatus, LapState, PitStatus, ResultStatus, Sector};
use crate::ParseResult;

#[derive(Debug, Copy, Clone)]
pub struct LapData {
    pub last_lap_time: f32,
    pub current_lap_time: f32,
    pub best_lap_time: f32,
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub sector: Sector,
    pub current_lap_invalid: LapState,
    pub penalties: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
}

impl LapData {
    fn parse(input: &[u8]) -> ParseResult<LapData> {
        map(
            tuple((
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_u8,
                le_u8,
                PitStatus::parse,
                Sector::parse,
                LapState::parse,
                le_u8,
                le_u8,
                DriverStatus::parse,
                ResultStatus::parse,
            )),
            |(
                last_lap_time,
                current_lap_time,
                best_lap_time,
                sector1_time,
                sector2_time,
                lap_distance,
                total_distance,
                safety_car_delta,
                car_position,
                current_lap_num,
                pit_status,
                sector,
                current_lap_invalid,
                penalties,
                grid_position,
                driver_status,
                result_status,
            )| LapData {
                last_lap_time,
                current_lap_time,
                best_lap_time,
                sector1_time,
                sector2_time,
                lap_distance,
                total_distance,
                safety_car_delta,
                car_position,
                current_lap_num,
                pit_status,
                sector,
                current_lap_invalid,
                penalties,
                grid_position,
                driver_status,
                result_status,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct PacketLapData {
    pub lap_data: Vec<LapData>,
}

impl PacketLapData {
    pub fn parse(input: &[u8]) -> ParseResult<PacketLapData> {
        map(count(LapData::parse, 20), |lap_data| PacketLapData {
            lap_data,
        })(input)
    }
}
