import type { QueueTrack, RawTimestamp, Track } from '$lib/type';
import { getCurrentWindow, UserAttentionType } from '@tauri-apps/api/window';
import type AppConfig from '$lib/config.svelte';

export function clickOutside(element: Element, callbackFunction: () => void) {
	function onClick(event: MouseEvent) {
		if (!element.contains(event.target as Node)) {
			callbackFunction();
		}
	}

	document.body.addEventListener('click', onClick);

	return {
		update(newCallbackFunction: () => void) {
			callbackFunction = newCallbackFunction;
		},
		destroy() {
			document.body.removeEventListener('click', onClick);
		}
	};
}

export function unionToMap<V>(record: Array<[string, V]>): Map<string, V> {
	const map = new Map<string, V>();

	for (const [key, value] of record) {
		// Step 4: Add entries to the Map
		map.set(key, value);
	}

	return map;
}

function* SequenceGenerator(): Generator<number, number, unknown> {
	let i = 0;
	while (true) {
		yield i++;
	}
}

export function createSequenceGenerator() {
	const sequenceGenerator = SequenceGenerator();
	return () => sequenceGenerator.next().value;
}

export function isElementVisible(element: HTMLElement) {
	if (!element) {
		return false;
	}
	const rect = element.getBoundingClientRect();
	const style = window.getComputedStyle(element);

	const isInDOM = element.offsetParent !== null;
	const hasSize = rect.width > 0 && rect.height > 0;
	const isDisplayed = style.display !== 'none';
	const isVisible = style.visibility !== 'hidden' && style.opacity !== '0';
	const isInViewport =
		rect.top >= 0 &&
		rect.left >= 0 &&
		rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
		rect.right <= (window.innerWidth || document.documentElement.clientWidth);

	return isInDOM && hasSize && isDisplayed && isVisible && isInViewport;
}

export function formatTime(seconds: number) {
	if (isNaN(seconds)) {
		return '--:--';
	}
	if (seconds >= 60 * 60) {
		return new Date(seconds * 1000).toISOString().substring(11, 16);
	} else {
		return new Date(seconds * 1000).toISOString().substring(14, 19);
	}
}
export function getCoverUri(album_id: string, ext: String, config: AppConfig, size = -1) {
	const endpoint = config.getDaemonEndpoint();
	if (size > 0) {
		return `http://${endpoint}/cover/${album_id}${ext}?size=${size}x${size}`;
	} else {
		return `http://${endpoint}/cover/${album_id}${ext}`;
	}
}

export function getAudioUri(path: string, config: AppConfig) {
	const endpoint = config.getDaemonEndpoint();
	return `http://${endpoint}/audio?path=${path}`;
}

export function getLyricsUri(path: string, config: AppConfig) {
	const endpoint = config.getDaemonEndpoint();
	return `http://${endpoint}/lyrics?path=${path}`;
}

export function searchLyricsUri(path: string, config: AppConfig) {
	const endpoint = config.getDaemonEndpoint();
	return `http://${endpoint}/search/lyrics?path=${path}`;
}

export function setRandomId(track: Track): QueueTrack {
	return {
		...track,
		id: crypto.randomUUID()
	};
}

export function setTitle(text: string) {
	(async () => {
		await getCurrentWindow().setTitle(text);
	})();
}

export function requestUserAttention() {
	(async () => {
		console.warn('User attention requested');
		await getCurrentWindow().requestUserAttention(UserAttentionType.Critical);
	})();
}

export function getLNTime(line: RawTimestamp) {
	return (line.minutes * 60 + line.seconds) * 1000 + (line.millis ?? 0);
}

export function removeDuplicate<T>(elements: T[]): T[] {
	return elements.reduce((acc: T[], item) => {
		if (!acc.some((obj) => JSON.stringify(obj) === JSON.stringify(item))) {
			acc.push(item);
		}
		return acc;
	}, []);
}

export function sortTracksByDate(tracks: Track[]): Track[] {
	return tracks.slice().sort((a, b) => {
		return b.created_at - a.created_at;
	});
}
