use cosmwasm_std::{Coin, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{RPS, GameResult};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    NewGame {
        player_name: String,
        bet: Option<Coin>,
    },
    JoinGame { 
        player_name: String,
        game_code: String,
     },
    SubmitChoice { 
        game_code: String,
        choice: RPS,
     },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    WhoWhon {game: String},
    GameState {game: String},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CheckWinner {
    pub winner: GameResult,
    pub address: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameStateResponse {
    pub game: String,
    pub state: crate::state::GameStatus,
}
