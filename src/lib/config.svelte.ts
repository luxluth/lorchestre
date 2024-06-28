import { invoke } from '@tauri-apps/api/core';
import type { Config, DefinedConfig, Theme } from './type';
import { getContext, setContext } from 'svelte';

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

	async setBlurTo(state: boolean) {
		this.config = await invoke<Config>('set_blur', { state });
	}

	getDaemonEndpoint() {
		return (
			`${this.config.network?.host ?? this.defaults.network.host}` +
			':' +
			`${this.config.network?.port ?? this.defaults.network.port}`
		);
	}
}

export const CONF_SYMBOL = Symbol('APPCONF');

export function setAppConfig(config: Config, defaults: DefinedConfig) {
	return setContext<AppConfig>(CONF_SYMBOL, new AppConfig(config, defaults));
}

export function getAppConfig() {
	return getContext<ReturnType<typeof setAppConfig>>(CONF_SYMBOL);
}
