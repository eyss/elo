use std::collections::BTreeMap;

use crate::elo_rating_system::EloRatingSystem;
use crate::game_result::handlers::get_last_game_result_for_agents;
pub use skill_rating::elo::{EloRating, DRAW, LOSS, WIN};

use crate::game_result::GameResult;
use ::hdk::prelude::holo_hash::AgentPubKeyB64;
use ::hdk::prelude::*;

pub fn get_elo_rating_for_agents<S: EloRatingSystem>(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, EloRating>> {
    let last_result_by_agent = get_last_game_result_for_agents(agent_pub_keys)?;

    // For each agent, extract their ELO rating from their latest game result
    let mut elo_ratings_by_agent: BTreeMap<AgentPubKeyB64, EloRating> = BTreeMap::new();

    for (agent_pub_key, latest_game_result) in last_result_by_agent {
        let elo_rating =
            elo_rating_from_last_game_result::<S>(&agent_pub_key, &latest_game_result)?;
        elo_ratings_by_agent.insert(agent_pub_key, elo_rating);
    }

    Ok(elo_ratings_by_agent)
}

pub(crate) fn elo_rating_from_last_game_result<S: EloRatingSystem>(
    agent_pub_key: &AgentPubKeyB64,
    last_game_result: &Option<(HeaderHashed, GameResult)>,
) -> ExternResult<EloRating> {
    match last_game_result {
        Some(game_result) => {
            let elo_update =
                game_result
                    .1
                    .elo_update_for(agent_pub_key)
                    .ok_or(WasmError::Guest(format!(
                        "Agent {} was not a player of this game",
                        agent_pub_key
                    )))?;

            Ok(elo_update.current_elo)
        }
        None => Ok(S::initial_rating()),
    }
}
