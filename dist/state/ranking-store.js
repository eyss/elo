var _EloRankingStore_store;
import { __classPrivateFieldGet } from "tslib";
import { get, writable, } from 'svelte/store';
import flatten from 'lodash-es/flatten';
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
        const fromElo = this.newFromElo();
        const nextChunk = await this.eloService.getEloRankingChunk(fromElo, this.chunkSize);
        const allPubKeys = flatten(Object.values(nextChunk));
        await this.profilesStore.fetchAgentsProfiles(allPubKeys);
        const thereAreMoreChunksToFetch = allPubKeys.length !== 0 && allPubKeys.length >= this.chunkSize;
        __classPrivateFieldGet(this, _EloRankingStore_store, "f").update(({ ranking }) => ({
            ranking: {
                ...ranking,
                ...nextChunk,
            },
            thereAreMoreChunksToFetch,
        }));
    }
    newFromElo() {
        const ranking = get(__classPrivateFieldGet(this, _EloRankingStore_store, "f")).ranking;
        const elos = Object.keys(ranking).map(parseInt);
        if (elos.length === 0)
            return undefined;
        return Math.min(...elos) - 1;
    }
}
_EloRankingStore_store = new WeakMap();
//# sourceMappingURL=ranking-store.js.map