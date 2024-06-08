import { FilterOrder, FilterType } from './type';

export default class FilterQuery {
	type: FilterType = $state(FilterType.TimeBased);
	order: FilterOrder = $state(FilterOrder.Ascendant);
}
