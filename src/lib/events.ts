import { ToastKind, type Track, type PlayerDispatch, type ToastEvent } from './type';

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
	export const PLAY_AT = '__player::play_at';
	export const PLAYER_DISPATCH = '__player::dispatch';
	export const PLAYER_ACTIVATE = '__player::activate';

	export function play(track: Track) {
		let ev: CustomEvent<Track> = new CustomEvent(PLAY_EV, {
			detail: track
		});

		document.dispatchEvent(ev);
	}

	export function setTimeTo(time: number) {
		let ev: CustomEvent<number> = new CustomEvent(PLAY_AT, {
			detail: time
		});

		document.dispatchEvent(ev);
	}

	export function dispatch(detail: PlayerDispatch) {
		let ev: CustomEvent<PlayerDispatch> = new CustomEvent(PLAYER_DISPATCH, {
			detail
		});

		document.dispatchEvent(ev);
	}

	export function activate() {
		let ev: CustomEvent = new CustomEvent(PLAYER_ACTIVATE);

		document.dispatchEvent(ev);
	}
}
