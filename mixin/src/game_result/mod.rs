use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use skill_rating::elo::EloRating;

pub mod handlers;
pub mod unpublished;
pub mod validation;

#[hdk_entry(id = "game_result", required_validation_type = "sub_chain")]
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

    pub fn opponent(&self) -> ExternResult<AgentPubKeyB64> {
        let agent_info = agent_info()?;

        let agents = self.agents();
        if AgentPubKey::from(agents.0.clone()).eq(&agent_info.agent_latest_pubkey) {
            Ok(agents.1.clone())
        } else if AgentPubKey::from(agents.1).eq(&agent_info.agent_latest_pubkey) {
            Ok(agents.0.clone())
        } else {
            Err(WasmError::Guest(
                "This GameResult does not have my agent pub key in it".into(),
            ))
        }
    }

    pub fn agents(&self) -> (AgentPubKeyB64, AgentPubKeyB64) {
        (
            self.player_a.player_address.clone(),
            self.player_b.player_address.clone(),
        )
    }

    pub fn entry_type() -> ExternResult<EntryType> {
        Ok(EntryType::App(AppEntryType::new(
            EntryDefIndex::from(0),
            zome_info()?.id,
            EntryVisibility::Public,
        )))
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EloSignal {
    NewGameResult {
        entry_hash: EntryHashB64,
        game_result: GameResult,
    },
}
