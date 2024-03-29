use hdk::prelude::*;

use crate::game_result::GameResult;

pub(crate) fn build_game_result_preflight(
    game_result: &GameResult,
) -> ExternResult<PreflightRequest> {
    let game_result_hash = hash_entry(game_result.clone())?;

    let times = session_times_from_millis(5_000)?;

    let agent_info = agent_info()?;

    let header_base = HeaderBase::Create(CreateBase::new(GameResult::entry_type()?));

    let opponent_address = game_result.opponent()?;

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
