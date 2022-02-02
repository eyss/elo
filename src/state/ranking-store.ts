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
    const fromElo = this.newFromElo();

    const nextChunk = await this.eloService.getEloRankingChunk(
      fromElo,
      this.chunkSize
    );
    let thereAreMoreChunksToFetch = Object.keys(nextChunk).length !== 0;

    const pubKeysToFetch: AgentPubKeyB64[] = [];

    const chunk: EloRanking = {};

    for (const [ranking, agents] of Object.entries(nextChunk)) {
      for (const agent of agents) {
        if (pubKeysToFetch.length < this.chunkSize) {
          pubKeysToFetch.push(agent);
          if (!chunk[ranking]) chunk[ranking] = [];
          chunk[ranking].push(agent);
        } else {
          thereAreMoreChunksToFetch = true;
        }
      }
    }

    await this.profilesStore.fetchAgentsProfiles(pubKeysToFetch);

    this.#store.update(({ ranking }) => ({
      ranking: {
        ...ranking,
        ...chunk,
      },
      thereAreMoreChunksToFetch,
    }));
  }

  private newFromElo(): number | undefined {
    const ranking = get(this.#store).ranking;

    const elos = Object.keys(ranking).map(parseInt);
    if (elos.length === 0) return undefined;

    return Math.min(...elos) - 1;
  }
}
