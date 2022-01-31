use hc_lib_ranking_index::*;
use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;
use skill_rating::elo::EloRating;
use std::collections::BTreeMap;

pub type EloRanking = BTreeMap<usize, Vec<AgentPubKeyB64>>;

pub const ELO_RANKING_INDEX: RankingIndex = RankingIndex {
    name: "elo_ranking",
    index_interval: 200,
};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEloRankingChunkInput {
    pub from_elo: Option<usize>,
    pub agent_count: usize,
}

pub fn get_elo_ranking_chunk(
    from_elo: Option<usize>,
    agent_count: usize,
) -> ExternResult<EloRanking> {
    let cursor = from_elo.map(|elo| GetRankingCursor {
        from_ranking: elo as i64,
    });

    let next_chunk = ELO_RANKING_INDEX.get_entry_ranking_chunk(
        GetRankingDirection::Descendent,
        agent_count,
        cursor,
    )?;

    let mut elo_ranking: EloRanking = BTreeMap::new();

    for (ranking, entries_with_that_ranking) in next_chunk {
        let agent_pub_keys = entries_with_that_ranking
            .into_iter()
            .filter_map(|e| e.tag)
            .map(|tag| AgentPubKey::try_from(tag))
            .collect::<Result<Vec<AgentPubKey>, SerializedBytesError>>()?;

        let pub_keys_b64 = agent_pub_keys
            .into_iter()
            .map(|pub_key| AgentPubKeyB64::from(pub_key))
            .collect();

        elo_ranking.insert(ranking as usize, pub_keys_b64);
    }

    Ok(elo_ranking)
}

pub fn put_elo_rating_in_ranking(
    game_result_hash: EntryHash,
    agent_pub_key: AgentPubKey,
    previous_rating: Option<(EntryHash, EloRating)>,
    new_rating: EloRating,
) -> ExternResult<()> {
    if let Some((last_game_result_hash, previous_rating)) = previous_rating {
        ELO_RANKING_INDEX.delete_entry_ranking(last_game_result_hash, previous_rating as i64)?;
    }

    let tag = SerializedBytes::try_from(agent_pub_key)?;
    ELO_RANKING_INDEX.create_entry_ranking(game_result_hash, new_rating as i64, Some(tag))?;

    Ok(())
}
