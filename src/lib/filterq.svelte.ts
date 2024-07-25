import { getContext, setContext } from 'svelte';
import { FilterOrder, FilterType } from './type';

export default class FilterQuery {
	type: FilterType = $state(FilterType.TimeBased);
	order: FilterOrder = $state(FilterOrder.Ascendant);

	constructor(
		defaultType: FilterType = FilterType.TimeBased,
		defaultOrder = FilterOrder.Ascendant
	) {
		this.type = defaultType;
		this.order = defaultOrder;
	}
}

export const FILTER_SYMBOL = Symbol('FILTERQUERY');

export function setFilter() {
	return setContext<FilterQuery>(FILTER_SYMBOL, new FilterQuery());
}

export function getFilter() {
	return getContext<ReturnType<typeof setFilter>>(FILTER_SYMBOL);
}
