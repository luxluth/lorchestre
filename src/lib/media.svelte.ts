import type { Album, Media, Playlist, Track } from './type';
import { io } from 'socket.io-client';
import { getContext, setContext } from 'svelte';
import { listen } from '@tauri-apps/api/event';
import { unionToMap } from './utils';
import type SearchSupervisor from './search.svelte';
import { getAppConfig } from './config.svelte';

export default class MediaState {
	albums: Album[] = [];
	playlists: Playlist[] = $state([]);
	tracks: Map<string, Track> = $state(new Map());
	loaded = $state(false);
	loading = $state(false);
	loading_data = $state('');
	files_count = $state(100);
	treatedFilesCount = $state(0);
	updatingmedia = $state(false);
	search: SearchSupervisor;
	loadIntervalPingId: number = -1;

	constructor(search: SearchSupervisor) {
		this.search = search;
	}

	async load() {
		let config = getAppConfig();
		const endpoint = config.getDaemonEndpoint();

		let response = null;
		try {
			response = await fetch(`http://${endpoint}/media`);
		} catch (e) {}
		if (response) {
			let media = (await response.json()) as Media;
			this.albums = media.albums;
			this.playlists = media.playlists;
			this.tracks = unionToMap(media.tracks);
			this.loaded = true;

			await listen('startsync', () => {
				this.updatingmedia = true;
			});

			await listen('endsync', () => {
				this.updatingmedia = false;
			});

			try {
				const socket = io(`ws://${endpoint}`);
				if (!this.search.initialized) {
					this.search.init(socket);
				}
				socket.on('newmedia', (media: Media) => {
					this.albums = media.albums;
					this.playlists = media.playlists;
					this.tracks = unionToMap(media.tracks);
				});
			} catch (e) {
				console.warn(e);
			}
		} else {
			this.loadIntervalPingId = window.setInterval(() => {
				(async () => {
					try {
						let response = await fetch(`http://${endpoint}/media`);
						if (response.status === 200) {
							let media = (await response.json()) as Media;
							this.albums = media.albums;
							this.playlists = media.playlists;
							this.tracks = unionToMap(media.tracks);
							this.loaded = true;

							await listen('startsync', () => {
								this.updatingmedia = true;
							});

							await listen('endsync', () => {
								this.updatingmedia = false;
							});

							try {
								const socket = io(`ws://${endpoint}`);
								if (!this.search.initialized) {
									this.search.init(socket);
								}
								socket.on('newmedia', (media: Media) => {
									this.albums = media.albums;
									this.playlists = media.playlists;
									this.tracks = unionToMap(media.tracks);
								});
							} catch (e) {
								console.warn(e);
							}

							clearInterval(this.loadIntervalPingId);
						}
					} catch (e) {}
				})();
			}, 1500);
		}
	}

	getSongsCount() {
		let count = 0;
		this.albums.forEach((album) => {
			album.tracks.forEach((_song) => {
				count++;
			});
		});

		return count;
	}

	getSongs() {
		let songs: Track[] = [];
		for (const [_, t] of this.tracks) {
			songs.push(t);
		}

		return songs;
	}

	getTrack(path: string) {
		return this.tracks.get(path);
	}

	getAlbum(id: string) {
		return this.albums.find((v) => v.id === id);
	}
}

export const MEDIA_SYMBOL = Symbol('MEDIA');

export function setMedia(s: SearchSupervisor) {
	return setContext<MediaState>(MEDIA_SYMBOL, new MediaState(s));
}

export function getMedia() {
	return getContext<ReturnType<typeof setMedia>>(MEDIA_SYMBOL);
}
