use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{
    countersigning::sender::try_create_countersigned_game_result,
    game_result::handlers::{build_new_game_result, create_unilateral_game_result_and_flag},
    EloRatingSystem,
};

pub fn init_elo() -> ExternResult<()> {
    // TODO: restrict to only the agents with which we are playing
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "request_publish_game_result".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    schedule("scheduled_try_resolve_unpublished_game_results")?;

    Ok(())
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn attempt_create_countersigned_game_result<S: EloRatingSystem>(
    game_info: S::GameInfo,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<()> {
    let bytes: SerializedBytes = game_info.try_into().or(Err(WasmError::Guest(String::from(
        "Error converting game info into SerializedBytes",
    ))))?;

    try_create_countersigned_game_result::<S>(bytes, opponent_address, my_score)
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn create_unilateral_game_result<S: EloRatingSystem>(
    game_info: S::GameInfo,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<()> {
    let bytes: SerializedBytes = game_info.try_into().or(Err(WasmError::Guest(String::from(
        "Error converting game info into SerializedBytes",
    ))))?;

    let new_game_result = build_new_game_result::<S>(bytes, &opponent_address, my_score)?;
    create_unilateral_game_result_and_flag(new_game_result)?;

    Ok(())
}

#[macro_export]
macro_rules! mixin_elo {
    ( $elo_rating_system:ty ) => {
        use hdk::prelude::holo_hash::*;
        use hdk::prelude::*;
        use std::collections::BTreeMap;

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
        ) -> ExternResult<$crate::PublishGameResultResponse> {
            $crate::handle_request_publish_game_result::<$elo_rating_system>(
                counterparty_preflight_response,
            )
        }

        /**
         * Get the game results for the given agents
         */
        #[hdk_extern]
        pub fn get_games_results_for_agents(
            agent_pub_keys: Vec<AgentPubKeyB64>,
        ) -> ExternResult<BTreeMap<AgentPubKeyB64, Vec<(HeaderHashed, $crate::GameResult)>>> {
            $crate::get_games_results_for_agents(agent_pub_keys)
        }

        /**
         * Get the game results for the given agents
         */
        #[hdk_extern]
        pub fn scheduled_try_resolve_unpublished_game_results(
            _: Option<Schedule>,
        ) -> ExternResult<Option<Schedule>> {
            $crate::try_resolve_unpublished_game_results::<$elo_rating_system>()?;
            Ok(Some(Schedule::Persisted(format!(
                "0/{} * * * *",
                <$elo_rating_system>::unpublished_games_retry_interval_in_seconds()
            ))))
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
