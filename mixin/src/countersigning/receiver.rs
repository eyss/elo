use hdk::prelude::*;

use crate::{
    elo_rating_system::EloRatingSystem,
    game_result::{handlers::create_countersigned_game_result, GameResult, GameResultInfo},
};

/**
 * Receives the publish game result request, verifies that we don't have any latest game result, and creates the first part of the countersigned entry
 */
pub fn handle_request_publish_game_result<S: EloRatingSystem>(
    counterparty_preflight_response: PreflightResponse,
) -> ExternResult<PreflightResponse> {
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

    let my_response = match accept_countersigning_preflight_request(request.clone())? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error accepting the publishing of game result".into(),
        )),
    }?;

    let responses = vec![counterparty_preflight_response, my_response.clone()];

    create_countersigned_game_result(game_result, responses)?;

    Ok(my_response)
}