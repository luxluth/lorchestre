<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Themes from './Themes.svelte';
	import Lang from './Lang.svelte';
	import Refresh from './Refresh.svelte';
	import { getContext } from 'svelte';
	import type AppConfig from '$lib/config.svelte';
	import { setTitle } from '$lib/utils';

	let appConf = getContext<AppConfig>('appconf');

	$effect(() => {
		setTitle(`mu -- ${$_('settings').toLowerCase()}`);
	});
</script>

<h1>{$_('settings')}</h1>
<p class="tips">{$_('settings_page.tip_top')}</p>
<hr />

<section id="block" class="ui-ux">
	<h2>{$_('settings_page.section_ui_ux.title')}</h2>
	<Lang {appConf} />
	<Themes {appConf} />
</section>
<section id="block" class="ui-ux">
	<h2>Music Library</h2>
	<Refresh />
</section>

<style>
	section#block {
		padding-block: 1.5em;
	}

	h1 {
		padding-bottom: 0.2em;
	}

	hr {
		margin-top: 1.5em;
		opacity: 0.2;
	}

	p.tips {
		opacity: 0.5;
	}
</style>
