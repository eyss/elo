import { Context, createContext } from '@holochain-open-dev/context';
import { EloStore } from './state/elo-store';

export const eloStoreContext: Context<EloStore> =
  createContext('hc_mixin_elo/store');
