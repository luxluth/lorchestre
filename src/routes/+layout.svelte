<script lang="ts">
	import './fonts';
	import '../styles/global.css';

	import ToastDisplay from './ToastDisplay.svelte';
	import Player from './Player.svelte';
	import MiniPlayer from '$lib/components/MiniPlayer.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import type { LayoutData } from './$types';
	import { onMount, type Snippet } from 'svelte';
	import { setManager } from '$lib/manager.svelte';
	import { setCtx } from '$lib/ctx.svelte';
	import ContextMenu from './ContextMenu.svelte';
	import { setCmds } from '$lib/commands.svelte';
	import Commands from '$lib/components/Commands.svelte';
	import Queue from '$lib/components/Queue.svelte';
	import Lrc from '$lib/components/Lrc.svelte';
	import { setLrc } from '$lib/lrc.svelte';
	import { setMedia } from '$lib/media.svelte';
	import { setFilter } from '$lib/filterq.svelte';
	import { setList } from '$lib/playlist.svelte';
	import { setAppConfig } from '$lib/config.svelte';
	import { setSearch } from '$lib/search.svelte';
	import { browser, dev } from '$app/environment';
	import { setPage } from '$lib/page.svelte';
	import { page } from '$app/stores';
	import FirstRun from './FirstRun.svelte';
	import { setToastManager } from '$lib/toast.svelte';
	import { setEvc } from '$lib/editviewController.svelte';
	import EditView from './EditView.svelte';
	// import { invoke } from '@tauri-apps/api/core';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();

	let first_run = $state(data.app_info.first_run);

	const tm = setToastManager();
	const evc = setEvc();

	let p = setPage();
	let conf = setAppConfig(data.config, data.default_config);
	let search = setSearch();
	let media = setMedia(search);

	setManager();
	setCmds();
	setCtx();
	setLrc(conf, tm, evc);
	setFilter();
	setList(media);

	if (browser) {
		if (!dev) {
			window.addEventListener('contextmenu', (e) => e.preventDefault());
		}
		document.body.setAttribute(
			'data-theme',
			conf.config?.global?.theme ?? conf.defaults.global.theme
		);
	}

	onMount(() => {
		(async () => {
			if (!media.loaded) {
				await media.load();
			}
			// await invoke('plugin:decorum|show_snap_overlay');
		})();
	});

	$effect(() => {
		p.currentAddr = $page.url.pathname;
	});
</script>

<div class="layout" class:first_run>
	{#if !first_run}
		<section class="__navigation">
			<Navigation pathId={data.route as string} platform={data.platform} />
		</section>
		<section class="__content">
			<header class="glass">
				<MiniPlayer />
				<Commands />
			</header>
			<main>
				{@render children()}
				<Queue />
				<Lrc />
			</main>
		</section>
	{:else}
		<FirstRun bind:first_run />
	{/if}
</div>
{#if !first_run}
	<ToastDisplay />
	<Player />
	<EditView />
	<ContextMenu />
{/if}

<style>
	.layout {
		display: flex;
		height: 100vh;
	}

	.layout.first_run {
		flex-direction: column;
		overflow: hidden;
		align-items: center;
		justify-content: center;
		position: relative;
		gap: 1em;
		background-color: var(--brand-color);
	}

	.__navigation {
		width: 270px;
		flex-shrink: 0; /* Prevents shrinking of the navigation section */
		border-right: 1px solid rgba(100, 100, 100, 0.18);
		background: var(--bg);
		z-index: var(--header-z-index);
	}

	.__content {
		flex-grow: 1; /* Takes up the remaining space */
		display: flex;
		flex-direction: column;
	}

	main {
		position: relative;
		flex-grow: 1; /* Main content takes up the remaining space in the column */
		padding: 20px; /* Optional: padding */
		overflow-y: auto; /* Ensures main content is scrollable if it overflows */
	}

	header {
		position: fixed;
		z-index: var(--header-z-index);
		width: calc(100% - 270px);
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1em;
		height: fit-content;
		border-bottom: 1px solid rgba(100, 100, 100, 0.18);
	}

	main {
		padding-block: 8em;
		padding-inline: 3em;
	}
</style>
