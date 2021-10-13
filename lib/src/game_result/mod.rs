use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use skill_rating::elo::EloRating;

pub mod handlers;
pub mod validation;

#[hdk_entry(id = "game_result")]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct GameResult {
    pub player_a: EloUpdate,
    pub player_b: EloUpdate,
    pub score_player_a: f32,
    pub game_info: SerializedBytes,
}

impl GameResult {
    pub fn elo_update_for(&self, agent: &AgentPubKeyB64) -> Option<EloUpdate> {
        if self.player_a.player_address.clone().eq(agent) {
            return Some(self.player_a.clone());
        } else if self.player_b.player_address.clone().eq(agent) {
            return Some(self.player_b.clone());
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct EloUpdate {
    pub player_address: AgentPubKeyB64,
    pub current_elo: EloRating,
    // Will be None in the first GameResult entry for that player
    pub previous_game_result: Option<HeaderHashB64>,
}

pub struct GameResultInfo {
    pub player_a: AgentPubKeyB64,
    pub player_b: AgentPubKeyB64,
    pub score_player_a: f32,
}

impl GameResultInfo {
    pub fn new(game_result: &GameResult) -> Self {
        GameResultInfo {
            player_a: game_result.player_a.player_address.clone(),
            player_b: game_result.player_b.player_address.clone(),
            score_player_a: game_result.score_player_a,
        }
    }
}
