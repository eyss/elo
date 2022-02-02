import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';
import { ref } from 'lit/directives/ref.js';
import { when } from 'lit/directives/when.js';
import {
  Card,
  CircularProgress,
  List,
  ListItem,
} from '@scoped-elements/material-web';
import { contextProvided } from '@holochain-open-dev/context';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';

import { eloStoreContext } from '../context';
import { EloStore } from '../state/elo-store';
import { sharedStyles } from '../shared-styles';
import { EloRankingStore } from '../state/ranking-store';

export class EloRankingElement extends ScopedElementsMixin(LitElement) {
  @contextProvided({ context: eloStoreContext })
  _eloStore!: EloStore;

  @state()
  _loading = true;

  private _allProfiles = new StoreSubscriber(
    this,
    () => this._eloStore.profilesStore.knownProfiles
  );

  @property({ type: Object })
  private _rankingStore!: EloRankingStore;

  private _eloRanking = new StoreSubscriber(this, () => this._rankingStore);

  async firstUpdated() {
    this._rankingStore = this._eloStore.createEloRankingStore(6);
    await this._rankingStore.fetchNextChunk();

    this._loading = false;
  }

  respondToVisibility(
    element: HTMLElement,
    callback: (visible: boolean) => void
  ) {
    const options = {
      root: this.shadowRoot?.firstElementChild,
    };

    const observer = new IntersectionObserver((entries, observer) => {
      entries.forEach(entry => {
        callback(entry.intersectionRatio > 0);
      });
    }, options);

    observer.observe(element);
  }

  renderPlayer(agentPubKey: AgentPubKeyB64, elo: number) {
    const profile = this._allProfiles.value[agentPubKey];

    if (!profile) return html``;

    return html`
      <mwc-list-item
        graphic="avatar"
        hasMeta
        .value=${agentPubKey}
        style="--mdc-list-item-graphic-size: 32px;"
      >
        <agent-avatar slot="graphic" .agentPubKey=${agentPubKey}>
        </agent-avatar>
        <span>${profile.nickname}</span>
        <span slot="meta" style="color: black; font-size: 16px;">${elo}</span>
      </mwc-list-item>
    `;
  }

  renderSkeleton() {
    return html` <div class="column" style="margin-top: 8px; margin-left: 4px;">
      ${[0, 1, 2].map(
        () => html`
          <div class="row" style="align-items: center; margin: 8px;">
            <sl-skeleton
              effect="sheen"
              style="width: 32px; height: 32px; margin-right: 16px;"
            ></sl-skeleton>

            <sl-skeleton
              effect="sheen"
              style="width: 200px; height: 16px;"
            ></sl-skeleton>
          </div>
        `
      )}
    </div>`;
  }

  renderRanking() {
    const rankingEntries = Object.entries(this._eloRanking.value?.ranking).sort(
      ([r1, _], [r2, __]) => parseInt(r2) - parseInt(r1)
    );
    if (rankingEntries.length === 0) return html``;

    return html`
      <div class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <mwc-list noninteractive style="margin-right: 8px;">
              ${rankingEntries.map(([eloRanking, agentsPubKeys]) =>
                agentsPubKeys.map(agentPubKey =>
                  this.renderPlayer(agentPubKey, parseInt(eloRanking))
                )
              )}
            </mwc-list>

            ${when(
              this._eloRanking.value.thereAreMoreChunksToFetch,
              () => html`
                <div
                  class="row"
                  style="align-items: center; justify-content: center"
                >
                  <mwc-circular-progress
                    indeterminate
                    ${ref(
                      el =>
                        el &&
                        this.respondToVisibility(
                          el as HTMLElement,
                          visible =>
                            visible && this._rankingStore.fetchNextChunk()
                        )
                    )}
                  ></mwc-circular-progress>
                </div>
              `
            )}
          </div>
        </div>
      </div>
    `;
  }

  render() {
    return html`
      <mwc-card style="flex: 1; min-width: 270px;">
        <div class="column" style="margin: 16px; flex: 1;">
          <span class="title">ELO Ranking</span>
          ${this._loading ? this.renderSkeleton() : this.renderRanking()}
        </div>
      </mwc-card>
    `;
  }

  static get scopedElements() {
    return {
      'sl-skeleton': SlSkeleton,
      'agent-avatar': AgentAvatar,
      'mwc-card': Card,
      'mwc-list': List,
      'mwc-list-item': ListItem,
      'mwc-circular-progress': CircularProgress,
    };
  }

  static get styles() {
    return [sharedStyles];
  }
}
