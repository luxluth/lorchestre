import type { PageLoad } from './$types';
import type { Album } from '$lib/type';
import { invoke } from '@tauri-apps/api/core';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params, fetch }) => {
	let endpoint = await invoke('daemon_endpoint');
	let req = await fetch(`http://${endpoint}/album/${params.id}`);
	if (req.ok) {
		return {
			album: (await req.json()) as Album
		};
	}
	return {
		error: await req.text()
	};
};
