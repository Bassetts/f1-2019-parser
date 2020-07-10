use bitflags::bitflags;
use nom::combinator::{map, map_opt};
use nom::multi::count;
use nom::number::complete::{le_f32, le_i8, le_u16, le_u32, le_u8};
use nom::sequence::tuple;

use crate::mappings::SurfaceType;
use crate::{ParseResult, WheelData};

type BrakeTemperatures = WheelData<u16>;
type TyreSurfaceTemperatures = WheelData<u8>;
type TyreInnerTemperatures = WheelData<u8>;
type TyrePressures = WheelData<f32>;
type WheelSurfaceTypes = WheelData<SurfaceType>;

impl WheelSurfaceTypes {
    fn parse(input: &[u8]) -> ParseResult<WheelData<SurfaceType>> {
        map(
            tuple((
                SurfaceType::parse,
                SurfaceType::parse,
                SurfaceType::parse,
                SurfaceType::parse,
            )),
            |(rear_left, rear_right, front_left, front_right)| WheelData {
                rear_left,
                rear_right,
                front_left,
                front_right,
            },
        )(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: u8,
    pub rev_lights_percentage: u8,
    pub brakes_temperature: BrakeTemperatures,
    pub tyres_surface_temperature: TyreSurfaceTemperatures,
    pub tyres_inner_temperature: TyreInnerTemperatures,
    pub engine_temperature: u16,
    pub tyres_pressure: TyrePressures,
    pub surface_type: WheelSurfaceTypes,
}

impl CarTelemetryData {
    fn parse(input: &[u8]) -> ParseResult<CarTelemetryData> {
        map(
            tuple((
                le_u16,
                le_f32,
                le_f32,
                le_f32,
                le_u8,
                le_i8,
                le_u16,
                le_u8,
                le_u8,
                BrakeTemperatures::parse_u16,
                TyreSurfaceTemperatures::parse_u8,
                TyreInnerTemperatures::parse_u8,
                le_u16,
                TyrePressures::parse_f32,
                WheelSurfaceTypes::parse,
            )),
            |(
                speed,
                throttle,
                steer,
                brake,
                clutch,
                gear,
                engine_rpm,
                drs,
                rev_lights_percentage,
                brakes_temperature,
                tyres_surface_temperature,
                tyres_inner_temperature,
                engine_temperature,
                tyres_pressure,
                surface_type,
            )| CarTelemetryData {
                speed,
                throttle,
                steer,
                brake,
                clutch,
                gear,
                engine_rpm,
                drs,
                rev_lights_percentage,
                brakes_temperature,
                tyres_surface_temperature,
                tyres_inner_temperature,
                engine_temperature,
                tyres_pressure,
                surface_type,
            },
        )(input)
    }
}

bitflags! {
  pub struct ButtonStatus: u32 {
    const A = 0x0001;
    const Y = 0x0002;
    const B = 0x0004;
    const X = 0x0008;
    const DPAD_LEFT = 0x0010;
    const DPAD_RIGHT = 0x0020;
    const DPAD_UP = 0x0040;
    const DPAD_DOWN = 0x0080;
    const MENU = 0x0100;
    const LB = 0x0200;
    const RB = 0x0400;
    const LT = 0x0800;
    const RT = 0x1000;
    const LEFT_STICK = 0x2000;
    const RIGHT_STICK = 0x4000;
  }
}

#[derive(Debug)]
pub struct PacketCarTelemetryData {
    pub car_telemetry_data: Vec<CarTelemetryData>,
    pub button_status: ButtonStatus,
    pub mfd_panel_index: u8,
    pub mfd_panel_index_secondary_player: u8,
    pub suggested_gear: i8,
}

impl PacketCarTelemetryData {
    pub fn parse(input: &[u8]) -> ParseResult<PacketCarTelemetryData> {
        map(
            tuple((
                count(CarTelemetryData::parse, 22),
                map_opt(le_u32, ButtonStatus::from_bits),
                le_u8,
                le_u8,
                le_i8,
            )),
            |(
                car_telemetry_data,
                button_status,
                mfd_panel_index,
                mfd_panel_index_secondary_player,
                suggested_gear,
            )| PacketCarTelemetryData {
                car_telemetry_data,
                button_status,
                mfd_panel_index,
                mfd_panel_index_secondary_player,
                suggested_gear,
            },
        )(input)
    }
}
