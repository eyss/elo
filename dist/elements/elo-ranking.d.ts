import { LitElement } from 'lit';
import { Card, CircularProgress, List, ListItem } from '@scoped-elements/material-web';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { EloStore } from '../state/elo-store';
declare const EloRankingElement_base: typeof LitElement & import("@open-wc/dedupe-mixin").Constructor<import("@open-wc/scoped-elements/types/src/types").ScopedElementsHost>;
export declare class EloRankingElement extends EloRankingElement_base {
    _eloStore: EloStore;
    _loading: boolean;
    private _allProfiles;
    private _rankingStore;
    private _eloRanking;
    firstUpdated(): Promise<void>;
    respondToVisibility(element: HTMLElement, callback: (visible: boolean) => void): void;
    renderPlayer(agentPubKey: AgentPubKeyB64, elo: number): import("lit-html").TemplateResult<1>;
    renderSkeleton(): import("lit-html").TemplateResult<1>;
    renderRanking(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1>;
    static get scopedElements(): {
        'sl-skeleton': typeof SlSkeleton;
        'agent-avatar': typeof AgentAvatar;
        'mwc-card': typeof Card;
        'mwc-list': typeof List;
        'mwc-list-item': typeof ListItem;
        'mwc-circular-progress': typeof CircularProgress;
    };
    static get styles(): import("lit").CSSResult[];
}
export {};
