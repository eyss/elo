use crate::game_result::GameResult;
use crate::{elo_rating::elo_rating_from_last_game_result, elo_rating_system::EloRatingSystem};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::handlers::{element_to_game_result, game_results_tag, get_my_last_game_result};
use super::EloUpdate;

pub fn unpublished_game_tag() -> LinkTag {
    LinkTag::new("unpublished_game")
}

pub fn try_resolve_unpublished_game_results<S: EloRatingSystem>() -> ExternResult<()> {
    let unpublished_game_results = get_my_unpublished_game_results()?;

    for unpublished_game_result in unpublished_game_results {
        create_game_result_and_resolve_flag::<S>(
            unpublished_game_result.game_result,
            unpublished_game_result.flag_link_hash,
        )?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct UnpublishedGameResult {
    game_result: GameResult,
    flag_link_hash: HeaderHash,
}

fn get_my_unpublished_game_results() -> ExternResult<Vec<UnpublishedGameResult>> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    let unpublished_links = get_links(my_pub_key.into(), Some(unpublished_game_tag()))?;

    let get_inputs = unpublished_links
        .clone()
        .into_inner()
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let maybe_elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let create_link_hashes: Vec<HeaderHash> = unpublished_links
        .into_inner()
        .into_iter()
        .map(|l| l.create_link_hash)
        .collect();

    // TODO: Check whether I've already published this game

    let game_results = maybe_elements
        .into_iter()
        .zip(create_link_hashes.into_iter())
        .filter_map(|(maybe_element, create_link_hash)| {
            maybe_element.map(|e| (e, create_link_hash))
        })
        .filter_map(|(e, create_link_header)| {
            let game_result: ExternResult<(HeaderHashed, GameResult)> = element_to_game_result(e);
            match game_result {
                Ok((_, gr)) => Some(UnpublishedGameResult {
                    game_result: gr,
                    flag_link_hash: create_link_header,
                }),
                _ => None,
            }
        })
        .collect();

    Ok(game_results)
}

pub(crate) fn create_game_result_and_resolve_flag<S: EloRatingSystem>(
    game_result: GameResult,
    create_link_hash: HeaderHash,
) -> ExternResult<HeaderHash> {
    rebase_game_result::<S>(&mut game_result.clone())?;

    let header_hash = create_entry(game_result.clone())?;

    let game_result_hash = hash_entry(game_result.clone())?;

    create_link(
        agent_info()?.agent_latest_pubkey.into(),
        game_result_hash.clone(),
        game_results_tag(),
    )?;

    delete_link(create_link_hash)?;

    Ok(header_hash)
}

fn rebase_game_result<S: EloRatingSystem>(old_game_result: &mut GameResult) -> ExternResult<()> {
    let maybe_my_last_game_result = get_my_last_game_result()?;

    // Get the previous game result for the opponent
    let opponent = old_game_result.opponent()?;
    let elo_update = old_game_result
        .elo_update_for(&opponent)
        .ok_or(WasmError::Guest(
            "Unreachable: cannot find elo update for counterparty".into(),
        ))?;

    let previous_game_result = match elo_update.previous_game_result {
        Some(header_hash) => {
            let element = get(
                AnyDhtHash::from(HeaderHash::from(header_hash)),
                GetOptions::default(),
            )?;

            match element {
                Some(e) => Ok(Some(element_to_game_result(e)?)),
                None => Err(WasmError::Guest(
                    "Cannot get the last game for counterparty".into(),
                )),
            }
        }
        None => Ok(None),
    }?;

    let my_pub_key = AgentPubKeyB64::from(agent_info()?.agent_latest_pubkey);
    let opponent_previous_elo =
        elo_rating_from_last_game_result::<S>(&opponent, &previous_game_result)?;
    let my_previous_elo =
        elo_rating_from_last_game_result::<S>(&my_pub_key, &maybe_my_last_game_result)?;

    let am_i_player_a = old_game_result.player_a.player_address.eq(&my_pub_key);

    let (player_a, player_b) = match am_i_player_a {
        true => (my_previous_elo, opponent_previous_elo),
        false => (opponent_previous_elo, my_previous_elo),
    };

    let (player_a_new_elo, player_b_new_elo) = skill_rating::elo::game(
        player_a,
        player_b,
        old_game_result.score_player_a,
        S::k_factor(),
        S::k_factor(),
    );

    let previous_game_result =
        maybe_my_last_game_result.map(|(h, _)| HeaderHashB64::from(h.into_hash()));

    if am_i_player_a {
        old_game_result.player_a = EloUpdate {
            player_address: my_pub_key,
            current_elo: player_a_new_elo,
            previous_game_result,
        };
    } else {
        old_game_result.player_b = EloUpdate {
            player_address: my_pub_key,
            current_elo: player_b_new_elo,
            previous_game_result,
        };
    }

    Ok(())
}
