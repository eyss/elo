use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::common::PublishGameResultResponse;
use crate::{
    countersigning::common::build_game_result_preflight,
    elo_rating_system::EloRatingSystem,
    game_result::{
        handlers::{build_new_game_result, create_countersigned_game_result},
        GameResult,
    },
};

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum CreateGameResultOutcome {
    Published { game_result_hash: EntryHashB64 },
    OutdatedLastGameResult,
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn try_create_countersigned_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<CreateGameResultOutcome> {
    let new_game_result = build_new_game_result::<S>(game_info, &opponent_address, my_score)?;

    send_publish_game_result_request::<S>(new_game_result)
}

pub fn send_publish_game_result_request<S: EloRatingSystem>(
    new_game_result: GameResult,
) -> ExternResult<CreateGameResultOutcome> {
    let preflight_request = build_game_result_preflight(&new_game_result)?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error when building the preflight_request for the publishing of game result".into(),
        )),
    }?;

    let opponent_address = new_game_result.opponent()?;

    let call_remote_result = call_remote(
        AgentPubKey::from(opponent_address.clone()),
        zome_info()?.zome_name,
        FunctionName("request_publish_game_result".into()),
        None,
        my_response.clone(),
    )?;

    match call_remote_result {
        ZomeCallResponse::Ok(response) => {
            let response: PublishGameResultResponse = response.decode()?;
            match response {
                PublishGameResultResponse::InSession(counterparty_preflight_response) => {
                    let entry_hash = create_countersigned_game_result(
                        new_game_result.clone(),
                        vec![my_response, counterparty_preflight_response],
                    )?;

                    Ok(CreateGameResultOutcome::Published { game_result_hash: entry_hash })
                }
                PublishGameResultResponse::OutdatedLastGameResult {
                    latest_game_result_hash: _,
                } => Ok(CreateGameResultOutcome::OutdatedLastGameResult),
            }
        }
        _ => Err(WasmError::Guest(format!(
            "There was an error calling the opponent's request_publish_game_result: {:?}",
            call_remote_result
        ))),
    }
}
