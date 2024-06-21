import type { Album, Media, Playlist, Track } from './type';
import { io } from 'socket.io-client';
import { getContext } from 'svelte';
import type AppConfig from './config.svelte';

export default class MediaState {
	albums: Album[] = $state([]);
	playlists: Playlist[] = $state([]);
	loaded = $state(false);
	loading = $state(false);
	loading_data = $state('');
	files_count = $state(100);
	treatedFilesCount = $state(0);
	updatingmedia = $state(false);

	constructor() {}

	async load() {
		let config = getContext<AppConfig>('appconf');
		const endpoint = config.getMUDEndpoint();
		let media = (await (await fetch(`http://${endpoint}/media`)).json()) as Media;
		this.albums = media.albums;
		this.playlists = media.playlists;
		this.loaded = true;
		try {
			console.log(`ws://${endpoint}`);
			const socket = io(`ws://${endpoint}`);

			socket.on('updatingmedia', (state: boolean) => {
				this.updatingmedia = state;
				console.log(this.updatingmedia);
			});

			socket.on('newmedia', (media: Media) => {
				this.albums = media.albums;
				this.playlists = media.playlists;
			});
		} catch (e) {
			console.log(e);
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
		this.albums.forEach((album) => {
			album.tracks.forEach((song) => {
				songs.push(song);
			});
		});

		return songs;
	}

	getAlbum(id: string) {
		return this.albums.find((v) => v.id === id);
	}
}
