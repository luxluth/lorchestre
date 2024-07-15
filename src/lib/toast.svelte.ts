import { getContext, setContext } from 'svelte';
import { ToastKind, type Toast } from './type';

const DEFAULT_TIMEOUT = 3500;

export default class ToastManager {
	innerId = 0;
	toasts: Array<Toast> = $state([]);
	private onCloseHandlers = new Set<(id: number) => void>();

	new(kind: ToastKind, message: string, title?: string, timeout = DEFAULT_TIMEOUT) {
		const id = ++this.innerId;
		if (kind === ToastKind.Loading) {
			this.toasts.push({
				id,
				kind,
				message,
				title,
				timeout: -1
			});
		} else {
			this.toasts.push({
				id,
				kind,
				message,
				title,
				timeout
			});

			setTimeout(() => {
				this.close(id);
			}, timeout);
		}

		return id;
	}

	set onClose(v: (id: number) => void) {
		this.onCloseHandlers.add(v);
	}

	close(id: number) {
		this.toasts = this.toasts.filter((t) => t.id !== id);
		this.onCloseHandlers.forEach((handler) => {
			handler(id);
		});
	}

	updateMessage(id: number, newMessage: string) {
		let toast = this.toasts.find((t) => t.id === id);
		if (toast) {
			toast.message = newMessage;
		}
	}
}

export const TOAST_SYMBOL = Symbol('TOAST_MANAGER');

export function setToastManager() {
	return setContext<ToastManager>(TOAST_SYMBOL, new ToastManager());
}

export function getToastManager() {
	return getContext<ReturnType<typeof setToastManager>>(TOAST_SYMBOL);
}
