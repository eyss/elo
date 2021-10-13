use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{
    elo_rating_system::EloRatingSystem,
    game_result::{GameResult, GameResultInfo},
};

use super::handlers::internal_build_new_game_result;

pub fn validate_entry_game_result<S: EloRatingSystem>(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    match validate_data.element.entry().to_app_option()? {
        Some(game_result) => validate_game_result::<S>(validate_data, game_result),
        None => {
            let entry_hash =
                validate_data
                    .element
                    .header()
                    .entry_hash()
                    .ok_or(WasmError::Guest(
                        "This header doesn't contain any entry hash".into(),
                    ))?;
            return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                entry_hash.clone().into(),
            ]));
        }
    }
}

fn validate_game_result<S: EloRatingSystem>(
    validate_data: ValidateData,
    game_result: GameResult,
) -> ExternResult<ValidateCallbackResult> {
    let author = validate_data.element.header().author();
    if game_result
        .elo_update_for(&AgentPubKeyB64::from(author.clone()))
        .is_none()
    {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "The author of the element was not playing the game",
        )));
    }
    let game_result_info = GameResultInfo::new(&game_result);

    let game_info = S::GameInfo::try_from(game_result.game_info.clone()).or(Err(
        WasmError::Guest(String::from("Could not convert game info")),
    ))?;

    let validate_elo_result = validate_elo_update_is_correct::<S>(game_result)?;

    match validate_elo_result {
        ValidateCallbackResult::Valid => S::validate_game_result(game_info, game_result_info),
        _ => Ok(validate_elo_result),
    }
}

fn validate_elo_update_is_correct<S: EloRatingSystem>(
    game_result: GameResult,
) -> ExternResult<ValidateCallbackResult> {
    // Get player a previous game result
    let mut player_a: Option<(HeaderHashed, GameResult)> = None;
    if let Some(previous_game_result_hash) = game_result.player_a.previous_game_result.clone() {
        let element = must_get_valid_element(previous_game_result_hash.into())?;

        let maybe_game_result: Option<GameResult> = element.entry().to_app_option()?;

        if let Some(game_result) = maybe_game_result {
            player_a = Some((element.header_hashed().clone(), game_result));
        } else {
            match element.header().entry_hash() {
                Some(hash) => {
                    return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![hash
                        .clone()
                        .into()]));
                }
                None => {
                    return Ok(ValidateCallbackResult::Invalid(String::from(
                        "Invalid header for a game result",
                    )));
                }
            }
        }
    }

    // Get player b previous game result
    let mut player_b: Option<(HeaderHashed, GameResult)> = None;
    if let Some(previous_game_result_hash) = game_result.player_b.previous_game_result.clone() {
        let element = must_get_valid_element(previous_game_result_hash.into())?;

        let maybe_game_result: Option<GameResult> = element.entry().to_app_option()?;

        if let Some(game_result) = maybe_game_result {
            player_b = Some((element.header_hashed().clone(), game_result));
        } else {
            match element.header().entry_hash() {
                Some(hash) => {
                    return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![hash
                        .clone()
                        .into()]));
                }
                None => {
                    return Ok(ValidateCallbackResult::Invalid(String::from(
                        "Invalid header for a game result",
                    )));
                }
            }
        }
    }

    // Compute new game result and see it's equal
    let new_game_result = internal_build_new_game_result::<S>(
        game_result.game_info.clone(),
        &game_result.player_a.player_address,
        &game_result.player_b.player_address,
        game_result.score_player_a,
        player_a,
        player_b,
    )?;

    if !new_game_result.eq(&game_result) {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Invalid ELO score update",
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
