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
        // This is needed to handle the case in which we already have some agents for a
        // certain ELO and we want to fetch N more than those
        const chunkSize = this.chunkSize +
            (fromElo && existingRanking[fromElo]
                ? existingRanking[fromElo].length
                : 0);
        const nextChunk = await this.eloService.getEloRankingChunk(fromElo, chunkSize);
        let thereAreMoreChunksToFetch = false;
        const pubKeysToFetch = [];
        for (const [ranking, agents] of Object.entries(nextChunk)) {
            if (!existingRanking[ranking])
                existingRanking[ranking] = [];
            for (const agent of agents) {
                if (!existingRanking[ranking].includes(agent)) {
                    if (pubKeysToFetch.length < this.chunkSize) {
                        pubKeysToFetch.push(agent);
                        existingRanking[ranking].push(agent);
                    }
                    else {
                        thereAreMoreChunksToFetch = true;
                    }
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
        const elos = Object.keys(ranking).map(key => parseInt(key));
        if (elos.length === 0)
            return undefined;
        return Math.min(...elos);
    }
}
_EloRankingStore_store = new WeakMap();
//# sourceMappingURL=ranking-store.js.map