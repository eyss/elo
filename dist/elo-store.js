var _EloStore_gameResults, _EloStore_elos;
import { __classPrivateFieldGet } from "tslib";
import { serializeHash } from '@holochain-open-dev/core-types';
import { derived, writable } from 'svelte/store';
import { headerTimestamp } from './utils';
export var ShortResult;
(function (ShortResult) {
    ShortResult[ShortResult["Win"] = 1] = "Win";
    ShortResult[ShortResult["Loss"] = 0] = "Loss";
    ShortResult[ShortResult["Draw"] = 0.5] = "Draw";
})(ShortResult || (ShortResult = {}));
export class EloStore {
    constructor(eloService, profilesStore) {
        this.eloService = eloService;
        this.profilesStore = profilesStore;
        _EloStore_gameResults.set(this, writable({}));
        _EloStore_elos.set(this, writable({}));
        this.elos = derived(__classPrivateFieldGet(this, _EloStore_elos, "f"), i => i);
        this.eloRanking = derived(__classPrivateFieldGet(this, _EloStore_elos, "f"), i => Object.entries(i)
            .map(([agentPubKey, elo]) => ({ agentPubKey, elo }))
            .sort((a, b) => b.elo - a.elo));
        this.gameResults = derived(__classPrivateFieldGet(this, _EloStore_gameResults, "f"), i => i);
        this.myElo = derived(__classPrivateFieldGet(this, _EloStore_elos, "f"), i => i[this.myAgentPubKey]);
        this.myGameResults = derived(__classPrivateFieldGet(this, _EloStore_gameResults, "f"), i => {
            const myResults = i[this.myAgentPubKey];
            if (!myResults)
                return [];
            return myResults.sort((gr1, gr2 // TODO: fix this
            ) => headerTimestamp(gr2[0]) - headerTimestamp(gr1[0]));
        });
        // TODO: remove when scheduler actually works
        setInterval(() => this.eloService.resolveFlags(), 5000);
        this.eloService.resolveFlags();
        this.eloService.cellClient.addSignalHandler(signal => {
            if (signal.data.payload.type === 'NewGameResult') {
                this.handleNewGameResult(signal.data.payload.game_result);
            }
        });
    }
    get myAgentPubKey() {
        return serializeHash(this.eloService.cellClient.cellId[1]);
    }
    /** Helpers for the types */
    getOpponent(gameResult) {
        if (gameResult.player_a.player_address === this.myAgentPubKey)
            return gameResult.player_b.player_address;
        return gameResult.player_a.player_address;
    }
    getMyResult(gameResult) {
        if (gameResult.player_a.player_address)
            return gameResult.score_player_a;
        return 1 - gameResult.score_player_a;
    }
    /** Backend actions */
    async fetchMyGameResults() {
        return this.fetchGameResultsForAgents([this.myAgentPubKey]);
    }
    async fetchMyElo() {
        return this.fetchEloForAgents([this.myAgentPubKey]);
    }
    async fetchGameResultsForAgents(agents) {
        const gameResults = await this.eloService.getGameResultsForAgents(agents);
        __classPrivateFieldGet(this, _EloStore_gameResults, "f").update(r => ({ ...r, ...gameResults }));
    }
    async fetchEloForAgents(agents) {
        const elos = await this.eloService.getEloRatingForAgents(agents);
        __classPrivateFieldGet(this, _EloStore_elos, "f").update(e => ({ ...e, ...elos }));
    }
    async handleNewGameResult(gameResult) {
        const players = [
            gameResult.player_a.player_address,
            gameResult.player_b.player_address,
        ];
        const promises = [
            this.fetchGameResultsForAgents(players),
            this.fetchEloForAgents(players),
            this.profilesStore.fetchAgentsProfiles(players),
        ];
        await Promise.all(promises);
    }
}
_EloStore_gameResults = new WeakMap(), _EloStore_elos = new WeakMap();
//# sourceMappingURL=elo-store.js.map