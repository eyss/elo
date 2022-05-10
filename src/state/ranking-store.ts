import { ProfilesStore } from '@holochain-open-dev/profiles';
import {
  get,
  Readable,
  Subscriber,
  Unsubscriber,
  Writable,
  writable,
} from 'svelte/store';
import flatten from 'lodash-es/flatten';

import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { EloService } from '../elo-service';
import { EloRanking } from '../types';

export interface ChunkedEloRanking {
  ranking: EloRanking;
  thereAreMoreChunksToFetch: boolean;
}

export class EloRankingStore implements Readable<ChunkedEloRanking> {
  #store: Writable<ChunkedEloRanking> = writable({
    ranking: {},
    thereAreMoreChunksToFetch: true,
  });

  constructor(
    protected eloService: EloService,
    protected profilesStore: ProfilesStore,
    protected chunkSize: number
  ) {}

  subscribe(subscriber: Subscriber<ChunkedEloRanking>): Unsubscriber {
    return this.#store.subscribe(subscriber);
  }

  async fetchNextChunk() {
    const existingRanking = get(this.#store).ranking;

    const fromElo = this.newFromElo(existingRanking);

    // This is needed to handle the case in which we already have some agents for a
    // certain ELO and we want to fetch N more than those
    const chunkSize =
      this.chunkSize +
      (fromElo && existingRanking[fromElo]
        ? existingRanking[fromElo].length
        : 0);

    const nextChunk = await this.eloService.getEloRankingChunk(
      fromElo,
      chunkSize
    );
    let thereAreMoreChunksToFetch = false;

    const pubKeysToFetch: AgentPubKeyB64[] = [];

    for (const [ranking, agents] of Object.entries(nextChunk)) {
      if (!existingRanking[ranking]) existingRanking[ranking] = [];
      for (const agent of agents) {
        if (!existingRanking[ranking].includes(agent)) {
          if (pubKeysToFetch.length < this.chunkSize) {
            pubKeysToFetch.push(agent);
            existingRanking[ranking].push(agent);
          } else {
            thereAreMoreChunksToFetch = true;
          }
        }
      }
    }

    await this.profilesStore.fetchAgentsProfiles(pubKeysToFetch);

    this.#store.set({
      ranking: existingRanking,
      thereAreMoreChunksToFetch,
    });
  }

  private newFromElo(ranking: EloRanking): number | undefined {
    const elos = Object.keys(ranking).map(key => {return parseInt(key)});
    if (elos.length === 0) return undefined;

    return Math.min(...elos);
  }
}
