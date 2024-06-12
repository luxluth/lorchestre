import type { Album } from './type';

export default class AlbumPageData {
	activeAlbum = $state<Album>();
	constructor() {}
}
