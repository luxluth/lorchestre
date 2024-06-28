import { getContext, setContext } from 'svelte';
import FilterQuery from './filterq.svelte';
import type MediaState from './media.svelte';
import { type Playlist, type Track } from './type';

export default class List {
	activeList = $state<Playlist>();
	filters = new FilterQuery();
	tracks = $state<Track[]>([]);

	constructor(media: MediaState) {
		$effect(() => {
			let tracks = [];
			for (const path of this.activeList?.tracks ?? []) {
				let track = media.getTrack(path);
				if (track) tracks.push(track);
			}
			this.tracks = tracks;
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
