import { type Track, type QueueTrack, QueueAddMode, QueueMode, PlayingMode } from './type';
import { toQueueTrack } from './utils';

/**
 * The player Manager
 * That big man
 */
export default class Manager {
	queue: QueueTrack[] = $state([]);
	currentTrack?: QueueTrack = $state();
	private lastQueueOrder: QueueTrack[] = [];
	currentTime = $state(0);
	history: QueueTrack[] = $state([]);
	volume: number = $state(1);
	paused: boolean = $state(true);
	duration: number = $derived(this.currentTrack ? this.currentTrack.duration : 0);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	onseekto?: (time: number) => void;
	ontogglepp?: () => Promise<void>;
	onvolumechange?: (vol: number) => void;
	ontimeupdate?: (time: number) => void;
	onplay?: (track: QueueTrack) => Promise<void>;
	onPlayerActivate?: () => void;
	onPlayerDeactivate?: () => void;
	onstop?: () => void;

	activatePlayer() {
		if (this.onPlayerActivate) {
			this.onPlayerActivate();
		}
	}

	async play(track: Track) {
		await this.onplay?.(toQueueTrack(track));
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

	async toggleShuffle() {
		switch (this.pmode) {
			case PlayingMode.Normal:
				this.pmode = PlayingMode.Shuffle;
				this.lastQueueOrder = this.queue;
				this.queue = this.shuffle(this.queue);
				break;
			case PlayingMode.Shuffle:
				this.pmode = PlayingMode.Normal;
				this.queue = this.lastQueueOrder;
				if (this.currentTrack) await this.shiftTo(this.currentTrack, false);
				break;
		}
	}

	// Fisher-Yates shuffle function
	private shuffle(queue: QueueTrack[]): QueueTrack[] {
		for (let i = queue.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[queue[i], queue[j]] = [queue[j], queue[i]];
		}
		return queue;
	}

	async shufflePlay(tracks: Track[]) {
		this.clearQueue();
		this.pmode = PlayingMode.Normal;
		this.addManyToQueue(tracks);
		this.toggleShuffle();
		let track = this.queue.shift();
		if (track) {
			await this.play(track as Track);
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

	clearQueue() {
		this.queue = [];
	}

	async shiftTo(t: QueueTrack, play = true) {
		let new_queue = [];
		let push = false;
		for (let i = 0; i < this.queue.length; i++) {
			if (push) {
				new_queue.push(this.queue[i]);
			}

			if (this.queue[i].id === t.id && this.queue[i].uuid === t.uuid) {
				if (play) await this.play(t as Track);
				push = true;
			}
		}

		this.queue = new_queue;
	}

	remove(track: QueueTrack) {
		this.queue = this.queue.filter((t) => t.id !== track.id && t.uuid !== track.uuid);
	}

	addToQueue(track: Track, mode = QueueAddMode.Bottom) {
		switch (mode) {
			case QueueAddMode.Top:
				this.queue = [toQueueTrack(track), ...this.queue];
				break;
			case QueueAddMode.Bottom:
				this.queue.push(toQueueTrack(track));
				break;
		}
	}

	addManyToQueue(tracks: Track[], mode = QueueAddMode.Bottom) {
		switch (mode) {
			case QueueAddMode.Top:
				tracks.reverse().forEach((track) => {
					this.addToQueue(track, mode);
				});
				break;
			case QueueAddMode.Bottom:
				tracks.forEach((track) => {
					this.addToQueue(track);
				});
				break;
		}
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
				this.paused = true;
				this.currentTrack = undefined;
				this.onstop?.();
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
