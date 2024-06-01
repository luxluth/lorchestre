import type { LyricLine, Line } from './type';

const eq = (a: Line[], b: Line[]) => {
	return a.length === b.length && a.every((element, index) => element === b[index]);
};

export default class LrcManager {
	lines: Line[] = $state([]);
	private currentActiveLines: Line[] = $state([]);
	private chs = new Set<() => void>();

	constructor(duration: number, raw_lines: LyricLine[]) {
		let lines: Line[] = [];

		for (let i = 0; i < raw_lines.length; i++) {
			const current = raw_lines[i] as LyricLine;
			const next = raw_lines[i + 1] as LyricLine | undefined;

			if (next) {
				lines.push({
					startTime: current.start_time / 1000,
					text: current.text,
					id: i,
					endTime: next.start_time / 1000
				});
			} else {
				lines.push({
					startTime: current.start_time / 1000,
					text: current.text,
					id: i,
					endTime: duration
				});
			}
		}

		this.lines = lines;
	}

	reset(duration: number, raw_lines: LyricLine[]) {
		let lines: Line[] = [];

		for (let i = 0; i < raw_lines.length; i++) {
			const current = raw_lines[i] as LyricLine;
			const next = raw_lines[i + 1] as LyricLine | undefined;

			if (next) {
				lines.push({
					startTime: current.start_time / 1000,
					text: current.text,
					id: i,
					endTime: next.start_time / 1000
				});
			} else {
				lines.push({
					startTime: current.start_time / 1000,
					text: current.text,
					id: i,
					endTime: duration
				});
			}
		}

		this.lines = lines;
		this.currentActiveLines = [];
	}

	private set als(val: Line[]) {
		if (!eq(val, this.currentActiveLines)) {
			this.currentActiveLines = val;
			this.chs.forEach((f) => {
				f();
			});
		}
	}

	update(time: number) {
		this.als = this.lines.filter((line) => {
			return line.startTime <= time && line.endTime >= time;
		});
	}

	get activeLines(): Line[] {
		return this.currentActiveLines;
	}

	set oncuechange(fn: () => void) {
		this.chs.add(fn);
	}
}
