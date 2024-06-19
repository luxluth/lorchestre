import type { Album, Media, Playlist, Track } from './type';
import { MUD_ENDPOINT } from './config';

export default class MediaState {
	albums: Album[] = $state([]);
	playlists: Playlist[] = $state([]);
	loaded = $state(false);
	loading = $state(false);
	loading_data = $state('');
	files_count = $state(100);
	treatedFilesCount = $state(0);

	constructor() {}

	async load() {
		let media = (await (await fetch(`${MUD_ENDPOINT}/media`)).json()) as Media;
		this.albums = media.albums;
		this.playlists = media.playlists;
		this.loaded = true;
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
