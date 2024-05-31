import { invoke } from '@tauri-apps/api/core';
import type { PageLoad } from './$types';
import type { Album } from '$lib/type';

export const csr = true;
export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	const album = await invoke<Album | undefined>('get_album', { id: params.id });
	return {
		album
	};
};
