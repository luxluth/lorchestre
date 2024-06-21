import type { QueueTrack, Track } from '$lib/type';
import { v4 as uuidv4 } from 'uuid';
import { getCurrent } from '@tauri-apps/api/window';
import { getContext } from 'svelte';
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
export function getCoverUri(album_id: string, ext: String) {
	let config = getContext<AppConfig>('appconf');
	const endpoint = config.getMUDEndpoint();
	return `http://${endpoint}/cover/${album_id}${ext}`;
}

export function getAudioUri(id: string) {
	let config = getContext<AppConfig>('appconf');
	const endpoint = config.getMUDEndpoint();
	return `http://${endpoint}/audio/${id}`;
}

export function toQueueTrack(track: Track): QueueTrack {
	return {
		...track,
		uuid: uuidv4()
	};
}

export function setTitle(text: string) {
	(async () => {
		await getCurrent().setTitle(text);
	})();
}
