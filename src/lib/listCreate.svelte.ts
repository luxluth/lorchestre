import { getContext, setContext } from 'svelte';
import type MediaState from './media.svelte';

export default class ListCreate {
	addedTracks: string[] = $state([]);
	Name: string = $state('');
	Description: string = $state('');
	xCoverPath: string = $state('');
	ImageData = $state('');
	media: MediaState;

	constructor(media: MediaState) {
		this.media = media;
		this.Name = `Playlist N°${media.playlists.length + 1}`;
	}
}

export const LS_SYMBOL = Symbol('LS_CREATE');

export function setListCreator(media: MediaState) {
	return setContext<ListCreate>(LS_SYMBOL, new ListCreate(media));
}

export function getListCreator() {
	return getContext<ReturnType<typeof setListCreator>>(LS_SYMBOL);
}
