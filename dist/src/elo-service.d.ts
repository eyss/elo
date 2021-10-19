import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/conductor-api';
import { GameResult } from './types';
export declare class EloService {
    cellClient: CellClient;
    protected zomeName: string;
    constructor(cellClient: CellClient, zomeName: string);
    getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<{
        [key: AgentPubKeyB64]: Array<[HoloHashed<any>, GameResult]>;
    }>;
    getEloRatingsForAgents(agents: AgentPubKeyB64[]): Promise<{
        [key: AgentPubKeyB64]: number;
    }>;
    resolveFlags(): Promise<void>;
    private callZome;
}
