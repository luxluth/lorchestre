import { getContext, setContext } from 'svelte';
import { FilterOrder, FilterType } from './type';

export default class FilterQuery {
	type: FilterType = $state(FilterType.TimeBased);
	order: FilterOrder = $state(FilterOrder.Ascendant);
}

export const FILTER_SYMBOL = Symbol('FILTERQUERY');

export function setFilter() {
	return setContext<FilterQuery>(FILTER_SYMBOL, new FilterQuery());
}

export function getFilter() {
	return getContext<ReturnType<typeof setFilter>>(FILTER_SYMBOL);
}
