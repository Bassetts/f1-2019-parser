use std::convert::TryFrom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;

use crate::ParseResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum VehicleController {
    Human = 0,
    AI = 1,
}

#[non_exhaustive]
pub struct InvalidVehicleController(());

impl InvalidVehicleController {
    fn new() -> Self {
        InvalidVehicleController(())
    }
}

impl TryFrom<u8> for VehicleController {
    type Error = InvalidVehicleController;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(VehicleController::Human),
            1 => Ok(VehicleController::AI),
            _ => Err(InvalidVehicleController::new()),
        }
    }
}

impl VehicleController {
    pub fn parse(input: &[u8]) -> ParseResult<Self> {
        map_res(le_u8, |vehicle_controller: u8| {
            VehicleController::try_from(vehicle_controller)
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_parse() {
        let packet = 0u8.to_le_bytes();
        let result = VehicleController::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VehicleController::Human)));

        let packet = 1u8.to_le_bytes();
        let result = VehicleController::parse(&packet[..]);
        assert_eq!(result, Ok((&[][..], VehicleController::AI)));

        let packet = 2u8.to_le_bytes();
        let result = VehicleController::parse(&packet[..]);
        assert_eq!(result, Err(Err::Error((&packet[..], ErrorKind::MapRes))));
    }
}
