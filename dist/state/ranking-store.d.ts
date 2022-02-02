import { ProfilesStore } from '@holochain-open-dev/profiles';
import { Readable, Subscriber, Unsubscriber } from 'svelte/store';
import { EloService } from '../elo-service';
import { EloRanking } from '../types';
export interface ChunkedEloRanking {
    ranking: EloRanking;
    thereAreMoreChunksToFetch: boolean;
}
export declare class EloRankingStore implements Readable<ChunkedEloRanking> {
    #private;
    protected eloService: EloService;
    protected profilesStore: ProfilesStore;
    protected chunkSize: number;
    constructor(eloService: EloService, profilesStore: ProfilesStore, chunkSize: number);
    subscribe(subscriber: Subscriber<ChunkedEloRanking>): Unsubscriber;
    fetchNextChunk(): Promise<void>;
    private newFromElo;
}
