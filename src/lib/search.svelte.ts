import type { SearchResults } from './type';
import { getContext, setContext } from 'svelte';
import type Ws from './utils/websocket';

export default class SearchSupervisor {
	query: string = $state('');
	socket: Ws | null = $state(null);
	results: SearchResults = $state({
		albums: [],
		tracks: []
	});
	local_results: SearchResults = $state({
		albums: [],
		tracks: []
	});
	initialized = $state(false);

	isEmpty() {
		return this.results.albums.length === 0 && this.results.tracks.length === 0;
	}

	init(socket: Ws) {
		socket.on('searchresponse', (res: SearchResults) => {
			this.results = res;
		});

		socket.on('localsr', (res: SearchResults) => {
			this.local_results = res;
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

	localSearch(term: string) {
		if (this.socket) {
			if (term.length > 0) {
				this.socket.emit('localsearch', term);
			} else {
				this.local_results = { albums: [], tracks: [] };
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
