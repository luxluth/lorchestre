import { goto } from '$app/navigation';
import { getContext, onMount, setContext } from 'svelte';

export default class Page {
	initialized = false;
	innerAddr = $state('/');

	set currentAddr(v: string) {
		if (this.initialized) {
			this.innerAddr = v;
		}
	}

	constructor() {
		onMount(() => {
			const path = localStorage.getItem('currentAddr');
			if (path) {
				goto(JSON.parse(path));
			}
			this.initialized = true;
		});

		$effect(() => {
			localStorage.setItem('currentAddr', JSON.stringify(this.innerAddr));
		});
	}
}

export const PAGE_SYMBOL = Symbol('PAGE');

export function setPage() {
	return setContext<Page>(PAGE_SYMBOL, new Page());
}

export function getList() {
	return getContext<ReturnType<typeof setPage>>(PAGE_SYMBOL);
}
