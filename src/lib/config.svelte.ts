import { invoke } from '@tauri-apps/api/core';
import type { Config, DefinedConfig, Theme } from './type';

export default class AppConfig {
	//@ts-ignore
	config: Config = $state();
	//@ts-ignore
	defaults: DefinedConfig = $state();

	constructor(config: Config, defaults: DefinedConfig) {
		this.config = config;
		this.defaults = defaults;
	}

	async setLocale(l: string) {
		this.config = await invoke<Config>('set_locale', { locale: l });
	}

	async setTheme(theme: Theme) {
		this.config = await invoke<Config>('set_theme', { theme });
		document.body.setAttribute('data-theme', theme);
	}
}
