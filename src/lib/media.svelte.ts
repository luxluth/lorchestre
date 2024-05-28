import type { Media } from './type';

export default function createMediaState(initial: Media) {
	let albums = $state(initial.albums);
	return {
		get albums() {
			return albums;
		},
		set albums(v) {
			albums = v;
		}
	};
}
