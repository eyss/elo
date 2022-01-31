import { AgentPubKeyB64, EntryHashB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/client';

import { EloRanking, GameResult } from './types';

export class EloService {
  constructor(public cellClient: CellClient, protected zomeName: string) {}

  public getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: string]: Array<[HoloHashed<any>, GameResult]>;
  }> {
    return this.callZome('get_game_results_for_agents', agents);
  }

  public getEloRatingForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: string]: number;
  }> {
    return this.callZome('get_elo_rating_for_agents', agents);
  }

  public getEloRankingChunk(
    fromElo: number | undefined,
    agentCount: number
  ): Promise<EloRanking> {
    return this.callZome('get_elo_ranking_chunk', {
      fromElo,
      agentCount,
    });
  }

  // TODO: remove when schedule lands
  public resolveFlags(): Promise<void> {
    return this.callZome(
      'scheduled_try_resolve_unpublished_game_results',
      null
    );
  }

  private callZome(fnName: string, payload: any): Promise<any> {
    return this.cellClient.callZome(this.zomeName, fnName, payload);
  }
}
