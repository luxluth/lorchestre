import { getContext, onMount, setContext } from 'svelte';
import FilterQuery from './filterq.svelte';
import type MediaState from './media.svelte';
import { FilterType, type Playlist, type Track } from './type';

export default class List {
	activeList = $state<Playlist | null>(null);
	filters = new FilterQuery(FilterType.NoFilter);
	tracks = $state<Track[]>([]);

	constructor(media: MediaState) {
		onMount(() => {
			const playlist = localStorage.getItem('playlist');
			if (playlist) {
				this.activeList = JSON.parse(playlist);
			}
		});

		$effect(() => {
			let tracks = [];
			for (const path of this.activeList?.tracks ?? []) {
				let track = media.getTrack(path);
				if (track) tracks.push(track);
			}
			this.tracks = tracks;
		});

		$effect(() => {
			localStorage.setItem('playlist', JSON.stringify(this.activeList));
		});
	}
}

export const LIST_SYMBOL = Symbol('LIST');

export function setList(m: MediaState) {
	return setContext<List>(LIST_SYMBOL, new List(m));
}

export function getList() {
	return getContext<ReturnType<typeof setList>>(LIST_SYMBOL);
}
