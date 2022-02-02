import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { HoloHashed } from '@holochain/client';
import { EloService } from '../elo-service';
import { GameResult } from '../types';
import { EloRankingStore } from './ranking-store';
export declare enum ShortResult {
    Win = 1,
    Loss = 0,
    Draw = 0.5
}
export declare class EloStore {
    #private;
    protected eloService: EloService;
    profilesStore: ProfilesStore;
    get myAgentPubKey(): string;
    elos: import("svelte/store").Readable<{
        [key: string]: number;
    }>;
    eloForAgent(agent: AgentPubKeyB64): import("svelte/store").Readable<number>;
    gameResults: import("svelte/store").Readable<{
        [key: string]: [HoloHashed<any>, GameResult][];
    }>;
    myElo: import("svelte/store").Readable<number>;
    myGameResults: import("svelte/store").Readable<[HoloHashed<any>, GameResult][]>;
    createEloRankingStore(chunkSize: number): EloRankingStore;
    constructor(eloService: EloService, profilesStore: ProfilesStore);
    /** Helpers for the types */
    getOpponent(gameResult: GameResult): AgentPubKeyB64;
    getMyResult(gameResult: GameResult): number;
    /** Backend actions */
    fetchMyGameResults(): Promise<void>;
    fetchMyElo(): Promise<void>;
    fetchGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<void>;
    fetchEloForAgents(agents: AgentPubKeyB64[]): Promise<void>;
    private handleNewGameResult;
}
