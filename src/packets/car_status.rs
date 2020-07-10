use nom::combinator::map;
use nom::multi::many1;
use nom::number::complete::{le_f32, le_u16, le_u8};
use nom::sequence::{pair, tuple};

use crate::mappings::{
    AntiLockBrakes, DrsAllowed, ErsDeployMode, FiaFlag, FuelMix, TractionControl, TyreCompound,
    VisualCompound,
};
use crate::{ParseResult, WheelData};

type TyresDamage = WheelData<u8>;
type TyresWear = WheelData<u8>;

#[derive(Debug, Copy, Clone)]
pub struct CarStatusData {
    pub traction_control: TractionControl,
    pub anti_lock_brakes: AntiLockBrakes,
    pub fuel_mix: FuelMix,
    pub front_brake_bias: u8,
    pub pit_limiter_status: u8,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_remaining_laps: f32,
    pub max_rpm: u16,
    pub idle_rpm: u16,
    pub max_gears: u8,
    pub drs_allowed: DrsAllowed,
    pub drs_activation_distance: u16,
    pub tyres_wear: TyresWear,
    pub actual_tyre_compound: TyreCompound,
    pub tyre_visual_compound: VisualCompound,
    pub tyres_age_laps: u8,
    pub tyres_damage: TyresDamage,
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub drs_fault: u8,
    pub engine_damage: u8,
    pub gear_box_damage: u8,
    pub vehicle_fia_flags: FiaFlag,
    pub ers_store_energy: f32,
    pub ers_deploy_mode: ErsDeployMode,
    pub ers_harvested_this_lap_mguk: f32,
    pub ers_harvested_this_lap_mguh: f32,
    pub ers_deployed_this_lap: f32,
}

impl CarStatusData {
    fn parse(input: &[u8]) -> ParseResult<CarStatusData> {
        map(
            pair(
                tuple((
                    TractionControl::parse,
                    AntiLockBrakes::parse,
                    FuelMix::parse,
                    le_u8,
                    le_u8,
                    le_f32,
                    le_f32,
                    le_f32,
                    le_u16,
                    le_u16,
                    le_u8,
                    DrsAllowed::parse,
                    le_u16,
                    TyresWear::parse_u8,
                    TyreCompound::parse,
                    VisualCompound::parse,
                    le_u8,
                    TyresDamage::parse_u8,
                    le_u8,
                    le_u8,
                    le_u8,
                )),
                tuple((
                    le_u8,
                    le_u8,
                    le_u8,
                    FiaFlag::parse,
                    le_f32,
                    ErsDeployMode::parse,
                    le_f32,
                    le_f32,
                    le_f32,
                )),
            ),
            |(
                (
                    traction_control,
                    anti_lock_brakes,
                    fuel_mix,
                    front_brake_bias,
                    pit_limiter_status,
                    fuel_in_tank,
                    fuel_capacity,
                    fuel_remaining_laps,
                    max_rpm,
                    idle_rpm,
                    max_gears,
                    drs_allowed,
                    drs_activation_distance,
                    tyres_wear,
                    actual_tyre_compound,
                    tyre_visual_compound,
                    tyres_age_laps,
                    tyres_damage,
                    front_left_wing_damage,
                    front_right_wing_damage,
                    rear_wing_damage,
                ),
                (
                    drs_fault,
                    engine_damage,
                    gear_box_damage,
                    vehicle_fia_flags,
                    ers_store_energy,
                    ers_deploy_mode,
                    ers_harvested_this_lap_mguk,
                    ers_harvested_this_lap_mguh,
                    ers_deployed_this_lap,
                ),
            )| {
                CarStatusData {
                    traction_control,
                    anti_lock_brakes,
                    fuel_mix,
                    front_brake_bias,
                    pit_limiter_status,
                    fuel_in_tank,
                    fuel_capacity,
                    fuel_remaining_laps,
                    max_rpm,
                    idle_rpm,
                    max_gears,
                    drs_allowed,
                    drs_activation_distance,
                    tyres_wear,
                    actual_tyre_compound,
                    tyre_visual_compound,
                    tyres_age_laps,
                    tyres_damage,
                    front_left_wing_damage,
                    front_right_wing_damage,
                    rear_wing_damage,
                    drs_fault,
                    engine_damage,
                    gear_box_damage,
                    vehicle_fia_flags,
                    ers_store_energy,
                    ers_deploy_mode,
                    ers_harvested_this_lap_mguk,
                    ers_harvested_this_lap_mguh,
                    ers_deployed_this_lap,
                }
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct PacketCarStatusData {
    pub car_status_data: Vec<CarStatusData>,
}

impl PacketCarStatusData {
    pub fn parse(input: &[u8]) -> ParseResult<PacketCarStatusData> {
        map(many1(CarStatusData::parse), move |car_status_data| {
            PacketCarStatusData { car_status_data }
        })(input)
    }
}
