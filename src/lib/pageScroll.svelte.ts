import { page } from '$app/state';
import { getContext, onMount, setContext } from 'svelte';

type Ev = UIEvent & {
	currentTarget: EventTarget & HTMLElement;
};

class PageScroll {
	scrollPositions: Map<string, number> = $state(new Map());
	prevPath: string = $state('');

	constructor() {
		$effect(() => {
			const path = page.url.pathname;
			const main = document.getElementById('__main__') as HTMLElement;
			if (path !== this.prevPath) {
				const ev = new CustomEvent('pagechanged');
				document.dispatchEvent(ev);
				const scrollTop = this.scrollPositions.get(path);
				if (scrollTop) {
					main.scrollTo({ top: scrollTop });
				} else {
					main.scrollTo({ top: 0 });
				}

				this.prevPath = path;
			}
		});
	}

	save(e: Ev, url: string) {
		this.scrollPositions.set(url, e.currentTarget.scrollTop);
	}
}

const PAGE_SYMBOL = Symbol('PAGE_SCROLL');

export function setPageScroll() {
	return setContext<PageScroll>(PAGE_SYMBOL, new PageScroll());
}

export function getPageScroll() {
	return getContext<ReturnType<typeof setPageScroll>>(PAGE_SYMBOL);
}
