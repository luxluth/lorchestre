import { createUrlFrom } from '$lib';
import { ToastKind, type AudioMedia, type ToastEvent } from './type';

export namespace toast {
	export const SHOW_EV = '__toast::show';
	export const CLOSE_EV = '__toast::close';

	export function show(
		message: string,
		kind: ToastKind = ToastKind.Message,
		title = '',
		timeout = -1
	) {
		let ev: CustomEvent<ToastEvent> = new CustomEvent(SHOW_EV, {
			detail: {
				kind,
				message,
				title,
				timeout
			}
		});
		document.dispatchEvent(ev);
	}

	export function close() {
		let ev: CustomEvent<ToastEvent> = new CustomEvent(CLOSE_EV, {
			detail: {
				kind: ToastKind.Close
			}
		});
		document.dispatchEvent(ev);
	}
}

export namespace player {
	export const PLAY_EV = '__player::play';

	export function play(media: AudioMedia) {
		let ev: CustomEvent<AudioMedia> = new CustomEvent(PLAY_EV, {
			detail: media
		});

		document.dispatchEvent(ev);
	}
}
