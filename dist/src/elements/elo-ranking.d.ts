import { LitElement } from 'lit';
import { List, ListItem } from '@scoped-elements/material-web';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';
import { EloStore } from '../elo-store';
declare const EloRanking_base: typeof LitElement & import("@open-wc/dedupe-mixin").Constructor<import("@open-wc/scoped-elements/types/src/types").ScopedElementsHost>;
export declare class EloRanking extends EloRanking_base {
    _eloStore: EloStore;
    _profilesStore: ProfilesStore;
    _loading: boolean;
    _allProfiles: StoreSubscriber<import("@holochain-open-dev/core-types").Dictionary<import("@holochain-open-dev/profiles").Profile>>;
    _eloRanking: StoreSubscriber<{
        agentPubKey: string;
        elo: number;
    }[]>;
    firstUpdated(): Promise<void>;
    renderPlayer(agentPubKey: AgentPubKeyB64, elo: number): import("lit-html").TemplateResult<1>;
    renderSkeleton(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1>;
    static get scopedElements(): {
        'sl-skeleton': typeof SlSkeleton;
        'mwc-list': typeof List;
        'mwc-list-item': typeof ListItem;
    };
    static get styles(): import("lit").CSSResult;
}
export {};
