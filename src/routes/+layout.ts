// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
import { invoke } from '@tauri-apps/api/core';
import type { LayoutLoad } from './$types';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ route }) => {
	return {
		route: route.id as string,
		platform: await invoke<string>('platform')
	};
};
