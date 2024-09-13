import { getContext, setContext } from 'svelte';
import {
	type LyricLine,
	type Line,
	type Track,
	type LyricsResponse,
	ToastKind,
	type ProccessedMarker
} from './type';
import type AppConfig from './config.svelte';
import { createSequenceGenerator, getLNTime, getLyricsUri, searchLyricsUri } from './utils';
import type ToastManager from './toast.svelte';
import type EditViewController from './editviewController.svelte';
import LyricsChoiceEditView from '$lib/components/LrycisChoiceEditView.svelte';
import { _ } from 'svelte-i18n';
import { get } from 'svelte/store';

const eq = (a: Line[], b: Line[]) => {
	return a.length === b.length && a.every((element, index) => element === b[index]);
};

type HookFunc = (lines: Line[]) => void;

export default class LrcManager {
	lines: Line[] = $state([]);
	proccessedMarkers: ProccessedMarker[] = $state([]);
	private conf: AppConfig;
	private currentActiveLines: Line[] = $state([]);
	private chs = new Set<[number, HookFunc]>();
	idNext = createSequenceGenerator();
	searching: boolean = $state(false);
	tm: ToastManager;
	evc: EditViewController;

	constructor(
		duration: number,
		raw_lines: LyricLine[],
		conf: AppConfig,
		tm: ToastManager,
		evc: EditViewController
	) {
		this.conf = conf;
		this.tm = tm;
		this.evc = evc;
		let lines: Line[] = [];

		for (let i = 0; i < raw_lines.length; i++) {
			const current = raw_lines[i] as LyricLine;
			const next = raw_lines[i + 1] as LyricLine | undefined;

			if (next) {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: getLNTime(next.time) / 1000,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			} else {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: duration,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			}
		}

		this.lines = lines;
	}

	async reset(duration: number, track: Track) {
		let lines: Line[] = [];
		let url = getLyricsUri(track.path_base64, this.conf);
		const raw_lines: LyricLine[] = ((await (await fetch(url)).json()) as { lyrics: LyricLine[] })
			.lyrics;

		for (let i = 0; i < raw_lines.length; i++) {
			const current = raw_lines[i] as LyricLine;
			const next = raw_lines[i + 1] as LyricLine | undefined;

			if (next) {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: getLNTime(next.time) / 1000,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			} else {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: duration,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			}
		}

		this.lines = lines;
		this.proccessedMarkers = this.analyzeMarkers(this.lines, track);
		this.currentActiveLines = [];
	}

	resetFromLines(raw_lines: LyricLine[], track: Track) {
		let lines: Line[] = [];

		for (let i = 0; i < raw_lines.length; i++) {
			const current = raw_lines[i] as LyricLine;
			const next = raw_lines[i + 1] as LyricLine | undefined;

			if (next) {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: getLNTime(next.time) / 1000,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			} else {
				lines.push({
					startTime: getLNTime(current.time) / 1000,
					text: current.text,
					id: i,
					endTime: track.duration,
					vocals: current.vocals,
					syllables: current.syllables,
					isInstrumental: current.is_instrumental,
					marker: current.marker
				});
			}
		}

		this.lines = lines;
		this.proccessedMarkers = this.analyzeMarkers(this.lines, track);
		this.currentActiveLines = [];
	}

	async searchLyrics(track: Track) {
		this.searching = true;
		let url = searchLyricsUri(track.path_base64, this.conf);
		let loc = get(_);

		try {
			let response = (await (await fetch(url)).json()) as LyricsResponse;
			if (response.lyrics.length === 0) {
				this.tm.new(ToastKind.Simple, loc('lrc_related.no_lyrics'));
			} else {
				this.evc.show(LyricsChoiceEditView, { response, track });
			}
		} catch (e) {
			this.tm.new(ToastKind.Error, loc('lrc_related.fetch_error'));
			console.warn(e);
		}
		this.searching = false;
	}

	private set als(val: Line[]) {
		if (!eq(val, this.currentActiveLines)) {
			this.currentActiveLines = val;
			this.chs.forEach(([_, f]) => {
				f(val);
			});
		}
	}

	update(time: number) {
		let activeLines = [];
		let i = 0,
			len = this.lines.length;
		while (i < len) {
			const line = this.lines[i];
			if (line.startTime > time) {
				break;
			}
			if (line.startTime <= time && line.endTime >= time) {
				activeLines.push(line);
			}
			i++;
		}
		this.als = activeLines;
	}

	get activeLines(): Line[] {
		return this.currentActiveLines;
	}

	oncuechange(fn: HookFunc): number {
		const hookId = this.idNext();
		this.chs.add([hookId, fn]);
		return hookId;
	}

	removeHook(id: number) {
		this.chs.forEach((pair) => {
			const [i, _] = pair;
			if (i === id) {
				this.chs.delete(pair);
			}
		});
	}

	analyzeMarkers(lyrics: Line[], track: Track) {
		let artistsInMarkers = new Set<string>();
		lyrics.forEach((line) => {
			if (typeof line.marker === 'object') {
				let [name, value] = line.marker.Named;
				if (name === 'singer') {
					if (track.artists.includes(value)) {
						artistsInMarkers.add(value);
					}
				}
			}
		});

		let artists = Array.from(artistsInMarkers);
		let proccessedMarkers: ProccessedMarker[] = [];

		lyrics.forEach((line, i) => {
			if (typeof line.marker === 'object') {
				let [name, value] = line.marker.Named;
				if (name === 'singer') {
					let idx = artists.findIndex((v) => v == value);
					proccessedMarkers.push({
						i,
						artistName: !line.isInstrumental && line.text != '' ? value : '@@',
						isMainVocal: idx === 0 && !line.isInstrumental && line.text != '',
						order: idx
					});
				}
			}
		});

		return proccessedMarkers;
	}

	isMainVocal(i: number) {
		const marker = this.getMarker(i);
		if (marker) {
			return marker.isMainVocal;
		} else {
			return false;
		}
	}

	getOrder(i: number) {
		const marker = this.getMarker(i);
		if (marker) {
			return marker.order;
		} else {
			return -1;
		}
	}

	getArtistName(i: number) {
		const marker = this.getMarker(i);
		if (marker) {
			return marker.artistName;
		} else {
			return '';
		}
	}

	getMarker(i: number) {
		return this.proccessedMarkers.find((v) => v.i == i);
	}
}

export function base(conf: AppConfig, tm: ToastManager, evc: EditViewController) {
	return new LrcManager(0, [], conf, tm, evc);
}

export const LRC_SYMBOL = Symbol('LRCMANAGER');

export function setLrc(conf: AppConfig, tm: ToastManager, evc: EditViewController) {
	return setContext<LrcManager>(LRC_SYMBOL, base(conf, tm, evc));
}

export function getLrc() {
	return getContext<ReturnType<typeof setLrc>>(LRC_SYMBOL);
}
