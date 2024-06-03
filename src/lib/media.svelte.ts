import { invoke } from '@tauri-apps/api/core';
import { type UnlistenFn, listen } from '@tauri-apps/api/event';
import type { Album, Media, Payload } from './type';

type FP = {
	FileProcessed: string;
};

type Ended = {
	Ended: {
		media: Media;
	};
};

type TF = {
	TotalFiles: {
		count: number;
	};
};

export default class MediaState {
	albums: Album[] = $state([]);
	loaded = $state(false);
	loading = $state(false);
	private unlistenners = new Set<UnlistenFn>();
	loading_data = $state('');
	files_count = $state(100);
	treatedFilesCount = $state(0);

	constructor() {}

	async load() {
		this.loading = true;
		this.treatedFilesCount = 0;

		this.unlistenners.add(
			await listen<Ended>('cache-update-end', (ev) => {
				this.albums = ev.payload.Ended.media.albums;
				this.loading = false;
				this.loaded = true;

				this.unlistenners.forEach((e) => {
					e();
				});
			})
		);

		this.unlistenners.add(
			await listen<TF>('cache-update-files', (ev) => {
				this.treatedFilesCount = 0;
				this.files_count = ev.payload.TotalFiles.count;
			})
		);

		this.unlistenners.add(
			await listen<FP>('cache-update-data', (ev) => {
				this.loading_data = ev.payload.FileProcessed;
				this.treatedFilesCount++;
			})
		);

		await invoke('index');
	}

	getAlbum(id: string) {
		return this.albums.find((v) => v.id === id);
	}
}
