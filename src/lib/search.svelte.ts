import type { Socket } from 'socket.io-client';
import type { SearchResults } from './type';
import { getContext, setContext } from 'svelte';

export default class SearchSupervisor {
	query: string = $state('');
	socket: Socket | null = $state(null);
	results: SearchResults = $state({
		albums: [],
		tracks: []
	});
	initialized = $state(false);

	isEmpty() {
		return this.results.albums.length === 0 && this.results.tracks.length === 0;
	}

	init(socket: Socket) {
		socket.on('searchresponse', (res: SearchResults) => {
			this.results = res;
		});

		this.socket = socket;
		this.initialized = true;
	}
	search() {
		if (this.socket) {
			if (this.query.length > 0) {
				this.socket.emit('search', this.query);
			} else {
				this.results = { albums: [], tracks: [] };
			}
		}
	}
}

export const SEARCH_SYMBOL = Symbol('SEARCH');

export function setSearch() {
	return setContext<SearchSupervisor>(SEARCH_SYMBOL, new SearchSupervisor());
}

export function getSearch() {
	return getContext<ReturnType<typeof setSearch>>(SEARCH_SYMBOL);
}
