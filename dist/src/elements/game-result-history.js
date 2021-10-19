import { __decorate } from "tslib";
import { html, LitElement } from 'lit';
import { styleMap } from 'lit/directives/style-map.js';
import { contextProvided } from '@lit-labs/context';
import { profilesStoreContext, } from '@holochain-open-dev/profiles';
import { Card, List, ListItem, Icon, CircularProgress, } from '@scoped-elements/material-web';
import { StoreSubscriber } from 'lit-svelte-stores';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { ShortResult } from '../elo-store';
import { eloStoreContext } from '../context';
import { headerTimestamp } from '../utils';
import { sharedStyles } from '../shared-styles';
export class ChessGameResultsHistory extends ScopedElementsMixin(LitElement) {
    constructor() {
        super(...arguments);
        this._knownProfiles = new StoreSubscriber(this, () => this._profilesStore.knownProfiles);
        this._myGameResults = new StoreSubscriber(this, () => this._eloStore.myGameResults);
    }
    async firstUpdated() {
        await this._eloStore.fetchMyGameResults();
        const promises = this._myGameResults.value.map(async (r) => this._profilesStore.fetchAgentProfile(this._eloStore.getOpponent(r[1])));
        await Promise.all(promises);
    }
    getIcon(result) {
        if (this._eloStore.getMyResult(result) === ShortResult.Draw)
            return 'drag_handle';
        if (this._eloStore.getMyResult(result) === ShortResult.Win)
            return 'thumb_up';
        if (this._eloStore.getMyResult(result) === ShortResult.Loss)
            return 'thumb_down';
    }
    getColor(result) {
        if (this._eloStore.getMyResult(result) === ShortResult.Draw)
            return 'grey';
        if (this._eloStore.getMyResult(result) === ShortResult.Win)
            return 'green';
        return 'red';
    }
    getSummary() {
        let summary = {
            [ShortResult.Draw]: 0,
            [ShortResult.Loss]: 0,
            [ShortResult.Win]: 0,
        };
        for (const result of this._myGameResults.value) {
            summary[this._eloStore.getMyResult(result[1])]++;
        }
        return summary;
    }
    renderResults() {
        if (this._myGameResults.value.length === 0)
            return html `<div class="column center-content" style="flex: 1;">
        <span class="placeholder">There are no games in your history yet</span>
      </div>`;
        return html `<div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <mwc-list disabled>
            ${this._myGameResults.value.map(result => html `<mwc-list-item twoline graphic="icon">
                  <span
                    >vs
                    ${this._knownProfiles.value[this._eloStore.getOpponent(result[1])].nickname}
                  </span>
                  <span slot="secondary"
                    >${new Date(headerTimestamp(result[0])).toLocaleString()}</span
                  >
                  <mwc-icon
                    slot="graphic"
                    style=${styleMap({
            color: this.getColor(result[1]),
        })}
                    >${this.getIcon(result[1])}</mwc-icon
                  >
                </mwc-list-item>`)}
          </mwc-list>
        </div>
      </div>
    </div>`;
    }
    render() {
        if (!this._myGameResults.value)
            return html `<div class="column center-content" style="flex: 1;">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`;
        const summary = this.getSummary();
        return html `
      <mwc-card style="flex: 1; min-width: 270px;">
        <div class="column" style="margin: 16px; flex: 1;">
          <span class="title">Game History</span>
          ${this.renderResults()}
          <div class="row center-content">
            <span class="placeholder"
              >Summary: ${summary[ShortResult.Win]}
              ${summary[ShortResult.Win] === 1 ? 'win' : 'wins'},
              ${summary[ShortResult.Loss]}
              ${summary[ShortResult.Loss] === 1 ? 'loss' : 'losses'},
              ${summary[ShortResult.Draw]}
              ${summary[ShortResult.Draw] === 1 ? 'draw' : 'draws'}</span
            >
          </div>
        </div>
      </mwc-card>
    `;
    }
    static get scopedElements() {
        return {
            'mwc-icon': Icon,
            'mwc-card': Card,
            'mwc-list': List,
            'mwc-list-item': ListItem,
            'mwc-circular-progress': CircularProgress,
        };
    }
}
ChessGameResultsHistory.styles = [sharedStyles];
__decorate([
    contextProvided({ context: eloStoreContext })
], ChessGameResultsHistory.prototype, "_eloStore", void 0);
__decorate([
    contextProvided({ context: profilesStoreContext })
], ChessGameResultsHistory.prototype, "_profilesStore", void 0);
//# sourceMappingURL=game-result-history.js.map