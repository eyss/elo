pub use countersigning::{accept_game_publish_result_request, init};
pub use skill_rating::elo::{EloRating, DRAW, LOSS, WIN};

use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

mod countersigning;
mod elo_rating;
mod game_result;

use elo_rating::*;
use game_result::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGameResultRequestInput {
    opponent_address: AgentPubKeyB64,
    my_score: f32,
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
#[hdk_extern]
pub fn send_publish_game_result_request(input: NewGameResultRequestInput) -> ExternResult<()> {
    countersigning::send_publish_game_result_request(input.opponent_address, input.my_score)
}

/**
 * Get the ELO ratings for the given users
 */
#[hdk_extern]
pub fn get_elo_rating_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, EloRating>> {
    elo_rating::get_elo_rating_for_agents(agent_pub_keys)
}

#[hdk_extern]
pub fn get_games_results_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, BTreeMap<EntryHashB64, (Timestamp, GameResult)>>> {
    game_result::get_games_results_for_agents(agent_pub_keys)
}
