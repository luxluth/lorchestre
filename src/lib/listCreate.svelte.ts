import { getContext, setContext } from 'svelte';
import type MediaState from './media.svelte';
import type AppConfig from './config.svelte';
import type ToastManager from './toast.svelte';
import { ToastKind } from './type';
import { goto } from '$app/navigation';
import { _ as loc } from 'svelte-i18n';
import { get } from 'svelte/store';

export type ListPayload = {
	meta: Array<[string, string]>;
	tracks: string[];
};

const _ = (key: string) => get(loc)(key);
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
		this.Name = `${_('playlist')} #${media.playlists.length + 1}`;
	}

	async create() {
		const endpoint = this.conf.getDaemonEndpoint();
		let req_url = `http://${endpoint}/playlist/create`;
		if (this.addedTracks.length > 0 && this.Name.length > 0) {
			let loadingToastId = this.tm.new(
				ToastKind.Loading,
				`${_('playlist_create_page.creating_message')} \`` + this.Name + '`'
			);
			const payload: ListPayload = {
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

						this.tm.new(ToastKind.Simple, _('playlist_create_page.success_creation'));
						await goto(`/list/${p.path}`);
					} else {
						this.tm.new(
							ToastKind.Error,
							`${_('playlist_create_page.failed_creation')}: ${await response.text()}`
						);
					}
				})
				.catch((error) => {
					this.tm.close(loadingToastId);
					this.tm.new(ToastKind.Error, `${_('playlist_create_page.failed_creation')}: ${error}`);
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
