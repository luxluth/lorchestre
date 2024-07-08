import { init, register } from 'svelte-i18n';

register('en', () => import('./en.json'));
register('fr', () => import('./fr.json'));
register('ja', () => import('./ja.json'));

export const _init = (locale: string) => {
	init({
		fallbackLocale: 'en',
		initialLocale: locale
	});
};
