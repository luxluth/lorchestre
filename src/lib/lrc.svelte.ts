import { getContext, setContext } from 'svelte';
import { type LyricLine, type Line, type Track, type LyricsResponse, ToastKind } from './type';
import type AppConfig from './config.svelte';
import { getLNTime, getLyricsUri, searchLyricsUri } from './utils';
import type ToastManager from './toast.svelte';
import type EditViewController from './editviewController.svelte';
import LyricsChoiceEditView from '$lib/components/LrycisChoiceEditView.svelte';
import { _ } from 'svelte-i18n';
import { get } from 'svelte/store';

const eq = (a: Line[], b: Line[]) => {
	return a.length === b.length && a.every((element, index) => element === b[index]);
};

export default class LrcManager {
	lines: Line[] = $state([]);
	private conf: AppConfig;
	private currentActiveLines: Line[] = $state([]);
	private chs = new Set<(lines: Line[]) => void>();
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
		console.debug(raw_lines);

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
		this.currentActiveLines = [];
	}

	resetFromLines(duration: number, raw_lines: LyricLine[]) {
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
			this.chs.forEach((f) => {
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

	set oncuechange(fn: () => void) {
		this.chs.add(fn);
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
