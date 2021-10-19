import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/conductor-api';

import { GameResult } from './types';

export class EloService {
  constructor(public cellClient: CellClient, protected zomeName: string) {}

  public getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: AgentPubKeyB64]: Array<[HoloHashed<any>, GameResult]>;
  }> {
    return this.callZome('get_elo_ratings_for_agents', agents);
  }

  public getEloRatingsForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: AgentPubKeyB64]: number;
  }> {
    return this.callZome('get_elo_ratings_for_agents', agents);
  }

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
