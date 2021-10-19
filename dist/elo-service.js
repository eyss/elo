export class EloService {
    constructor(cellClient, zomeName) {
        this.cellClient = cellClient;
        this.zomeName = zomeName;
    }
    getGameResultsForAgents(agents) {
        return this.callZome('get_game_results_for_agents', agents);
    }
    getEloRatingForAgents(agents) {
        return this.callZome('get_elo_rating_for_agents', agents);
    }
    resolveFlags() {
        return this.callZome('scheduled_try_resolve_unpublished_game_results', null);
    }
    callZome(fnName, payload) {
        return this.cellClient.callZome(this.zomeName, fnName, payload);
    }
}
//# sourceMappingURL=elo-service.js.map