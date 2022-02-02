use hdk::prelude::*;

use crate::{
    elo_rating_system::EloRatingSystem,
    game_result::{handlers::create_countersigned_game_result, GameResult, GameResultInfo},
};

use super::common::PublishGameResultResponse;

/**
 * Receives the publish game result request, verifies that we don't have any latest game result, and creates the first part of the countersigned entry
 */
pub fn handle_request_publish_game_result<S: EloRatingSystem>(
    counterparty_preflight_response: PreflightResponse,
) -> ExternResult<PublishGameResultResponse> {
    let request = counterparty_preflight_response.request();

    let game_result: GameResult =
        SerializedBytes::from(UnsafeBytes::from(request.preflight_bytes().0.clone())).try_into()?;

    let info = S::GameInfo::try_from(game_result.game_info.clone()).or(Err(WasmError::Guest(
        "Could not conver GameInfo into SerializedBytes".into(),
    )))?;
    let game_result_info = GameResultInfo::new(&game_result);

    let validation_output = S::validate_game_result(info, game_result_info);

    match validation_output {
        Ok(ValidateCallbackResult::Valid) => Ok(()),
        _ => Err(WasmError::Guest(
            format!("The game result that the opponent is trying to make me sign is actually not valid: {:?}", validation_output),
        )),
    }?;
    /*
    if let IsGameResultHashOutdated::Yes {
        latest_game_result_hash,
    } = is_request_game_result_hash_outdated(&game_result)?
    {
        return Ok(PublishGameResultResponse::OutdatedLastGameResult {
            latest_game_result_hash,
        });
    } */

    let my_response = match accept_countersigning_preflight_request(request.clone())? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error accepting the publishing of game result".into(),
        )),
    }?;

    let responses = vec![counterparty_preflight_response, my_response.clone()];

    create_countersigned_game_result(game_result, responses)?;

    Ok(PublishGameResultResponse::InSession(my_response))
}

/*
enum IsGameResultHashOutdated {
    Yes {
        latest_game_result_hash: HeaderHashB64,
    },
    No,
}

fn is_request_game_result_hash_outdated(
    game_result: &GameResult,
) -> ExternResult<IsGameResultHashOutdated> {
    let agent_info = agent_info()?;

    let my_elo_update = game_result
        .elo_update_for(&agent_info.agent_latest_pubkey.clone().into())
        .ok_or(WasmError::Guest(String::from(
            "I am not a player of this game",
        )))?;

    let my_latest_game_result_hash_from_prefligh_request = my_elo_update.previous_game_result;

    let my_actual_latest_game_result = get_my_last_game_result()?;

    match (my_latest_game_result_hash_from_prefligh_request, my_actual_latest_game_result) {
        (None, None) => Ok(IsGameResultHashOutdated::No),
        (Some(game_result_hash_from_request), Some(game_result)) => {
            match HeaderHash::from(game_result_hash_from_request).eq(game_result.0.as_hash()) {
                true =>  Ok(IsGameResultHashOutdated::No),
                false => Ok(IsGameResultHashOutdated::Yes {
                    latest_game_result_hash: game_result.0.as_hash().clone().into()
                })
            }
        },
        (None, Some(game_result)) => {
            Ok(IsGameResultHashOutdated::Yes{
                latest_game_result_hash: game_result.0.as_hash().clone().into()
            })
        }
        _ => Err(WasmError::Guest("Unreachable: the requesting agent knows about a game result for the reponse agent that not even the response agent knows about".into()))
    }
}
 */
