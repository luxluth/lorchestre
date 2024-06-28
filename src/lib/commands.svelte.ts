import { getContext, setContext } from 'svelte';

export default class Cmds {
	lrc = $state(false);
	queue = $state(false);

	constructor() {}
}

export const CMDS_SYMBOL = Symbol('CMDS');

export function setCmds() {
	return setContext<Cmds>(CMDS_SYMBOL, new Cmds());
}

export function getCmds() {
	return getContext<ReturnType<typeof setCmds>>(CMDS_SYMBOL);
}
