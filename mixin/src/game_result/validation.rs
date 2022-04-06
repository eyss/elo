/*use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{
    elo_rating_system::EloRatingSystem,
    game_result::{handlers::element_to_game_result, GameResult, GameResultInfo},
};

use super::{handlers::internal_build_new_game_result, EloUpdate};

pub fn validate_entry_game_result<S: EloRatingSystem>(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    if validate_data
        .element
        .entry()
        .clone()
        .into_option()
        .is_none()
    {
        let entry_hash = validate_data
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

    let (_, game_result) = element_to_game_result(validate_data.element.clone())?;

    validate_game_result::<S>(validate_data, game_result)
}

fn validate_game_result<S: EloRatingSystem>(
    validate_data: ValidateData,
    game_result: GameResult,
) -> ExternResult<ValidateCallbackResult> {
    if game_result.score_player_a < 0_f32 || game_result.score_player_a > 1_f32 {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "The score of a player must be between 0_f32 and 1_f32 (both 0 and 1 included)",
        )));
    }

    if game_result
        .player_a
        .player_address
        .eq(&game_result.player_b.player_address)
    {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Cannot publish a result where a player plays against themselves",
        )));
    }

    let author = validate_data.element.header().author();

    let package = validate_data.validation_package.ok_or(WasmError::Guest(
        "Validation package was not preset for game result entry".into(),
    ))?;

    let elo_update = game_result.elo_update_for(&AgentPubKeyB64::from(author.clone()));
    if let Some(update) = elo_update {
        validate_previous_game_result_hash(&package, &update)?;
    } else {
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

fn validate_previous_game_result_hash(
    validation_package: &ValidationPackage,
    elo_update: &EloUpdate,
) -> ExternResult<ValidateCallbackResult> {
    match elo_update.previous_game_result.clone() {
        None => {
            if validation_package.0.len() != 0 {
                return Ok(ValidateCallbackResult::Invalid("The player has committed other game results but their previous_game_result was None".into()));
            }
        }
        Some(hash) => {
            if let Some(element) = validation_package.0.last() {
                if !element.header_address().eq(&HeaderHash::from(hash)) {
                    return Ok(ValidateCallbackResult::Invalid(
                        "previous_game_result was not the latest game result for the agent".into(),
                    ));
                }
            } else {
                return Ok(ValidateCallbackResult::Invalid(
                    "previous_game_hash was Some but no elements were in the validation package"
                        .into(),
                ));
            }
        }
    }
    Ok(ValidateCallbackResult::Valid)
}

fn validate_elo_update_is_correct<S: EloRatingSystem>(
    game_result: GameResult,
) -> ExternResult<ValidateCallbackResult> {
    // Get player a previous game result
    let mut player_a: Option<(HeaderHashed, GameResult)> = None;
    if let Some(previous_game_result_hash) = game_result.player_a.previous_game_result.clone() {
        let element = must_get_valid_element(previous_game_result_hash.into())?;

        if element.entry().clone().into_option().is_none() {
            let entry_hash = element.header().entry_hash().ok_or(WasmError::Guest(
                "This header doesn't contain any entry hash".into(),
            ))?;
            return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                entry_hash.clone().into(),
            ]));
        }

        let (_, game_result) = element_to_game_result(element.clone())?;

        player_a = Some((element.header_hashed().clone(), game_result));
    }

    // Get player b previous game result
    let mut player_b: Option<(HeaderHashed, GameResult)> = None;
    if let Some(previous_game_result_hash) = game_result.player_b.previous_game_result.clone() {
        let element = must_get_valid_element(previous_game_result_hash.into())?;

        if element.entry().clone().into_option().is_none() {
            let entry_hash = element.header().entry_hash().ok_or(WasmError::Guest(
                "This header doesn't contain any entry hash".into(),
            ))?;
            return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                entry_hash.clone().into(),
            ]));
        }

        let (_, game_result) = element_to_game_result(element.clone())?;

        player_b = Some((element.header_hashed().clone(), game_result));
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
}*/
