var _EloRankingStore_store;
import { __classPrivateFieldGet } from "tslib";
import { get, writable, } from 'svelte/store';
export class EloRankingStore {
    constructor(eloService, profilesStore, chunkSize) {
        this.eloService = eloService;
        this.profilesStore = profilesStore;
        this.chunkSize = chunkSize;
        _EloRankingStore_store.set(this, writable({
            ranking: {},
            thereAreMoreChunksToFetch: true,
        }));
    }
    subscribe(subscriber) {
        return __classPrivateFieldGet(this, _EloRankingStore_store, "f").subscribe(subscriber);
    }
    async fetchNextChunk() {
        const existingRanking = get(__classPrivateFieldGet(this, _EloRankingStore_store, "f")).ranking;
        const fromElo = this.newFromElo(existingRanking);
        const nextChunk = await this.eloService.getEloRankingChunk(fromElo, this.chunkSize);
        let thereAreMoreChunksToFetch = Object.keys(nextChunk).length !== 0;
        const pubKeysToFetch = [];
        for (const [ranking, agents] of Object.entries(nextChunk)) {
            for (const agent of agents) {
                if (pubKeysToFetch.length < this.chunkSize) {
                    pubKeysToFetch.push(agent);
                    if (!existingRanking[ranking])
                        existingRanking[ranking] = [];
                    existingRanking[ranking].push(agent);
                }
                else {
                    thereAreMoreChunksToFetch = true;
                }
            }
        }
        await this.profilesStore.fetchAgentsProfiles(pubKeysToFetch);
        __classPrivateFieldGet(this, _EloRankingStore_store, "f").set({
            ranking: existingRanking,
            thereAreMoreChunksToFetch,
        });
    }
    newFromElo(ranking) {
        const elos = Object.keys(ranking).map(parseInt);
        if (elos.length === 0)
            return undefined;
        return Math.min(...elos) - 1;
    }
}
_EloRankingStore_store = new WeakMap();
//# sourceMappingURL=ranking-store.js.map