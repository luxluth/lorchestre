import FilterQuery from './filterq.svelte';
import { type Playlist } from './type';

export default class List {
	activeList = $state<Playlist>();
	filters = new FilterQuery();
	constructor() {}
}
