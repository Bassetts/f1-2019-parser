use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16};
use nom::sequence::tuple;

use crate::{ParseResult, WheelData};

type WheelSuspensionPositions = WheelData<f32>;
type WheelVelocities = WheelData<f32>;
type WheelAccelerations = WheelData<f32>;
type WheelSpeeds = WheelData<f32>;
type WheelSlips = WheelData<f32>;

#[derive(Debug, Copy, Clone)]
pub struct Coordinates<T>
where
    T: Copy + Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Coordinates<f32> {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(tuple((le_f32, le_f32, le_f32)), |(x, y, z)| Coordinates {
            x,
            y,
            z,
        })(input)
    }
}

impl Coordinates<u16> {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(tuple((le_u16, le_u16, le_u16)), |(x, y, z)| Coordinates {
            x,
            y,
            z,
        })(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GForce {
    pub lateral: f32,
    pub longitudinal: f32,
    pub vertical: f32,
}

impl GForce {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(
            tuple((le_f32, le_f32, le_f32)),
            |(lateral, longitudinal, vertical)| GForce {
                lateral,
                longitudinal,
                vertical,
            },
        )(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RotationalAxes {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl RotationalAxes {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(tuple((le_f32, le_f32, le_f32)), |(yaw, pitch, roll)| {
            RotationalAxes { yaw, pitch, roll }
        })(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CarMotionData {
    pub world_position: Coordinates<f32>,
    pub world_velocity: Coordinates<f32>,
    pub world_forward_dir: Coordinates<u16>,
    pub world_right_dir: Coordinates<u16>,
    pub g_force: GForce,
    pub rotation: RotationalAxes,
}

impl CarMotionData {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(
            tuple((
                Coordinates::<f32>::parse,
                Coordinates::<f32>::parse,
                Coordinates::<u16>::parse,
                Coordinates::<u16>::parse,
                GForce::parse,
                RotationalAxes::parse,
            )),
            |(
                world_position,
                world_velocity,
                world_forward_dir,
                world_right_dir,
                g_force,
                rotation,
            )| {
                CarMotionData {
                    world_position,
                    world_velocity,
                    world_forward_dir,
                    world_right_dir,
                    g_force,
                    rotation,
                }
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct MotionData {
    pub car_motion_data: Vec<CarMotionData>,

    pub suspension_position: WheelSuspensionPositions,
    pub suspension_velocity: WheelVelocities,
    pub suspension_acceleration: WheelAccelerations,
    pub wheel_speed: WheelSpeeds,
    pub wheel_slip: WheelSlips,
    pub local_velocity: Coordinates<f32>,
    pub angular_velocity: Coordinates<f32>,
    pub angular_acceleration: Coordinates<f32>,
    pub front_wheels_angle: f32,
}

impl MotionData {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map(
            tuple((
                count(CarMotionData::parse, 20),
                WheelSuspensionPositions::parse_f32,
                WheelVelocities::parse_f32,
                WheelAccelerations::parse_f32,
                WheelSpeeds::parse_f32,
                WheelSlips::parse_f32,
                Coordinates::<f32>::parse,
                Coordinates::<f32>::parse,
                Coordinates::<f32>::parse,
                le_f32,
            )),
            |(
                car_motion_data,
                suspension_position,
                suspension_velocity,
                suspension_acceleration,
                wheel_speed,
                wheel_slip,
                local_velocity,
                angular_velocity,
                angular_acceleration,
                front_wheels_angle,
            )| MotionData {
                car_motion_data,
                suspension_position,
                suspension_velocity,
                suspension_acceleration,
                wheel_speed,
                wheel_slip,
                local_velocity,
                angular_velocity,
                angular_acceleration,
                front_wheels_angle,
            },
        )(input)
    }
}
