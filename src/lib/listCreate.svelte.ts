import { getContext, setContext } from 'svelte';
import type MediaState from './media.svelte';
import type AppConfig from './config.svelte';
import type ToastManager from './toast.svelte';
import { ToastKind } from './type';
import { goto } from '$app/navigation';

type CreateListPayload = {
	meta: Array<[string, string]>;
	tracks: string[];
};

export default class ListCreate {
	addedTracks: string[] = $state([]);
	Name: string = $state('');
	Description: string = $state('');
	xCoverPath: string = $state('');
	ImageData = $state('');
	media: MediaState;
	conf: AppConfig;
	tm: ToastManager;
	creating: boolean = $state(false);

	constructor(media: MediaState, conf: AppConfig, tm: ToastManager) {
		this.media = media;
		this.tm = tm;
		this.conf = conf;
		this.Name = `Playlist N°${media.playlists.length + 1}`;
	}

	async create() {
		const endpoint = this.conf.getDaemonEndpoint();
		let req_url = `http://${endpoint}/playlist/create`;
		if (this.addedTracks.length > 0 && this.Name.length > 0) {
			// TODO: localize
			let loadingToastId = this.tm.new(ToastKind.Loading, 'Creating playlist `' + this.Name + '`');
			const payload: CreateListPayload = {
				meta: [],
				tracks: []
			};

			payload.meta.push(['Name', this.Name]);
			payload.meta.push(['Description', this.Description]);
			if (this.xCoverPath.length > 0) {
				payload.meta.push(['x-CoverPath', this.xCoverPath]);
			}
			payload.tracks = [...this.addedTracks];

			await fetch(req_url, {
				method: 'POST',
				body: JSON.stringify(payload),
				headers: {
					'Content-Type': 'application/json'
				}
			})
				.then(async (response) => {
					this.tm.close(loadingToastId);
					if (response.ok) {
						const p = (await response.json()) as { path: string };
						this.addedTracks = [];
						this.Name = '';
						this.Description = '';
						this.xCoverPath = '';
						this.ImageData = '';

						// TODO: localize
						this.tm.new(ToastKind.Simple, 'Playlist created with success');
						goto(`/list/${p.path}`);
					} else {
						// TODO: localize
						this.tm.new(
							ToastKind.Error,
							`Unable to create a new playlist with the following error: ${await response.text()}`
						);
					}
				})
				.catch((error) => {
					this.tm.close(loadingToastId);
					// TODO: localize
					this.tm.new(
						ToastKind.Error,
						`Unable to create a new playlist with the following error: ${error}`
					);
				})
				.finally(() => {
					this.tm.close(loadingToastId);
					this.creating = false;
				});
		}
	}
}

export const LS_SYMBOL = Symbol('LS_CREATE');

export function setListCreator(media: MediaState, conf: AppConfig, tm: ToastManager) {
	return setContext<ListCreate>(LS_SYMBOL, new ListCreate(media, conf, tm));
}

export function getListCreator() {
	return getContext<ReturnType<typeof setListCreator>>(LS_SYMBOL);
}
