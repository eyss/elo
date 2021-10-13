use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{elo_rating::elo_rating_from_last_game_result, elo_rating_system::EloRatingSystem};

use super::{EloUpdate, GameResult};

pub fn get_games_results_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, Vec<(HeaderHashed, GameResult)>>> {
    let game_results_links = get_game_results_links_for_agents(agent_pub_keys)?;

    get_game_results_from_links(game_results_links)
}

pub(crate) fn build_new_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: &AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<GameResult> {
    let agent_info = agent_info()?;

    let my_address = AgentPubKeyB64::from(agent_info.agent_latest_pubkey);

    let results =
        get_last_game_result_for_agents(vec![my_address.clone(), opponent_address.clone()])?;

    let my_previous_game_result = results.get(&my_address).ok_or(WasmError::Guest(
        "Unreachable: error when getting my previous game result".into(),
    ))?;
    let opponent_previous_game_result = results.get(opponent_address).ok_or(WasmError::Guest(
        "Unreachable: error when getting the opponent's previous game result".into(),
    ))?;

    internal_build_new_game_result::<S>(
        game_info,
        &my_address,
        opponent_address,
        my_score,
        my_previous_game_result.clone(),
        opponent_previous_game_result.clone(),
    )
}

pub(crate) fn build_new_game_result_with_new_opponent_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: &AgentPubKeyB64,
    my_score: f32,
    opponent_latest_game_result: Option<(HeaderHashed, GameResult)>,
) -> ExternResult<GameResult> {
    let agent_info = agent_info()?;

    let my_address = AgentPubKeyB64::from(agent_info.agent_latest_pubkey);

    let results = get_last_game_result_for_agents(vec![my_address.clone()])?;

    let my_previous_game_result = results.get(&my_address).ok_or(WasmError::Guest(
        "Unreachable: error when getting my previous game result".into(),
    ))?;

    internal_build_new_game_result::<S>(
        game_info,
        &my_address,
        opponent_address,
        my_score,
        my_previous_game_result.clone(),
        opponent_latest_game_result,
    )
}

pub(crate) fn internal_build_new_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    player_a: &AgentPubKeyB64,
    player_b: &AgentPubKeyB64,
    score_player_a: f32,
    my_previous_game_result: Option<(HeaderHashed, GameResult)>,
    opponent_previous_game_result: Option<(HeaderHashed, GameResult)>,
) -> ExternResult<GameResult> {
    let my_previous_elo =
        elo_rating_from_last_game_result::<S>(player_a, &my_previous_game_result)?;
    let opponent_previous_elo =
        elo_rating_from_last_game_result::<S>(player_b, &opponent_previous_game_result)?;

    let (my_new_elo, opponent_new_elo) = skill_rating::elo::game(
        my_previous_elo,
        opponent_previous_elo,
        score_player_a,
        S::k_factor(),
        S::k_factor(),
    );

    let player_a = EloUpdate {
        player_address: player_a.clone(),
        current_elo: my_new_elo,
        previous_game_result: my_previous_game_result
            .map(|(header, _)| HeaderHashB64::from(header.into_hash())),
    };
    let player_b = EloUpdate {
        player_address: player_b.clone(),
        current_elo: opponent_new_elo,
        previous_game_result: opponent_previous_game_result
            .map(|(header, _)| HeaderHashB64::from(header.into_hash())),
    };

    let result = GameResult {
        game_info,
        player_a,
        player_b,
        score_player_a,
    };

    Ok(result)
}

/** Helper functions */

pub(crate) fn get_last_game_result_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, Option<(HeaderHashed, GameResult)>>> {
    // Get the game results links for the agents
    let mut game_results_links_by_agent = get_game_results_links_for_agents(agent_pub_keys)?;

    // We only care about the latest published game result for an agent
    // since it will contain the latest ELO for that agent
    for (agent_pub_key, mut links) in game_results_links_by_agent.clone() {
        links.sort_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

        let only_latest_link = match links.get(0) {
            Some(link) => vec![link.clone()],
            None => vec![],
        };

        game_results_links_by_agent.insert(agent_pub_key, only_latest_link);
    }

    // Actually get the latest game results indexed by agents
    let latest_game_results_by_agent = get_game_results_from_links(game_results_links_by_agent)?;

    let mut latest_result_by_agent: BTreeMap<AgentPubKeyB64, Option<(HeaderHashed, GameResult)>> =
        BTreeMap::new();

    for (agent_pub_key, game_results) in latest_game_results_by_agent {
        latest_result_by_agent.insert(agent_pub_key, game_results.into_iter().next());
    }

    Ok(latest_result_by_agent)
}

pub(crate) fn game_results_tag() -> LinkTag {
    LinkTag::new("game_result")
}

pub(crate) fn get_game_results_links_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, Vec<Link>>> {
    let input = agent_pub_keys
        .iter()
        .map(|pub_key| {
            GetLinksInput::new(
                EntryHash::from(AgentPubKey::from(pub_key.clone())),
                Some(game_results_tag()),
            )
        })
        .collect();
    let results = HDK.with(|hdk| hdk.borrow().get_links(input))?;

    let mut links: BTreeMap<AgentPubKeyB64, Vec<Link>> = BTreeMap::new();

    for (index, pub_key) in agent_pub_keys.into_iter().enumerate() {
        links.insert(pub_key, results[index].clone().into_inner());
    }

    Ok(links)
}

pub(crate) fn get_game_results_from_links(
    game_results_links_by_agent: BTreeMap<AgentPubKeyB64, Vec<Link>>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, Vec<(HeaderHashed, GameResult)>>> {
    let mut game_results: BTreeMap<AgentPubKeyB64, Vec<(HeaderHashed, GameResult)>> =
        BTreeMap::new();

    let game_results_input: Vec<GetInput> = game_results_links_by_agent
        .clone()
        .into_values()
        .flat_map(|links| {
            links
                .into_iter()
                .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        })
        .collect();
    let all_game_results_elements = HDK.with(|hdk| hdk.borrow().get(game_results_input))?;

    let mut index = 0;

    for (agent_pub_key, links) in game_results_links_by_agent.into_iter() {
        let mut results_for_agent: Vec<(HeaderHashed, GameResult)> = Vec::new();
        for _ in links {
            if let Some(game_result_element) = all_game_results_elements[index].clone() {
                let game_result: GameResult = game_result_element
                    .entry()
                    .to_app_option()?
                    .ok_or(WasmError::Guest("Malformed GameResults entry".into()))?;

                results_for_agent.push((game_result_element.header_hashed().clone(), game_result));
            }
            index += 1;
        }
        game_results.insert(agent_pub_key, results_for_agent);
    }

    Ok(game_results)
}
