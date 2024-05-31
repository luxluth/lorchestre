import { invoke } from '@tauri-apps/api/core';
import type { Album, Media, Payload } from './type';

export default class MediaState {
	albums: Album[] = $state([]);
	loaded = $state(false);
	loading = $state(false);

	constructor() {}

	async load() {
		this.loading = true;
		this.albums = (await invoke<Media>('index')).albums;
		this.loading = false;
		this.loaded = true;
	}

	getAlbum(id: string) {
		return this.albums.find((v) => v.id === id);
	}
}
