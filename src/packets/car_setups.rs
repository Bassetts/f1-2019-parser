use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u8};
use nom::sequence::tuple;

use crate::ParseResult;

#[derive(Debug, Copy, Clone)]
pub struct CarSetupData {
    pub front_wing: u8,
    pub rear_wing: u8,
    pub on_throttle: u8,
    pub off_throttle: u8,
    pub front_camber: f32,
    pub rear_camber: f32,
    pub front_toe: f32,
    pub rear_toe: f32,
    pub front_suspension: u8,
    pub rear_suspension: u8,
    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,
    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,
    pub brake_pressure: u8,
    pub brake_bias: u8,
    pub front_tyre_pressure: f32,
    pub rear_tyre_pressure: f32,
    pub ballast: u8,
    pub fuel_load: f32,
}

impl CarSetupData {
    fn parse(input: &[u8]) -> ParseResult<CarSetupData> {
        map(
            tuple((
                le_u8, le_u8, le_u8, le_u8, le_f32, le_f32, le_f32, le_f32, le_u8, le_u8, le_u8,
                le_u8, le_u8, le_u8, le_u8, le_u8, le_f32, le_f32, le_u8, le_f32,
            )),
            |(
                front_wing,
                rear_wing,
                on_throttle,
                off_throttle,
                front_camber,
                rear_camber,
                front_toe,
                rear_toe,
                front_suspension,
                rear_suspension,
                front_anti_roll_bar,
                rear_anti_roll_bar,
                front_suspension_height,
                rear_suspension_height,
                brake_pressure,
                brake_bias,
                front_tyre_pressure,
                rear_tyre_pressure,
                ballast,
                fuel_load,
            )| CarSetupData {
                front_wing,
                rear_wing,
                on_throttle,
                off_throttle,
                front_camber,
                rear_camber,
                front_toe,
                rear_toe,
                front_suspension,
                rear_suspension,
                front_anti_roll_bar,
                rear_anti_roll_bar,
                front_suspension_height,
                rear_suspension_height,
                brake_pressure,
                brake_bias,
                front_tyre_pressure,
                rear_tyre_pressure,
                ballast,
                fuel_load,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct PacketCarSetupData {
    pub car_setups: Vec<CarSetupData>,
}

impl PacketCarSetupData {
    pub fn parse(input: &[u8]) -> ParseResult<PacketCarSetupData> {
        map(count(CarSetupData::parse, 20), |car_setups| {
            PacketCarSetupData { car_setups }
        })(input)
    }
}
