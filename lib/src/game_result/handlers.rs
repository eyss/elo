use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{elo_rating::elo_rating_from_last_game_result, elo_rating_system::EloRatingSystem};

use super::{unpublished::unpublished_game_tag, EloUpdate, GameResult};

pub(crate) fn create_unilateral_game_result_and_flag(
    game_result: GameResult,
) -> ExternResult<HeaderHashB64> {
    let header_hash = create_entry(game_result.clone())?;

    let opponent = game_result.opponent()?;

    let game_result_hash = hash_entry(game_result)?;

    create_link(
        agent_info()?.agent_latest_pubkey.into(),
        game_result_hash.clone(),
        game_results_tag(),
    )?;

    create_link(
        AgentPubKey::from(opponent).into(),
        game_result_hash,
        unpublished_game_tag(),
    )?;

    Ok(header_hash.into())
}

pub(crate) fn create_countersigned_game_result(
    game_result: GameResult,
    responses: Vec<PreflightResponse>,
) -> ExternResult<EntryHashB64> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(responses).map_err(
                |countersigning_error| WasmError::Guest(countersigning_error.to_string()),
            )?,
        ),
        game_result.clone().try_into()?,
    );

    let game_result_hash = hash_entry(entry.clone())?;
    HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            (&game_result).into(),
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })?;

    Ok(game_result_hash.into())
}

pub fn get_game_results_for_agents(
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

pub(crate) fn get_my_last_game_result() -> ExternResult<Option<(HeaderHashed, GameResult)>> {
    let agent_info = agent_info()?;

    let game_results =
        get_last_game_result_for_agents(vec![agent_info.agent_latest_pubkey.clone().into()])?;

    let my_actual_latest_game_result = game_results
        .get(&AgentPubKeyB64::from(agent_info.agent_latest_pubkey))
        .ok_or(WasmError::Guest("Unreachable".into()))?;

    Ok(my_actual_latest_game_result.clone())
}

pub(crate) fn get_last_game_result_for_agents(
    agent_pub_keys: Vec<AgentPubKeyB64>,
) -> ExternResult<BTreeMap<AgentPubKeyB64, Option<(HeaderHashed, GameResult)>>> {
    // Get the game results links for the agents
    let mut game_results_links_by_agent = get_game_results_links_for_agents(agent_pub_keys)?;

    // We only care about the latest published game result for an agent
    // since it will contain the latest ELO for that agent
    for (agent_pub_key, mut links) in game_results_links_by_agent.clone() {
        links.sort_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));

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

pub fn game_results_tag() -> LinkTag {
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
    let all_game_results_elements = HDK.with(|hdk| hdk.borrow().get_details(game_results_input))?;

    let mut index = 0;

    for (agent_pub_key, links) in game_results_links_by_agent.into_iter() {
        let mut results_for_agent: Vec<(HeaderHashed, GameResult)> = Vec::new();
        for _ in links {
            if let Some(game_result_details) = all_game_results_elements[index].clone() {
                if let Details::Entry(entry_details) = game_result_details {
                    let header_for_author = entry_details.headers.into_iter().find(|h| {
                        h.header()
                            .author()
                            .eq(&AgentPubKey::from(agent_pub_key.clone()))
                    });

                    if let Some(header) = header_for_author {
                        let game_result = entry_to_game_result(&entry_details.entry)?;

                        results_for_agent.push((header.header_hashed().clone(), game_result));
                    }
                }
            }
            index += 1;
        }
        game_results.insert(agent_pub_key, results_for_agent);
    }

    Ok(game_results)
}

pub fn element_to_game_result(element: Element) -> ExternResult<(HeaderHashed, GameResult)> {
    let entry = element
        .entry()
        .as_option()
        .ok_or(WasmError::Guest("Malformed GameResults entry".into()))?;

    let game_result = entry_to_game_result(entry)?;

    Ok((element.header_hashed().clone(), game_result))
}

pub fn entry_to_game_result(entry: &Entry) -> ExternResult<GameResult> {
    let bytes = match entry.clone() {
        Entry::App(bytes) => Ok(bytes.into_sb()),
        Entry::CounterSign(_, bytes) => Ok(bytes.into_sb()),
        _ => Err(WasmError::Guest("Malformed GameResults entry".into())),
    }?;

    let game_result = GameResult::try_from(bytes)
        .or(Err(WasmError::Guest("Malformed GameResults entry".into())))?;

    Ok(game_result)
}
