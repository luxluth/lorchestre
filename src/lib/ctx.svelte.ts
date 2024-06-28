import { getContext, setContext } from 'svelte';
import type { ContextMenuItem } from './type';

export default class Ctx {
	x = $state(0);
	y = $state(0);
	visible = $state(false);
	data?: any = $state();
	items: ContextMenuItem[] = $state([]);

	constructor() {}
}

export const CTX_SYMBOL = Symbol('CTX');

export function setCtx() {
	return setContext<Ctx>(CTX_SYMBOL, new Ctx());
}

export function getCtx() {
	return getContext<ReturnType<typeof setCtx>>(CTX_SYMBOL);
}
