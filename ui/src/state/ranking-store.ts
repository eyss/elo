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

export class EloRankingStore implements Readable<EloRanking> {
  #store: Writable<EloRanking> = writable({});

  constructor(
    protected eloService: EloService,
    protected profilesStore: ProfilesStore,
    protected chunkSize: number
  ) {}

  subscribe(subscriber: Subscriber<EloRanking>): Unsubscriber {
    return this.#store.subscribe(subscriber);
  }

  async fetchNextChunk() {
    const fromElo = this.minRankingSeen();

    const nextChunk = await this.eloService.getEloRankingChunk(
      fromElo,
      this.chunkSize
    );

    const allPubKeys = flatten(Object.values(nextChunk));

    await this.profilesStore.fetchAgentsProfiles(allPubKeys);

    this.#store.update(ranking => ({
      ...ranking,
      ...nextChunk,
    }));
  }

  private minRankingSeen(): number | undefined {
    const ranking = get(this.#store);

    const elos = Object.keys(ranking).map(parseInt);
    if (elos.length === 0) return undefined;

    return Math.min(...elos);
  }
}
