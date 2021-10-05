use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::build_new_game_result;

/**
 * Build a new GameResult for the finished game, and call_remote to the opponent with a countersigning request
 */
pub fn send_publish_game_result_request(
    opponent_address: AgentPubKeyB64,
    my_score: f32,
) -> ExternResult<()> {
    let new_game_result = build_new_game_result(&opponent_address, my_score)?;

    let new_game_result = hash_entry(new_game_result)?;

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

    let preflight = PreflightRequest::try_new(
        new_game_result,
        countersigning_agents,
        Some(0),
        times,
        header_base,
        preflight_bytes,
    )
    .map_err(|err| WasmError::Guest(format!("Could not create preflight request: {:?}", err)))?;

    call_remote(
        AgentPubKey::from(opponent_address),
        zome_info.zome_name,
        FunctionName("request_publish_game_result".into()),
        None,
        preflight,
    )?;

    Ok(())
}

pub fn init() -> ExternResult<()> {
    // TODO: restrict to only the agents with which we are playing
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "request_publish_game_result".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(())
}

pub fn accept_game_publish_result_request(preflight_request: PreflightRequest) -> ExternResult<()> {
    match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(_) => Ok(()),
        _ => Err(WasmError::Guest(
            "There was an error accepting the publishing of game result".into(),
        )),
    }
}
