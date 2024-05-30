import type { ContextMenuItem } from './type';

export default class Ctx {
	x = $state(0);
	y = $state(0);
	visible = $state(false);
	data?: any = $state();
	items: ContextMenuItem[] = $state([]);

	constructor() {}
}
