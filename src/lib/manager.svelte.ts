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
	history: Track[] = $state([]);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	onplayat?: (time: number) => void;
	ontooglepp?: () => Promise<void>;
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
		if (this.onplay) {
			await this.onplay(track);
		}
	}

	timeTo(time: number) {
		if (this.ontimeupdate) {
			this.ontimeupdate(time);
		}
	}

	volumeTo(vol: number) {
		if (this.onvolumechange) {
			this.onvolumechange(vol);
		}
	}

	playAt(time: number) {
		if (this.onplayat) {
			this.onplayat(time);
		}
	}

	async tooglepp() {
		if (this.ontooglepp) {
			await this.ontooglepp();
		}
	}

	deactivate() {
		if (this.onPlayerDeactivate) {
			this.onPlayerDeactivate();
		}
	}

	async prev() {
		let track = this.history.shift();
		if (track) {
			if (this.currentTrack) {
				this.queue = [this.currentTrack, ...this.queue];
			}
			if (this.onplay) await this.onplay(track);
		}
	}

	addToQueue(track: Track) {
		this.queue.push(track);
	}

	async next() {
		const track = this.queue.shift();
		if (track) {
			if (this.onplay) await this.onplay(track);
			this.history = [track, ...this.history];
		} else {
			this.deactivate();
			this.currentTrack = undefined;
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
