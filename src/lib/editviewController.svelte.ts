import { getContext, setContext, type Component } from 'svelte';

export default class EditViewController {
	visible = $state(false);
	data: any = null;
	componnentToRender: Component<{ onClose: () => void; data: any }> | null = null;

	show(comp: Component<{ onClose: () => void; data: any }>, data: any) {
		this.componnentToRender = comp;
		this.data = data;
		this.visible = true;
	}
}

export const EVC_SYMBOL = Symbol('EDITORVIEWCONTROLLER');

export function setEvc() {
	return setContext<EditViewController>(EVC_SYMBOL, new EditViewController());
}

export function getEvc() {
	return getContext<ReturnType<typeof setEvc>>(EVC_SYMBOL);
}
