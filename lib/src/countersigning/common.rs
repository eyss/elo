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

pub(crate) fn create_game_result(
    game_result: GameResult,
    responses: Vec<PreflightResponse>,
) -> ExternResult<HeaderHash> {
    HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            (&game_result).into(),
            Entry::CounterSign(
                Box::new(
                    CounterSigningSessionData::try_from_responses(responses).map_err(
                        |countersigning_error| WasmError::Guest(countersigning_error.to_string()),
                    )?,
                ),
                game_result.try_into()?,
            ),
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })
}
