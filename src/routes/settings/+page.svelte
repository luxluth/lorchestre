<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Lang from './Lang.svelte';
	import Refresh from './Refresh.svelte';
	import Blur from './Blur.svelte';
	import { setTitle } from '$lib/utils';

	import banner from '$lib/assets/banner-bw.svg?raw';
	import { browser } from '$app/environment';
	import { invoke } from '@tauri-apps/api/core';
	import { getAppConfig } from '$lib/config.svelte';

	import Github from 'lucide-svelte/icons/git-branch';
	import { getNav } from '$lib/nav.svelte';

	let appConf = getAppConfig();
	let version = $state('vx.x.x');
	let nav = getNav();

	if (browser) {
		(async () => {
			const v = await invoke('version');
			version = `v${v}`;
		})();
	}

	$effect(() => {
		setTitle(`${$_('settings').toLowerCase()} — L'orchestre`);
		nav.pageName = $_('settings');
	});
</script>

<div class="settings ns">
	<h1>{$_('settings')}</h1>
	<p class="tips">{$_('settings_page.tip_top')}</p>
	<hr />

	<section id="block" class="ui-ux">
		<h2>{$_('settings_page.section_ui_ux.title')}</h2>
		<Lang {appConf} />
		<Blur {appConf} />
	</section>
	<hr />
	<section id="block" class="mlib">
		<h2>{$_('settings_page.section_mlib.title')}</h2>
		<Refresh />
	</section>
	<div class="banner">
		{@html banner}
		<code>{version}</code>
		<a class="btn" target="_blank" href="https://github.com/luxluth/lorchestre"
			><Github size={14} /> Star On Github</a
		>
	</div>
</div>

<style>
	.settings {
		padding-bottom: 4em;
	}

	a {
		text-decoration: none;
		color: inherit;
		margin-top: 1em;
	}
	.banner {
		margin-top: 8em;
		width: 100%;
		height: fit-content;
		color: var(--fg);
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
	}

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
