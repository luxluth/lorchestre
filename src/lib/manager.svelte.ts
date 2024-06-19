import { type Track } from './type';

enum PlayingMode {
	Normal,
	Shuffle
}

enum QueueMode {
	Continue,
	Repeat,
	RepeatAll
}

// enum QueueAddMode {
// 	Top = 'Top',
// 	Bottom = 'Bottom'
// }

/**
 * The player Manager
 * That big man
 */
export default class Manager {
	queue: Track[] = $state([]);
	currentTrack?: Track = $state();
	currentTime = $state(0);
	history: Track[] = $state([]);
	volume: number = $state(1);
	paused: boolean = $state(true);
	duration: number = $derived(this.currentTrack ? this.currentTrack.duration : 0);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	onseekto?: (time: number) => void;
	ontogglepp?: () => Promise<void>;
	onvolumechange?: (vol: number) => void;
	ontimeupdate?: (time: number) => void;
	onplay?: (track: Track) => Promise<void>;
	onPlayerActivate?: () => void;
	onPlayerDeactivate?: () => void;

	activatePlayer() {
		if (this.onPlayerActivate) {
			this.onPlayerActivate();
		}
	}

	async play(track: Track) {
		await this.onplay?.(track);
		this.paused = false;
	}

	async volumeTo(vol: number) {
		this.volume = vol;
		this.onvolumechange?.(vol);
	}

	async seekTo(time: number) {
		this.currentTime = time;
		this.ontimeupdate?.(time);
	}

	async togglepp() {
		await this.ontogglepp?.();
	}

	deactivate() {
		if (this.onPlayerDeactivate) {
			this.onPlayerDeactivate();
		}
	}

	async prev() {
		let track = this.history.pop();
		if (track) {
			if (this.currentTrack) {
				this.queue = [this.currentTrack, ...this.queue];
			}
			if (this.onplay) await this.onplay(track);
		}
	}

	async clearQueue() {
		this.queue = [];
	}

	async addToQueue(track: Track) {
		this.queue.push(track);
	}

	async addManyToQueue(tracks: Track[]) {
		this.queue.push(...tracks);
	}

	async next() {
		const track = this.queue.shift();
		if (track) {
			if (this.currentTrack) this.history.push(this.currentTrack);
			if (this.onplay) await this.onplay(track);
		} else {
			if (this.onPlayerDeactivate) {
				console.log('here');
				this.onPlayerDeactivate();
				this.currentTrack = undefined;
			}
		}
	}

	constructor(options?: { queue?: Track[]; qmode?: QueueMode; pmode?: PlayingMode }) {
		if (options) {
			if (options.queue) {
				this.queue = this.queue;
			}
			if (options.qmode) {
				this.qmode = options.qmode;
			}
			if (options.pmode) {
				this.pmode = options.pmode;
			}
		}
	}
}
