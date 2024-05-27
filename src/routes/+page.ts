import { invoke } from '@tauri-apps/api/core';
import type { PageLoad } from './$types';
import type { Media } from '$lib/type';

export const load: PageLoad = async () => {
	return {
		media: await invoke<Media>('index')
	};
};
