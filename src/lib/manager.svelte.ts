import { type Track, type QueueTrack, QueueAddMode, QueueMode, PlayingMode } from './type';
import { toQueueTrack } from './utils';

type Func<Out> = () => Out;
type VoidFunc = Func<void>;
type OneArgFunc<In, Out> = (arg: In) => Out;
type TimeFunction = OneArgFunc<number, void>;

/**
 * The player Manager
 * That big man
 */
export default class Manager {
	queue: QueueTrack[] = $state([]);
	currentTrack?: QueueTrack = $state();
	private lastQueueOrder: QueueTrack[] = [];
	history: QueueTrack[] = $state([]);
	#innerVolume: number = $state(1);
	paused: boolean = $state(true);
	#realCurrentTime = $state(0);
	duration: number = $derived(this.currentTrack ? this.currentTrack.duration : 0);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	ontogglepp?: Func<Promise<void>>;
	onplay?: OneArgFunc<QueueTrack, Promise<void>>;
	onPlayerActivate?: VoidFunction;
	onPlayerDeactivate?: VoidFunc;
	onstop?: VoidFunc;
	private onseektofuncs = new Set<TimeFunction>();
	private ontimeupdatefuncs = new Set<TimeFunction>();
	private afterplayfuncs = new Set<VoidFunc>();

	activatePlayer() {
		if (this.onPlayerActivate) {
			this.onPlayerActivate();
		}
	}

	set ontimeupdate(f: TimeFunction) {
		this.ontimeupdatefuncs.add(f);
	}

	set afterplay(f: VoidFunc) {
		this.afterplayfuncs.add(f);
	}

	set onseekto(f: TimeFunction) {
		this.onseektofuncs.add(f);
	}

	async play(track: Track | QueueTrack) {
		if (typeof (track as QueueTrack)['uuid'] !== 'undefined') {
			await this.onplay?.(track as QueueTrack);
		} else {
			this.clearQueue();
			await this.onplay?.(toQueueTrack(track));
		}
		this.paused = false;

		this.afterplayfuncs.forEach((func) => {
			func();
		});
	}

	seekTo(time: number) {
		this.currentTime = time;
		this.onseektofuncs.forEach((func) => {
			func(time);
		});
	}

	set currentTime(v: number) {
		this.#realCurrentTime = v;
	}

	get currentTime() {
		return this.#realCurrentTime;
	}

	set volume(v: number) {
		this.#innerVolume = v;
	}

	get volume() {
		return this.#innerVolume;
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
				this.lastQueueOrder = this.queue.map((t) => {
					return {
						title: t.title,
						artists: t.artists,
						track: t.track,
						album: t.album,
						album_artist: t.album_artist,
						album_id: t.album_id,
						album_year: t.album_year,
						lyrics: t.lyrics,
						cover_ext: t.cover_ext,
						mime: t.mime,
						color: t.color,
						created_at: t.created_at,
						is_light: t.is_light,
						file_path: t.file_path,
						duration: t.duration,
						bitrate: t.bitrate,
						id: t.id,
						uuid: t.uuid
					} as const;
				});
				this.queue = this.shuffle(this.queue);
				break;
			case PlayingMode.Shuffle:
				this.pmode = PlayingMode.Normal;
				this.queue = this.lastQueueOrder;
				// console.log('queue-reset', this.queue);
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
			this.play(track);
		}
	}

	clearQueue() {
		this.queue = [];
	}

	async shiftTo(t: QueueTrack, play = true) {
		let id = this.queue.findIndex((tr) => tr.id == t.id && tr.uuid == t.uuid);
		// console.log('shift-to', id, this.queue.length, t);
		if (id >= 0) {
			const track = this.queue[id];
			this.queue = this.queue.slice(id + 1);
			// console.log('new-queue', this.queue);
			if (play) await this.play(track);
		}
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
			this.play(track);
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
