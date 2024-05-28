import type { Album } from '$lib/type';
import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	let album = await invoke<Album>('get_album', { id: params.id });

	return {
		album
	};
};
