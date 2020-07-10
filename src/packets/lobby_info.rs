use nom::bytes::complete::{take, take_until};
use nom::combinator::{flat_map, map, map_res};
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::sequence::tuple;

use super::MAX_NAME_LENGTH;
use crate::mappings::{Nationality, TeamId, VehicleController};
use crate::ParseResult;

#[derive(Debug, Copy, Clone)]
pub struct LobbyInfoData<'a> {
  pub ai_controlled: VehicleController,
  pub team_id: TeamId,
  pub nationality: Nationality,
  pub name: &'a str,
  pub ready_status: u8,
}

impl<'a> LobbyInfoData<'a> {
  fn parse(input: &[u8]) -> ParseResult<LobbyInfoData> {
    map(
      tuple((
        VehicleController::parse,
        TeamId::parse,
        Nationality::parse,
        flat_map(map_res(take_until("\0"), std::str::from_utf8), |name| {
          map(take(MAX_NAME_LENGTH - name.len()), move |_| name)
        }),
        le_u8,
      )),
      |(ai_controlled, team_id, nationality, name, ready_status)| LobbyInfoData {
        ai_controlled,
        team_id,
        nationality,
        name,
        ready_status,
      },
    )(input)
  }
}

#[derive(Debug)]
pub struct LobbyInfoDatas<'a> {
  number_players: u8,
  lobby_players: Vec<LobbyInfoData<'a>>,
}

impl<'a> LobbyInfoDatas<'a> {
  pub fn parse(input: &[u8]) -> ParseResult<LobbyInfoDatas> {
    let (input, number_players) = le_u8(input)?;
    map(count(LobbyInfoData::parse, 22), move |lobby_players| {
      LobbyInfoDatas {
        number_players,
        lobby_players,
      }
    })(input)
  }
}
