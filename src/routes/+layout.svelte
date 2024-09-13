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
	import WindowControls from '$lib/components/WindowControls.svelte';
	import StartingScreen from './StartingScreen.svelte';
	import { setPageScroll } from '$lib/pageScroll.svelte';
	import { setNav } from '$lib/nav.svelte';
	import { fly } from 'svelte/transition';

	import { ScrollArea } from 'bits-ui';
	import { navigating } from '$app/stores';

	// import { invoke } from '@tauri-apps/api/core';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();

	let first_run = $state(data.app_info.first_run);

	const tm = setToastManager();
	const evc = setEvc();

	let p = setPage();
	let conf = setAppConfig(data.config, data.default_config);
	let search = setSearch();
	let media = setMedia(search);

	const manager = setManager();
	setCmds();
	setCtx();
	setLrc(conf, tm, evc);
	setFilter();
	setList();
	setNav();
	const ps = setPageScroll();

	onMount(() => {
		if (!dev) {
			window.addEventListener('contextmenu', (e) => e.preventDefault());
		}
		(async () => {
			if (!media.loaded) {
				await media.load();
			}
		})();
	});

	$effect(() => {
		p.currentAddr = $page.url.pathname;
	});
</script>

<div class="layout" class:first_run>
	{#if !first_run}
		<section class="__content" class:hide_scroll={manager.isPlayerActive?.()}>
			<main
				id="__main__"
				onscroll={(e) => {
					ps.save(e, $page.url.pathname);
				}}
			>
				{#key data.url}
					{#if media.loaded}
						{@render children()}
					{/if}
				{/key}
			</main>
			<Navigation />
			<MiniPlayer />
		</section>
	{:else}
		<FirstRun bind:first_run />
	{/if}
</div>
{#if !first_run}
	<ContextMenu />
	<Player />
{/if}

{#if !first_run && !media.loaded}
	<StartingScreen />
{/if}

<style>
	.layout,
	main {
		height: 100%;
	}
	.__content {
		position: relative;
		padding: 3vw;
		height: 100%;
	}

	.hide_scroll {
		overflow: hidden;
	}

	/* [data-transition] { */
	/* 	height: 100%; */
	/* } */
</style>
