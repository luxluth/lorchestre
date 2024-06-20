import type { LayoutLoad } from './$types';
import { _init } from '$lib/i18n';
import { locale } from 'svelte-i18n';
import { invoke } from '@tauri-apps/api/core';
import type { Config, DefinedConfig } from '$lib/type';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ route }) => {
	const lang = await invoke<string>('locale');
	const config = await invoke<Config>('config');
	const default_config = await invoke<DefinedConfig>('default_config');
	console.log('[INFO::locale]', lang);
	_init(lang);
	locale.set(lang);

	return {
		config,
		default_config,
		route: route.id as string,
		lang,
		platform: await invoke<string>('platform')
	};
};
