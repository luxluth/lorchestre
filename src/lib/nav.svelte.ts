import { getContext, setContext } from 'svelte';

export default class Nav {
	pageName = $state('Recommendation');
}

export const PAGE_NAME_SYMBOL = Symbol('PAGE_NAME_SYMBOL');

export function setNav() {
	return setContext<Nav>(PAGE_NAME_SYMBOL, new Nav());
}

export function getNav() {
	return getContext<ReturnType<typeof setNav>>(PAGE_NAME_SYMBOL);
}
