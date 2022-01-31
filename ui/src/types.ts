import {
  AgentPubKeyB64,
  Dictionary,
  EntryHashB64,
  HeaderHashB64,
} from '@holochain-open-dev/core-types';

export interface EloUpdate {
  player_address: AgentPubKeyB64;
  current_elo: number;
  // Will be None in the first GameResult entry for that player
  previous_game_result: HeaderHashB64 | undefined;
}

export interface GameResult {
  player_a: EloUpdate;
  player_b: EloUpdate;
  score_player_a: number;
  game_info: any;
}

export type CreateGameResultOutcome =
  | { type: 'Published'; game_result_hash: EntryHashB64 }
  | { type: 'OutdatedLastGameResult' };


  export type EloRanking = Dictionary<Array<AgentPubKeyB64>> 