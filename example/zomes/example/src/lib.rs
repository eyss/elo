use hc_elo::*;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct GameInfo2 {
    game_hash: EntryHashB64,
    last_move_hash: EntryHashB64,
}

pub struct ChessEloRating;

impl EloRatingSystem for ChessEloRating {
    type GameInfo = GameInfo2;

    fn validate_game_result(
        game: GameInfo2,
        result: GameResultInfo,
    ) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }
}

mixin_elo!(ChessEloRating);