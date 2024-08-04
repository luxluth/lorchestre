import type { PageLoad } from './$types';
import type { Playlist } from '$lib/type';
import { invoke } from '@tauri-apps/api/core';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params, fetch }) => {
	let endpoint = await invoke('daemon_endpoint');
	let req = await fetch(`http://${endpoint}/playlist/${params.id}`);
	if (req.ok) {
		return {
			list: (await req.json()) as Playlist
		};
	}
	return {
		error: await req.text()
	};
};
