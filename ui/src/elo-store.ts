import { AgentPubKeyB64, serializeHash } from '@holochain-open-dev/core-types';
import { HoloHashed } from '@holochain/conductor-api';
import { derived, writable, Readable, Writable } from 'svelte/store';
import { EloService } from './elo-service';
import { GameResult } from './types';
import { headerTimestamp } from './utils';

export enum ShortResult {
  Win = 1.0,
  Loss = 0.0,
  Draw = 0.5,
}

export class EloStore {
  #gameResults: Writable<{
    [key: AgentPubKeyB64]: Array<[HoloHashed<any>, GameResult]>;
  }> = writable({});
  #elos: Writable<{
    [key: AgentPubKeyB64]: number;
  }> = writable({});

  public elos = derived(this.#elos, i => i);
  public eloRanking = derived(this.#elos, i =>
    Object.entries(i)
      .map(([agentPubKey, elo]) => ({ agentPubKey, elo }))
      .sort((a, b) => b.elo - a.elo)
  );

  public gameResults = derived(this.#gameResults, i => i);

  public myElo = derived(this.#elos, i => i[this.myAgentPubKey]);

  public myGameResults = derived(this.#gameResults, i =>
    i[this.myAgentPubKey].sort(
      (
        gr1,
        gr2 // TODO: fix this
      ) => headerTimestamp(gr2[0]) - headerTimestamp(gr1[0])
    )
  );

  public get myAgentPubKey() {
    return serializeHash(this.eloService.cellClient.cellId[1]);
  }

  constructor(protected eloService: EloService) {}

  /** Helpers for the types */

  getOpponent(gameResult: GameResult): AgentPubKeyB64 {
    if (gameResult.player_a.player_address === this.myAgentPubKey)
      return gameResult.player_b.player_address;
    else return gameResult.player_a.player_address;
  }

  getMyResult(gameResult: GameResult): number {
    if (gameResult.player_a.player_address) return gameResult.score_player_a;
    else return 1 - gameResult.score_player_a;
  }

  /** Backend actions */

  async fetchMyGameResults() {
    return this.fetchElosForAgents([this.myAgentPubKey]);
  }

  async fetchMyElo() {
    return this.fetchGameResultsForAgents([this.myAgentPubKey]);
  }

  async fetchGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<void> {
    const gameResults = await this.eloService.getGameResultsForAgents(agents);

    this.#gameResults.update(r => ({ ...r, ...gameResults }));
  }

  async fetchElosForAgents(agents: AgentPubKeyB64[]): Promise<void> {
    const elos = await this.eloService.getEloRatingsForAgents(agents);
    this.#elos.update(e => ({ ...e, ...elos }));
  }
}
