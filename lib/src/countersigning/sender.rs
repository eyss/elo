use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::common::PublishGameResultResponse;
use crate::{
    countersigning::common::build_game_result_preflight,
    elo_rating_system::EloRatingSystem,
    game_result::{
        handlers::{
            build_new_game_result, build_new_game_result_with_new_opponent_game_result,
            create_countersigned_game_result, element_to_game_result,
        },
        GameResult,
    },
};

#[derive(Serialize, Debug, Deserialize)]
pub enum AttemptPublishGameResultOutcome {
    Published,
    Scheduled,
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn try_create_countersigned_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<()> {
    let new_game_result = build_new_game_result::<S>(game_info, &opponent_address, my_score)?;

    send_publish_game_result_request::<S>(new_game_result)
}

pub fn send_publish_game_result_request<S: EloRatingSystem>(
    new_game_result: GameResult,
) -> ExternResult<()> {
    let preflight_request = build_game_result_preflight(&new_game_result)?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error when building the preflight_request for the publishing of game result".into(),
        )),
    }?;

    let opponent_address = new_game_result.counterparty()?;

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
                    create_countersigned_game_result(
                        new_game_result.clone(),
                        vec![my_response, counterparty_preflight_response],
                    )?;

                    Ok(())
                }
                PublishGameResultResponse::OutdatedLastGameResult {
                    latest_game_result_hash,
                } => handle_outdated_last_game_result::<S>(
                    new_game_result.game_info,
                    opponent_address,
                    new_game_result.score_player_a,
                    latest_game_result_hash,
                ),
            }
        }
        _ => Err(WasmError::Guest(
            "There was an error calling the opponent's request_publish_game_result".into(),
        )),
    }
}

fn handle_outdated_last_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
    latest_game_result_hash: HeaderHashB64,
) -> ExternResult<()> {
    // Retry, and schedule if it fails
    if let Some(element) = get(
        HeaderHash::from(latest_game_result_hash),
        GetOptions::default(),
    )? {
        let game_result = element_to_game_result(element)?;

        let new_game_result = build_new_game_result_with_new_opponent_game_result::<S>(
            game_info,
            &opponent_address,
            my_score,
            Some(game_result),
        )?;
        send_publish_game_result_request::<S>(new_game_result)
    } else {
        Err(WasmError::Guest(
            "Cannot get latest game published for the counterparty".into(),
        ))
    }
}
