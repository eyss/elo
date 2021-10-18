use crate::{elo_rating::EloRating, game_result::GameResultInfo};
use hdk::prelude::*;

pub trait EloRatingSystem {
    type GameInfo: TryFrom<SerializedBytes> + TryInto<SerializedBytes>;
    // Initial rating for a player who hasn't played any games
    fn initial_rating() -> EloRating {
        1000
    }

    // This is the maximum gain or loss of ELO that a match can affect
    fn k_factor() -> u32 {
        32
    }

    // How long we are going to wait until retrying to publish the already finished game results
    fn unpublished_games_retry_interval_in_mins() -> u32 {
        1
    }

    fn validate_game_result(
        game: Self::GameInfo,
        result: GameResultInfo,
    ) -> ExternResult<ValidateCallbackResult>;
}
