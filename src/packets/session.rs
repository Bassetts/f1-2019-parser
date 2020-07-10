use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{le_f32, le_i8, le_u16, le_u8};
use nom::sequence::tuple;

use crate::mappings::{Flag, Formula, NetworkGame, SafetyCarStatus, SessionType, TrackId, Weather};
use crate::ParseResult;

#[derive(Debug, Copy, Clone)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: Flag,
}

impl MarshalZone {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(tuple((le_f32, Flag::parse)), |(zone_start, zone_flag)| {
            MarshalZone {
                zone_start,
                zone_flag,
            }
        })(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
}

impl WeatherForecastSample {
    fn parse(input: &[u8]) -> ParseResult<Self> {
        map(
            tuple((SessionType::parse, le_u8, Weather::parse, le_i8, le_i8)),
            |(session_type, time_offset, weather, track_temperature, air_temperature)| {
                WeatherForecastSample {
                    session_type,
                    time_offset,
                    weather,
                    track_temperature,
                    air_temperature,
                }
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct SessionData {
    weather: Weather,
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: SessionType,
    track_id: TrackId,
    formula: Formula,
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: u8,
    num_marshal_zones: u8,
    marshal_zones: Vec<MarshalZone>,
    safety_car_status: SafetyCarStatus,
    network_game: NetworkGame,
    num_weather_forecast_samples: u8,
    weather_forecast_samples: Vec<WeatherForecastSample>,
}

impl SessionData {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        let (
            input,
            (
                weather,
                track_temperature,
                air_temperature,
                total_laps,
                track_length,
                session_type,
                track_id,
                formula,
                session_time_left,
                session_duration,
                pit_speed_limit,
                game_paused,
                is_spectating,
                spectator_car_index,
                sli_pro_native_support,
                num_marshal_zones,
            ),
        ) = tuple((
            Weather::parse,
            le_i8,
            le_i8,
            le_u8,
            le_u16,
            SessionType::parse,
            TrackId::parse,
            Formula::parse,
            le_u16,
            le_u16,
            le_u8,
            le_u8,
            le_u8,
            le_u8,
            le_u8,
            le_u8,
        ))(input)?;

        let (input, (marshal_zones, safety_car_status, network_game, num_weather_forecast_samples)) =
            tuple((
                count(MarshalZone::parse, num_marshal_zones as usize),
                SafetyCarStatus::parse,
                NetworkGame::parse,
                le_u8,
            ))(input)?;

        let (input, weather_forecast_samples) = count(
            WeatherForecastSample::parse,
            num_weather_forecast_samples as usize,
        )(input)?;

        Ok((
            input,
            SessionData {
                weather,
                track_temperature,
                air_temperature,
                total_laps,
                track_length,
                session_type,
                track_id,
                formula,
                session_time_left,
                session_duration,
                pit_speed_limit,
                game_paused,
                is_spectating,
                spectator_car_index,
                sli_pro_native_support,
                num_marshal_zones,
                marshal_zones,
                safety_car_status,
                network_game,
                num_weather_forecast_samples,
                weather_forecast_samples,
            },
        ))
    }
}
