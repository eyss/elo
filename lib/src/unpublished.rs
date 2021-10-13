use std::collections::BTreeMap;

use crate::countersigning::sender::send_publish_game_result_request;
use crate::elo_rating_system::EloRatingSystem;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_entry(id = "unpublished_game", visibility = "private")]
pub struct UnpublishedGameResult {
    pub game_info: SerializedBytes,
    pub opponent_address: AgentPubKeyB64,
    pub my_score: f32,
}

pub fn scheduled_try_resolve_unpublished_game_results<S: EloRatingSystem>() -> ExternResult<()> {
    let unpublished_game_results = get_unpublished_game_results()?;

    for (header_hash, unpublished_game_result) in unpublished_game_results {
        let attempt_result = send_publish_game_result_request::<S>(
            unpublished_game_result.game_info,
            unpublished_game_result.opponent_address,
            unpublished_game_result.my_score,
        );

        if let Ok(_) = attempt_result {
            // We have succeeded in creating the game result entry: delete the unpublished private one
            delete_entry(header_hash)?;
        }
    }

    Ok(())
}

fn get_unpublished_game_results() -> ExternResult<BTreeMap<HeaderHash, UnpublishedGameResult>> {
    let filter = QueryFilter::new()
        .include_entries(true)
        .entry_type(EntryType::App(AppEntryType::new(
            EntryDefIndex::from(1),
            zome_info()?.zome_id,
            EntryVisibility::Private,
        )));

    let elements = query(filter)?;

    let mut unpublished_game_results: BTreeMap<HeaderHash, UnpublishedGameResult> = BTreeMap::new();

    for element in elements {
        let unpublished_game_result: UnpublishedGameResult = element
            .entry()
            .to_app_option()?
            .ok_or(WasmError::Guest("Invalid SourceChain element".into()))?;

        unpublished_game_results.insert(element.header_address().clone(), unpublished_game_result);
    }

    // Remove all the unpublished game entries that have been deleted, because they have already succeeded

    let delete_headers_filter = QueryFilter::new().header_type(HeaderType::Delete);

    let delete_elements = query(delete_headers_filter)?;

    for element in delete_elements {
        if let Header::Delete(Delete {
            deletes_address, ..
        }) = element.header()
        {
            unpublished_game_results.remove(deletes_address);
        }
    }

    Ok(unpublished_game_results)
}
