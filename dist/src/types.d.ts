import { AgentPubKeyB64, HeaderHashB64 } from '@holochain-open-dev/core-types';
export interface GameResult {
    player_a: EloUpdate;
    player_b: EloUpdate;
    score_player_a: number;
    game_info: any;
}
export interface EloUpdate {
    player_address: AgentPubKeyB64;
    current_elo: number;
    previous_game_result: HeaderHashB64 | undefined;
}
