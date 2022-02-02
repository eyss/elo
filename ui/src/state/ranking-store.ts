import { ProfilesStore } from '@holochain-open-dev/profiles';
import {
  get,
  Readable,
  readable,
  Subscriber,
  Unsubscriber,
  Writable,
  writable,
} from 'svelte/store';
import flatten from 'lodash-es/flatten';

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

    const allPubKeys = flatten(Object.values(nextChunk));

    await this.profilesStore.fetchAgentsProfiles(allPubKeys);

    const thereAreMoreChunksToFetch =
      allPubKeys.length !== 0 && allPubKeys.length >= this.chunkSize;

    this.#store.update(({ ranking }) => ({
      ranking: {
        ...ranking,
        ...nextChunk,
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
