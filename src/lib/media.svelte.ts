import type { Album, Media, Playlist, Track } from './type';
import { io } from 'socket.io-client';
import { getContext } from 'svelte';
import type AppConfig from './config.svelte';
import { listen } from '@tauri-apps/api/event';
import { recordToMap } from './utils';
import type SearchSupervisor from './search.svelte';

export default class MediaState {
	albums: Album[] = $state([]);
	playlists: Playlist[] = $state([]);
	tracks: Map<string, Track> = $state(new Map());
	loaded = $state(false);
	loading = $state(false);
	loading_data = $state('');
	files_count = $state(100);
	treatedFilesCount = $state(0);
	updatingmedia = $state(false);
	search: SearchSupervisor;

	constructor(search: SearchSupervisor) {
		this.search = search;
	}

	async load() {
		let config = getContext<AppConfig>('appconf');
		const endpoint = config.getDaemonEndpoint();
		let media = (await (await fetch(`http://${endpoint}/media`)).json()) as Media;
		this.albums = media.albums;
		this.playlists = media.playlists;
		this.tracks = recordToMap(media.tracks);
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
				this.tracks = recordToMap(media.tracks);
			});
		} catch (e) {
			console.warn(e);
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
