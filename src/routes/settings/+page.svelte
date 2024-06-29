<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Themes from './Themes.svelte';
	import Lang from './Lang.svelte';
	import Refresh from './Refresh.svelte';
	import Blur from './Blur.svelte';
	import { setTitle } from '$lib/utils';

	import banner from '$lib/assets/banner-bw.svg?raw';
	import { browser } from '$app/environment';
	import { invoke } from '@tauri-apps/api/core';
	import { getAppConfig } from '$lib/config.svelte';

	// import Github from 'lucide-svelte/icons/git-branch';

	let appConf = getAppConfig();
	let version = $state('vx.x.x');

	if (browser) {
		(async () => {
			const v = await invoke('version');
			version = `v${v}`;
		})();
	}

	$effect(() => {
		setTitle(`${$_('settings').toLowerCase()} — L'orchestre`);
	});
</script>

<div class="settings ns">
	<h1>{$_('settings')}</h1>
	<p class="tips">{$_('settings_page.tip_top')}</p>
	<hr />

	<section id="block" class="ui-ux">
		<h2>{$_('settings_page.section_ui_ux.title')}</h2>
		<Lang {appConf} />
		<Themes {appConf} />
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
		<!-- <a target="_blank" href="https://github.com/luxluth/lorchestre" class=""><Github /></a> -->
	</div>
</div>

<style>
	.banner {
		margin-top: 8em;
		width: 100%;
		height: fit-content;
		color: var(--brand-color);
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
