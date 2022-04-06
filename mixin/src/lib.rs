mod countersigning;
mod elo_ranking;
mod elo_rating;
mod elo_rating_system;
mod game_result;
mod mixin;

pub use crate::countersigning::{
    receiver::handle_request_publish_game_result, sender::send_publish_game_result_request,
};
pub use crate::elo_rating::{get_elo_rating_for_agents, EloRating, DRAW, LOSS};
pub use crate::elo_rating_system::*;
pub use crate::game_result::{
    handlers::{
        element_to_game_result, game_results_tag, get_game_results_for_agents,
        index_game_result_if_not_exists,
    },
    unpublished::try_resolve_unpublished_game_results,
    //validation::validate_entry_game_result,
    GameResult, GameResultInfo,
};
pub use elo_ranking::*;
pub use mixin::{
    attempt_create_countersigned_game_result, create_game_result_and_flag, init_elo,
    post_commit_elo,
};
