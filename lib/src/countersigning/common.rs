use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::game_result::GameResult;

/**
 * 1. Alice and Bob start to play a game.
 * 2. Alice finishes the game, calculates the ELO update, and requests a countersigning with Bob.
 *      with both the last game result for Alice and for Bob.
 * 3. At this point, Bob may have finished a parallel game and published a new result that Alice hasn't seen.
 * 4. If this is the case, Bob responds with PublishGameResultResponse::OutdatedLastGameResult, and Alice tries again with the new latest_game_result for Bob.
 */

#[derive(Serialize, Deserialize, Debug)]
pub enum PublishGameResultResponse {
    InSession(PreflightResponse),
    OutdatedLastGameResult {
        latest_game_result_hash: HeaderHashB64,
    },
}


pub(crate) fn build_game_result_preflight(
    game_result: &GameResult,
) -> ExternResult<PreflightRequest> {
    let game_result_hash = hash_entry(game_result.clone())?;

    let times = session_times_from_millis(1_000)?;

    let agent_info = agent_info()?;

    let zome_info = zome_info()?;

    let header_base = HeaderBase::Create(CreateBase::new(EntryType::App(AppEntryType::new(
        EntryDefIndex::from(0),
        zome_info.zome_id,
        EntryVisibility::Public,
    ))));

    let opponent_address = game_result.counterparty()?;

    let countersigning_agents = vec![
        (agent_info.agent_latest_pubkey, vec![]),
        (opponent_address.clone().into(), vec![]),
    ];

    let bytes = SerializedBytes::try_from(game_result.clone())?;

    let preflight_bytes = PreflightBytes(bytes.bytes().to_vec());

    let preflight_request = PreflightRequest::try_new(
        game_result_hash,
        countersigning_agents,
        Some(0),
        times,
        header_base,
        preflight_bytes,
    )
    .map_err(|err| WasmError::Guest(format!("Could not create preflight request: {:?}", err)))?;

    Ok(preflight_request)
}
