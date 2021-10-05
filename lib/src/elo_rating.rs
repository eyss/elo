use std::collections::BTreeMap;

use crate::{game_result::GameResult, get_last_game_result_for_agents, EloRating};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn initial_elo_rating() -> EloRating {
    1000
}

pub fn get_elo_rating_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, EloRating>> {
    let last_result_by_agent = get_last_game_result_for_agents(agent_pub_keys)?;

    // For each agent, extract their ELO rating from their latest game result
    let mut elo_ratings_by_agent: BTreeMap<AgentPubKeyB64, EloRating> = BTreeMap::new();

    for (agent_pub_key, latest_game_result) in last_result_by_agent {
        let elo_rating = elo_rating_from_last_game_result(&agent_pub_key, &latest_game_result)?;
        elo_ratings_by_agent.insert(agent_pub_key, elo_rating);
    }

    Ok(elo_ratings_by_agent)
}

pub(crate) fn elo_rating_from_last_game_result(
    agent_pub_key: &AgentPubKeyB64,
    last_game_result: &Option<GameResult>,
) -> ExternResult<EloRating> {
    match last_game_result {
        Some(game_result) => game_result.elo_rating_for(agent_pub_key),
        None => Ok(initial_elo_rating()),
    }
}
