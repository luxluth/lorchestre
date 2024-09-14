import { getContext, onMount, setContext } from 'svelte';
import { type Track, type QueueTrack, QueueAddMode, QueueMode, PlayingMode } from './type';
import { createSequenceGenerator, toQueueTrack } from './utils';
import { invoke } from '@tauri-apps/api/core';
import { TauriEvent, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

type Func<Out> = () => Out;
type VoidFunc = Func<void>;
type OneArgFunc<In, Out> = (arg: In) => Out;
type TimeFunction = OneArgFunc<number, void>;

const QUEUE_LIMIT = 70;

/**
 * The player Manager
 * That big man
 */
export default class Manager {
	queue: QueueTrack[] = $state([]);
	dormantQueue: QueueTrack[] = $state([]);
	currentTrack: QueueTrack | null = $state(null);
	private lastQueueOrder: QueueTrack[] = [];
	history: QueueTrack[] = $state([]);
	#innerVolume: number = $state(1);
	paused: boolean = $state(true);
	#realCurrentTime = $state(0);
	initialized = false;
	duration: number = $derived(this.currentTrack ? this.currentTrack.duration : 0);
	qmode = $state(QueueMode.Continue);
	pmode = $state(PlayingMode.Normal);
	ontogglepp?: Func<Promise<void>>;
	onplay?: OneArgFunc<QueueTrack, Promise<void>>;
	onPlayerActivate?: VoidFunction;
	onPlayerDeactivate?: VoidFunc;
	onstop?: VoidFunc;
	idNext = createSequenceGenerator();
	private onseektofuncs = new Set<TimeFunction>();
	private ontimeupdatefuncs = new Set<TimeFunction>();
	private afterplayfuncs = new Set<[number, VoidFunc]>();

	activatePlayer() {
		if (this.onPlayerActivate) {
			this.onPlayerActivate();
		}
	}

	set ontimeupdate(f: TimeFunction) {
		this.ontimeupdatefuncs.add(f);
	}

	afterplay(f: VoidFunc) {
		const id = this.idNext();
		this.afterplayfuncs.add([id, f]);
		return () => {
			this.#removeAfterPlayEvent(id);
		};
	}

	#removeAfterPlayEvent(id: number) {
		this.afterplayfuncs.forEach((pair) => {
			const [i, _] = pair;
			if (i === id) {
				this.afterplayfuncs.delete(pair);
			}
		});
	}

	set onseekto(f: TimeFunction) {
		this.onseektofuncs.add(f);
	}

	async play(track: Track | QueueTrack | null) {
		if (track !== null) {
			if (typeof (track as QueueTrack)['id'] !== 'undefined') {
				await this.onplay?.(track as QueueTrack);
			} else {
				this.clearQueue();
				await this.onplay?.(toQueueTrack(track));
			}
			// if (this.initialized) {
			// 	this.paused = false;
			// }

			this.afterplayfuncs.forEach(([_, func]) => {
				func();
			});
		}
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
						path_base64: t.path_base64,
						disc: t.disc,
						encoder: t.encoder,
						genres: t.genres
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
	private shuffle<T>(queue: T[]): T[] {
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
		this.dormantQueue = [];
		this.history = [];
	}

	async shiftTo(t: QueueTrack, play = true) {
		let id = this.queue.findIndex((tr) => tr.id == t.id);
		// console.log('shift-to', id, this.queue.length, t);
		if (id >= 0) {
			const track = this.queue[id];
			this.queue = this.queue.slice(id + 1);
			// console.log('new-queue', this.queue);
			if (play) await this.play(track);
		}
	}

	remove(track: QueueTrack) {
		this.queue = this.queue.filter((t) => t.id !== track.id);
	}

	addToQueue(track: Track, mode = QueueAddMode.Bottom) {
		switch (mode) {
			case QueueAddMode.Top:
				if (this.queue.length <= QUEUE_LIMIT) {
					this.queue = [toQueueTrack(track), ...this.queue];
				} else {
					this.dormantQueue = [toQueueTrack(track), ...this.dormantQueue];
				}
				break;
			case QueueAddMode.Bottom:
				if (this.queue.length <= QUEUE_LIMIT) {
					this.queue.push(toQueueTrack(track));
				} else {
					this.dormantQueue.push(toQueueTrack(track));
				}
				break;
		}
	}

	applyShuffle(tracks: Track[]) {
		if (this.pmode === PlayingMode.Shuffle) {
			return this.shuffle(tracks);
		} else {
			return tracks;
		}
	}

	cycleThroughQmode() {
		switch (this.qmode) {
			case QueueMode.Continue:
				this.qmode = QueueMode.RepeatAll;
				break;
			case QueueMode.Repeat:
				this.qmode = QueueMode.Continue;
				break;
			case QueueMode.RepeatAll:
				this.qmode = QueueMode.Repeat;
				break;
		}
	}

	addManyToQueue(tracks: Track[], mode = QueueAddMode.Bottom) {
		const songs = this.applyShuffle(tracks);
		switch (mode) {
			case QueueAddMode.Top:
				songs.reverse().forEach((track) => {
					this.addToQueue(track, mode);
				});
				break;
			case QueueAddMode.Bottom:
				songs.forEach((track) => {
					this.addToQueue(track);
				});
				break;
		}
	}

	async next() {
		let track: QueueTrack | undefined;
		switch (this.qmode) {
			case QueueMode.Continue:
				track = this.queue.shift();
				if (typeof track === 'undefined') {
					if (this.dormantQueue.length > 0) {
						let newTracks = this.dormantQueue.splice(0, QUEUE_LIMIT);
						this.addManyToQueue(newTracks);
						track = this.queue.shift();
					}
				}
				if (track) {
					if (this.currentTrack) this.history.push(this.currentTrack);
					this.play(track);
				} else {
					if (this.onPlayerDeactivate) {
						this.onPlayerDeactivate();
						this.paused = true;
						this.currentTrack = null;
						this.history = [];
						this.onstop?.();
					}
				}
				break;
			case QueueMode.Repeat:
				if (this.currentTrack) this.play(this.currentTrack);
				break;
			case QueueMode.RepeatAll:
				let trackToPlay = this.queue.shift();
				if (typeof trackToPlay === 'undefined') {
					if (this.dormantQueue.length > 0) {
						let newTracks = this.dormantQueue.splice(0, QUEUE_LIMIT);
						this.addManyToQueue(newTracks);
						trackToPlay = this.queue.shift();
					}
				}
				if (trackToPlay) {
					if (this.currentTrack) this.history.push(this.currentTrack);
					this.play(trackToPlay);
				} else {
					this.queue.length = 0;
					this.addManyToQueue(this.history.splice(0, QUEUE_LIMIT));
					const track = this.queue.shift();
					if (track) {
						if (this.currentTrack) this.history.push(this.currentTrack);
						this.play(track);
					} else {
						if (this.onPlayerDeactivate) {
							console.log('here');
							this.onPlayerDeactivate();
							this.paused = true;
							this.currentTrack = null;
							this.onstop?.();
						}
					}
				}
		}
	}

	static getLocalStorageData(currentVersion: string): DataFromLocalStorage {
		let dataVersion = localStorage.getItem('__version');
		if (dataVersion) {
			if (dataVersion != currentVersion) {
				console.log('different version:', dataVersion, currentVersion);
				localStorage.clear();
				localStorage.setItem('__version', currentVersion);
			}
		} else {
			localStorage.clear();
			localStorage.setItem('__version', currentVersion);
		}
		return {
			qmode: localStorage.getItem('qmode'),
			pmode: localStorage.getItem('pmode'),
			history: localStorage.getItem('history'),
			queue: localStorage.getItem('queue'),
			dormantQueue: localStorage.getItem('dormantQueue'),
			currentTrack: localStorage.getItem('currentTrack'),
			currentTime: localStorage.getItem('currentTime'),
			volume: localStorage.getItem('volume')
		};
	}

	constructor() {
		onMount(() => {
			let unlistens: UnlistenFn[] = [];
			(async () => {
				let currentVersion = await invoke<string>('version');
				const data = Manager.getLocalStorageData(currentVersion);

				if (data.currentTrack) await this.play(JSON.parse(data.currentTrack));
				if (data.queue) this.queue = JSON.parse(data.queue);
				if (data.dormantQueue) this.dormantQueue = JSON.parse(data.dormantQueue);
				if (data.history) this.history = JSON.parse(data.history);
				if (data.qmode) this.qmode = JSON.parse(data.qmode);
				if (data.pmode) this.pmode = JSON.parse(data.pmode);
				if (data.volume) this.volume = JSON.parse(data.volume);
				if (data.currentTime) this.currentTime = JSON.parse(data.currentTime);

				let win = getCurrentWindow();
				unlistens.push(
					await win.listen(TauriEvent.WINDOW_CLOSE_REQUESTED, async (_) => {
						localStorage.setItem('qmode', JSON.stringify(this.qmode));
						localStorage.setItem('pmode', JSON.stringify(this.pmode));
						localStorage.setItem('queue', JSON.stringify(this.queue));
						localStorage.setItem('history', JSON.stringify(this.history));
						localStorage.setItem('dormantQueue', JSON.stringify(this.dormantQueue));
						localStorage.setItem('currentTrack', JSON.stringify(this.currentTrack));
						localStorage.setItem('volume', JSON.stringify(this.volume));
						localStorage.setItem('currentTime', JSON.stringify(this.currentTime));
						await invoke('close');
					})
				);

				this.initialized = true;
			})();
			return () => {
				unlistens.forEach((unlisten) => {
					unlisten();
				});
			};
		});
	}
}

type DataFromLocalStorage = {
	qmode: string | null;
	pmode: string | null;
	history: string | null;
	queue: string | null;
	dormantQueue: string | null;
	currentTrack: string | null;
	currentTime: string | null;
	volume: string | null;
};

export const MANAGER_SYMBOL = Symbol('MANAGER');

export function setManager() {
	return setContext<Manager>(MANAGER_SYMBOL, new Manager());
}

export function getManager() {
	return getContext<ReturnType<typeof setManager>>(MANAGER_SYMBOL);
}
