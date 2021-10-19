import { LitElement } from 'lit';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { Card, List, ListItem, Icon, CircularProgress } from '@scoped-elements/material-web';
import { StoreSubscriber } from 'lit-svelte-stores';
import { EloStore } from '../elo-store';
import { GameResult } from '../types';
declare const ChessGameResultsHistory_base: typeof LitElement & import("@open-wc/dedupe-mixin").Constructor<import("@open-wc/scoped-elements/types/src/types").ScopedElementsHost>;
export declare class ChessGameResultsHistory extends ChessGameResultsHistory_base {
    _eloStore: EloStore;
    _profilesStore: ProfilesStore;
    _knownProfiles: StoreSubscriber<import("@holochain-open-dev/core-types").Dictionary<import("@holochain-open-dev/profiles").Profile>>;
    _myGameResults: StoreSubscriber<[import("@holochain/conductor-api").HoloHashed<any>, GameResult][]>;
    firstUpdated(): Promise<void>;
    getIcon(result: GameResult): "drag_handle" | "thumb_up" | "thumb_down" | undefined;
    getColor(result: GameResult): "grey" | "green" | "red";
    getSummary(): {
        0.5: number;
        0: number;
        1: number;
    };
    renderResults(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1>;
    static get scopedElements(): {
        'mwc-icon': typeof Icon;
        'mwc-card': typeof Card;
        'mwc-list': typeof List;
        'mwc-list-item': typeof ListItem;
        'mwc-circular-progress': typeof CircularProgress;
    };
    static styles: import("lit").CSSResult[];
}
export {};
