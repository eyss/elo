import { AgentPubKeyB64, Dictionary } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/client';
import { EloRanking, GameResult } from './types';
export declare class EloService {
    cellClient: CellClient;
    protected zomeName: string;
    constructor(cellClient: CellClient, zomeName: string);
    getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<Dictionary<Array<[HoloHashed<any>, GameResult]>>>;
    getEloRatingForAgents(agents: AgentPubKeyB64[]): Promise<Dictionary<number>>;
    getEloRankingChunk(fromElo: number | undefined, agentCount: number): Promise<EloRanking>;
    resolveFlags(): Promise<void>;
    private callZome;
}
