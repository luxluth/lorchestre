import { getContext, setContext } from 'svelte';
import FilterQuery from './filterq.svelte';
import { FilterType, type Playlist } from './type';

export default class List {
	activeList = $state<string | null>(null);
	filters = new FilterQuery(FilterType.NoFilter);
}

export const LIST_SYMBOL = Symbol('LIST');

export function setList() {
	return setContext<List>(LIST_SYMBOL, new List());
}

export function getList() {
	return getContext<ReturnType<typeof setList>>(LIST_SYMBOL);
}
