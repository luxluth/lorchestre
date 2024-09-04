import { getContext, onMount, setContext } from 'svelte';
import { page } from '$app/stores';

export default class Nav {
	pageName = $state('Recommendation');
	pageRouteId: string | null = $state('/');

	constructor() {
		onMount(() => {
			const unsub = page.subscribe((e) => {
				this.pageRouteId = e.route.id;
			});

			return () => {
				unsub();
			};
		});
	}
}

export const PAGE_NAME_SYMBOL = Symbol('PAGE_NAME_SYMBOL');

export function setNav() {
	return setContext<Nav>(PAGE_NAME_SYMBOL, new Nav());
}

export function getNav() {
	return getContext<ReturnType<typeof setNav>>(PAGE_NAME_SYMBOL);
}
