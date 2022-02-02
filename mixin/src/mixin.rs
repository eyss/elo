use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{
    countersigning::sender::try_create_countersigned_game_result,
    game_result::handlers::{build_new_game_result, create_unilateral_game_result_and_flag},
    put_elo_rating_in_ranking, EloRatingSystem, GameResult,
};

pub fn init_elo<S: EloRatingSystem>() -> ExternResult<()> {
    // TODO: restrict to only the agents with which we are playing
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.name, "request_publish_game_result".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    let my_pub_key = agent_info()?.agent_initial_pubkey;

    put_elo_rating_in_ranking::<S>(
        my_pub_key.clone().into(),
        my_pub_key,
        None,
        S::initial_rating(),
    )?;

    schedule("scheduled_try_resolve_unpublished_game_results")?;

    Ok(())
}

pub fn post_commit_elo(headers: Vec<SignedHeaderHashed>) -> ExternResult<()> {
    let filter = ChainQueryFilter::new()
        .entry_type(GameResult::entry_type()?)
        .include_entries(true);
    let elements = query(filter)?;

    let header_hashes: Vec<HeaderHash> = headers
        .into_iter()
        .map(|shh| shh.header_address().clone())
        .collect();

    let newly_created_game_results_elements: Vec<Element> = elements
        .into_iter()
        .filter(|el| header_hashes.contains(el.header_address()))
        .collect();

    let new_entry_hashes: Vec<EntryHash> = newly_created_game_results_elements
        .into_iter()
        .filter_map(|el| el.header().entry_hash().map(|h| h.clone()))
        .map(|e| e.clone())
        .collect();

    if new_entry_hashes.len() > 0 {
        let agent_info = agent_info()?;

        call_remote(
            agent_info.agent_initial_pubkey,
            zome_info()?.name,
            "index_game_results".into(),
            None,
            new_entry_hashes,
        )?;
    }

    Ok(())
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn attempt_create_countersigned_game_result<S: EloRatingSystem>(
    game_info: S::GameInfo,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<EntryHashB64> {
    let bytes: SerializedBytes = game_info.try_into().or(Err(WasmError::Guest(String::from(
        "Error converting game info into SerializedBytes",
    ))))?;

    try_create_countersigned_game_result::<S>(bytes, opponent_address, my_score)
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn create_game_result_and_flag<S: EloRatingSystem>(
    game_info: S::GameInfo,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<EntryHashB64> {
    let bytes: SerializedBytes = game_info.try_into().or(Err(WasmError::Guest(String::from(
        "Error converting game info into SerializedBytes",
    ))))?;

    let new_game_result = build_new_game_result::<S>(bytes, &opponent_address, my_score)?;
    create_unilateral_game_result_and_flag::<S>(new_game_result)
}

#[macro_export]
macro_rules! mixin_elo {
    ( $elo_rating_system:ty ) => {
        use hdk::prelude::holo_hash::*;
        use hdk::prelude::*;
        use std::collections::BTreeMap;

        /**
         * Get the next chunk for the ELO ranking
         */
        #[hdk_extern]
        pub fn get_elo_ranking_chunk(input: GetEloRankingChunkInput) -> ExternResult<EloRanking> {
            $crate::get_elo_ranking_chunk(input.from_elo, input.agent_count)
        }

        /**
         * Get the ELO ratings for the given users
         */
        #[hdk_extern]
        pub fn get_elo_rating_for_agents(
            agent_pub_keys: Vec<AgentPubKeyB64>,
        ) -> ExternResult<BTreeMap<AgentPubKeyB64, $crate::EloRating>> {
            $crate::get_elo_rating_for_agents::<$elo_rating_system>(agent_pub_keys)
        }

        /**
         * Receives a request to publish a countersigned GameResult
         */
        #[hdk_extern]
        pub fn request_publish_game_result(
            counterparty_preflight_response: PreflightResponse,
        ) -> ExternResult<PreflightResponse> {
            $crate::handle_request_publish_game_result::<$elo_rating_system>(
                counterparty_preflight_response,
            )
        }

        /**
         * Get the game results for the given agents
         */
        #[hdk_extern]
        pub fn get_game_results_for_agents(
            agent_pub_keys: Vec<AgentPubKeyB64>,
        ) -> ExternResult<BTreeMap<AgentPubKeyB64, Vec<(HeaderHashed, $crate::GameResult)>>> {
            $crate::get_game_results_for_agents(agent_pub_keys)
        }

        /**
         * Called from post_commit, index the game result
         */
        #[hdk_extern]
        pub fn index_game_results(game_results_hashes: Vec<EntryHashB64>) -> ExternResult<()> {
            for hash_b64 in game_results_hashes {
                let hash = EntryHash::from(hash_b64);
                // TODO: remove linking from opponent when postcommit lands
                let element = get(hash.clone(), GetOptions::default())?
                    .ok_or(WasmError::Guest("Could not get game result".into()))?;

                let (_, game_result) = $crate::element_to_game_result(element)?;

                $crate::index_game_result_if_not_exists::<$elo_rating_system>(game_result.clone(), hash.clone())?;
            }

            Ok(())
        }

        /**
         * Get the game results for the given agents
         */
        #[hdk_extern(infallible)]
        pub fn scheduled_try_resolve_unpublished_game_results(
            _: Option<Schedule>,
        ) -> Option<Schedule> {
            let _r = $crate::try_resolve_unpublished_game_results::<$elo_rating_system>();
            Some(Schedule::Persisted(format!("* * * * *")))
        }

        /**
         * Validate the game_results entry
         */
        #[hdk_extern]
        pub fn validate_create_entry_game_result(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            $crate::validate_entry_game_result::<$elo_rating_system>(validate_data)
        }
    };
}
