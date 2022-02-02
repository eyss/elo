import { __decorate } from "tslib";
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';
import { ref } from 'lit/directives/ref.js';
import { when } from 'lit/directives/when.js';
import { Card, CircularProgress, List, ListItem, } from '@scoped-elements/material-web';
import { contextProvided } from '@holochain-open-dev/context';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';
import { eloStoreContext } from '../context';
import { sharedStyles } from '../shared-styles';
export class EloRankingElement extends ScopedElementsMixin(LitElement) {
    constructor() {
        super(...arguments);
        this._loading = true;
        this._allProfiles = new StoreSubscriber(this, () => this._eloStore.profilesStore.knownProfiles);
        this._eloRanking = new StoreSubscriber(this, () => this._rankingStore);
    }
    async firstUpdated() {
        this._rankingStore = this._eloStore.createEloRankingStore(1);
        await this._rankingStore.fetchNextChunk();
        this._loading = false;
    }
    respondToVisibility(element, callback) {
        var _a;
        const options = {
            root: (_a = this.shadowRoot) === null || _a === void 0 ? void 0 : _a.firstElementChild,
        };
        const observer = new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                callback(entry.intersectionRatio > 0);
            });
        }, options);
        observer.observe(element);
    }
    renderPlayer(agentPubKey, elo) {
        const profile = this._allProfiles.value[agentPubKey];
        return html `
      <mwc-list-item
        graphic="avatar"
        hasMeta
        .value=${agentPubKey}
        style="--mdc-list-item-graphic-size: 32px;"
      >
        <agent-avatar slot="graphic" .agentPubKey=${agentPubKey}>
        </agent-avatar>
        <span>${profile ? profile.nickname : agentPubKey}</span>
        <span slot="meta" style="color: black; font-size: 16px;">${elo}</span>
      </mwc-list-item>
    `;
    }
    renderSkeleton() {
        return html ` <div class="column" style="margin-top: 8px; margin-left: 4px;">
      ${[0, 1, 2].map(() => html `
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
        `)}
    </div>`;
    }
    renderRanking() {
        var _a;
        const rankingEntries = Object.entries((_a = this._eloRanking.value) === null || _a === void 0 ? void 0 : _a.ranking);
        if (rankingEntries.length === 0)
            return html ``;
        return html `
      <div class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <mwc-list noninteractive style="margin-right: 8px;">
              ${rankingEntries.map(([eloRanking, agentsPubKeys]) => agentsPubKeys.map(agentPubKey => this.renderPlayer(agentPubKey, parseInt(eloRanking))))}
            </mwc-list>

            ${when(this._eloRanking.value.thereAreMoreChunksToFetch, () => html `
                <div
                  class="row"
                  style="align-items: center; justify-content: center"
                >
                  <mwc-circular-progress
                    indeterminate
                    ${ref(el => el &&
            this.respondToVisibility(el, visible => visible && this._rankingStore.fetchNextChunk()))}
                  ></mwc-circular-progress>
                </div>
              `)}
          </div>
        </div>
      </div>
    `;
    }
    render() {
        return html `
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
__decorate([
    contextProvided({ context: eloStoreContext })
], EloRankingElement.prototype, "_eloStore", void 0);
__decorate([
    state()
], EloRankingElement.prototype, "_loading", void 0);
__decorate([
    property({ type: Object })
], EloRankingElement.prototype, "_rankingStore", void 0);
//# sourceMappingURL=elo-ranking.js.map