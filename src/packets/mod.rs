pub use self::car_setups::PacketCarSetupData;
pub use self::car_status::PacketCarStatusData;
pub use self::car_telemetry::PacketCarTelemetryData;
pub use self::event::EventData;
pub use self::final_classification::FinalClassificationsData;
pub use self::header::Header;
pub use self::lap_data::PacketLapData;
pub use self::lobby_info::LobbyInfoDatas;
pub use self::motion::MotionData;
pub use self::participants::ParticipantsData;
pub use self::session::SessionData;

mod car_setups;
mod car_status;
mod car_telemetry;
mod event;
mod final_classification;
pub(crate) mod header;
mod lap_data;
mod lobby_info;
mod motion;
mod participants;
mod session;

pub(crate) const MAX_NAME_LENGTH: usize = 48;
