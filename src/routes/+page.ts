import type { Media } from '$lib/type';
import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load: PageLoad = async () => {
	return {
		media: await invoke<Media>('index')
	};
};
