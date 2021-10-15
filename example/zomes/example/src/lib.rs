use hc_elo::*;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct GameInfo2 {
    opponent: AgentPubKeyB64,
}

pub struct ChessEloRating;

impl EloRatingSystem for ChessEloRating {
    type GameInfo = GameInfo2;

    fn validate_game_result(
        _game: GameInfo2,
        _result: GameResultInfo,
    ) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }
}

entry_defs![GameResult::entry_def()];

mixin_elo!(ChessEloRating);

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_elo()?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn publish_result(result: (AgentPubKeyB64, f32)) -> ExternResult<()> {
    attempt_create_countersigned_game_result::<ChessEloRating>(
        GameInfo2 {
            opponent: result.0.clone(),
        },
        result.0,
        result.1,
    )?;

    Ok(())
}
