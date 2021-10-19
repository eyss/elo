import { __decorate } from "tslib";
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { state } from 'lit/decorators.js';
import { List, ListItem } from '@scoped-elements/material-web';
import { contextProvided } from '@lit-labs/context';
import { profilesStoreContext, } from '@holochain-open-dev/profiles';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';
import { eloStoreContext } from '../context';
import { sharedStyles } from '../shared-styles';
export class EloRanking extends ScopedElementsMixin(LitElement) {
    constructor() {
        super(...arguments);
        this._loading = true;
        this._allProfiles = new StoreSubscriber(this, () => this._profilesStore.knownProfiles);
        this._eloRanking = new StoreSubscriber(this, () => this._eloStore.eloRanking);
    }
    async firstUpdated() {
        await this._profilesStore.fetchAllProfiles();
        const allPubKeys = Object.keys(this._allProfiles.value);
        await this._eloStore.fetchElosForAgents(allPubKeys);
        this._loading = false;
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
        <span style="margin-left: 8px;"
          >${profile ? profile.nickname : agentPubKey}</span
        >
        <span slot="meta">${elo}</span>
      </mwc-list-item>
    `;
    }
    renderSkeleton() {
        return html ` <div class="column">
      ${[0, 1, 2].map(_ => html `
          <div class="row">
            <sl-skeleton effect="sheen"></sl-skeleton>
            <sl-skeleton effect="sheen"></sl-skeleton>
          </div>
        `)}
    </div>`;
    }
    render() {
        if (this._loading)
            return this.renderSkeleton();
        return html `
      <mwc-list>
        ${this._eloRanking.value.map(e => this.renderPlayer(e.agentPubKey, e.elo))}
      </mwc-list>
    `;
    }
    static get scopedElements() {
        return {
            'sl-skeleton': SlSkeleton,
            'mwc-list': List,
            'mwc-list-item': ListItem,
        };
    }
    static get styles() {
        return sharedStyles;
    }
}
__decorate([
    contextProvided({ context: eloStoreContext })
], EloRanking.prototype, "_eloStore", void 0);
__decorate([
    contextProvided({ context: profilesStoreContext })
], EloRanking.prototype, "_profilesStore", void 0);
__decorate([
    state()
], EloRanking.prototype, "_loading", void 0);
//# sourceMappingURL=elo-ranking.js.map