import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { HoloHashed } from '@holochain/conductor-api';
import { Readable } from 'svelte/store';
import { EloService } from './elo-service';
import { GameResult } from './types';
export declare enum ShortResult {
    Win = 1,
    Loss = 0,
    Draw = 0.5
}
export declare class EloStore {
    #private;
    protected eloService: EloService;
    elos: Readable<{
        [key: string]: number;
    }>;
    eloRanking: Readable<{
        agentPubKey: string;
        elo: number;
    }[]>;
    gameResults: Readable<{
        [key: string]: [HoloHashed<any>, GameResult][];
    }>;
    myElo: Readable<number>;
    myGameResults: Readable<[HoloHashed<any>, GameResult][]>;
    get myAgentPubKey(): string;
    constructor(eloService: EloService);
    /** Helpers for the types */
    getOpponent(gameResult: GameResult): AgentPubKeyB64;
    getMyResult(gameResult: GameResult): number;
    /** Backend actions */
    fetchMyGameResults(): Promise<void>;
    fetchMyElo(): Promise<void>;
    fetchGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<void>;
    fetchElosForAgents(agents: AgentPubKeyB64[]): Promise<void>;
}
