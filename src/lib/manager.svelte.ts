import { emit } from '@tauri-apps/api/event';
import { type Track } from './type';
import { type UnlistenFn, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

enum PlayingMode {
	Normal,
	Shuffle
}

enum QueueMode {
	Continue,
	Repeat,
	RepeatAll
}

enum QueueAddMode {
	Top = 'Top',
	Bottom = 'Bottom'
}

type QueueAdd = {
	tracks: Array<Track>;
	mode: QueueAddMode;
};

/**
 * The player Manager
 * That big man
 */
export default class Manager {
	queue: Track[] = $state([]);
	currentTrack?: Track = $state();
	currentTime = $state(0);
	volume: number = $state(1);
	paused: boolean = $state(true);
	duration: number = $derived(this.currentTrack ? this.currentTrack.duration : 0);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	watching = false;
	onplayat?: (time: number) => void;
	ontooglepp?: () => Promise<void>;
	onvolumechange?: (vol: number) => void;
	ontimeupdate?: (time: number) => void;
	onplay?: (track: Track) => void;
	onPlayerActivate?: () => void;
	onPlayerDeactivate?: () => void;

	activatePlayer() {
		if (this.onPlayerActivate) {
			this.onPlayerActivate();
		}
	}

	async play(track: Track) {
		await emit('play', track);
	}

	async volumeTo(vol: number) {
		await emit('volumeto', vol);
		this.volume = vol;
	}

	async seekTo(time: number) {
		await emit('seekto', time);
		this.currentTime = time;
	}

	async tooglepp() {
		if (this.paused) {
			await this.requestplay();
		} else {
			await this.requestpause();
		}
	}

	async requestplay() {
		await emit('requestplay');
	}
	async requestpause() {
		await emit('requestpause');
	}

	deactivate() {
		if (this.onPlayerDeactivate) {
			this.onPlayerDeactivate();
		}
	}

	async prev() {
		await emit('prevtrack');
	}

	async watchevents() {
		await invoke('start_player');
		await listen<Track | undefined>('newtrack', (ev) => {
			this.currentTrack = ev.payload;
			if (this.currentTrack) {
				this.paused = false;
				if (this.onplay) {
					this.onplay(this.currentTrack);
				}
			} else {
				this.paused = true;
			}
		});

		await listen<Track[]>('queueupdate', (ev) => {
			this.queue = ev.payload;
		});

		await listen<number>('timeupdate', (ev) => {
			this.currentTime = ev.payload;
		});

		await listen('pause', (_) => {
			this.paused = true;
		});

		await listen('play', (_) => {
			this.paused = false;
		});

		this.watching = true;
	}

	async clearQueue() {
		await emit('queueclear');
	}

	async addToQueue(track: Track) {
		const add: QueueAdd = {
			tracks: [track],
			mode: QueueAddMode.Bottom
		};
		await emit('queueadd', add);
	}

	async addManyToQueue(tracks: Track[]) {
		const add: QueueAdd = {
			tracks,
			mode: QueueAddMode.Bottom
		};
		await emit('queueadd', add);
	}

	async next() {
		await emit('nexttrack');
	}

	constructor() {}
}
