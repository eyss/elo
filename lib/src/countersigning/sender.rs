use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::common::{create_game_result, PublishGameResultResponse};
use crate::{
    elo_rating_system::EloRatingSystem,
    game_result::{
        handlers::{build_new_game_result, build_new_game_result_with_new_opponent_game_result},
        GameResult,
    },
    unpublished::UnpublishedGameResult,
};

#[derive(Serialize, Debug, Deserialize)]
pub enum AttemptPublishGameResultOutcome {
    Published,
    Scheduled,
}

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn send_publish_game_result_request<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<AttemptPublishGameResultOutcome> {
    let new_game_result = build_new_game_result::<S>(game_info, &opponent_address, my_score)?;
    send_publish_game_result_request_for_new_game_result::<S>(
        opponent_address,
        my_score,
        new_game_result,
    )
}

fn send_publish_game_result_request_for_new_game_result<S: EloRatingSystem>(
    opponent_address: AgentPubKeyB64,
    my_score: f32,
    new_game_result: GameResult,
) -> ExternResult<AttemptPublishGameResultOutcome> {
    let new_game_result_hash = hash_entry(new_game_result.clone())?;

    let times = session_times_from_millis(10_000)?;

    let agent_info = agent_info()?;

    let zome_info = zome_info()?;

    let header_base = HeaderBase::Create(CreateBase::new(EntryType::App(AppEntryType::new(
        EntryDefIndex::from(0),
        zome_info.zome_id,
        EntryVisibility::Public,
    ))));

    let countersigning_agents = vec![
        (agent_info.agent_latest_pubkey, vec![]),
        (opponent_address.clone().into(), vec![]),
    ];

    let bytes = SerializedBytes::try_from(new_game_result.clone())?;

    let preflight_bytes = PreflightBytes(bytes.bytes().to_vec());

    let preflight_request = PreflightRequest::try_new(
        new_game_result_hash,
        countersigning_agents,
        Some(0),
        times,
        header_base,
        preflight_bytes,
    )
    .map_err(|err| WasmError::Guest(format!("Could not create preflight request: {:?}", err)))?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error when building the preflight_request for the publishing of game result".into(),
        )),
    }?;

    let call_remote_result = call_remote(
        AgentPubKey::from(opponent_address.clone()),
        zome_info.zome_name,
        FunctionName("request_publish_game_result".into()),
        None,
        my_response.clone(),
    )?;

    match call_remote_result {
        ZomeCallResponse::Ok(response) => {
            let response: PublishGameResultResponse = response.decode()?;

            match response {
                PublishGameResultResponse::InSession(counterparty_preflight_response) => {
                    create_game_result(
                        new_game_result,
                        vec![my_response, counterparty_preflight_response],
                    )?;
                    Ok(AttemptPublishGameResultOutcome::Published)
                }
                PublishGameResultResponse::OutdatedLastGameResult {
                    latest_game_result_hash,
                } => handle_outdated_last_game_result::<S>(
                    new_game_result.game_info,
                    opponent_address,
                    my_score,
                    latest_game_result_hash,
                ),
            }
        }
        _ => schedule_for_later(new_game_result.game_info, opponent_address, my_score),
    }
}

fn handle_outdated_last_game_result<S: EloRatingSystem>(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
    latest_game_result_hash: HeaderHashB64,
) -> ExternResult<AttemptPublishGameResultOutcome> {
    // Retry, and schedule if it fails
    if let Some(element) = get(
        HeaderHash::from(latest_game_result_hash),
        GetOptions::default(),
    )? {
        let game_result: GameResult = element.entry().to_app_option()?.ok_or(WasmError::Guest(
            "Our opponent has sent a hash that doesn't correspond to a GameResult".into(),
        ))?;

        let new_game_result = build_new_game_result_with_new_opponent_game_result::<S>(
            game_info,
            &opponent_address,
            my_score,
            Some((element.header_hashed().clone(), game_result)),
        )?;
        send_publish_game_result_request_for_new_game_result::<S>(
            opponent_address,
            my_score,
            new_game_result,
        )
    } else {
        schedule_for_later(game_info, opponent_address, my_score)
    }
}

fn schedule_for_later(
    game_info: SerializedBytes,
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<AttemptPublishGameResultOutcome> {
    create_entry(UnpublishedGameResult {
        game_info,
        opponent_address,
        my_score,
    })?;

    Ok(AttemptPublishGameResultOutcome::Scheduled)
}
