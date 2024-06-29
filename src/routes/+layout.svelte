<script lang="ts">
	import '@fontsource-variable/noto-sans-kr';
	import '@fontsource-variable/noto-sans-jp';
	import '@fontsource-variable/noto-serif-jp';
	import '@fontsource-variable/noto-sans-sc';

	import '@fontsource/poppins/100.css';
	import '@fontsource/poppins/200.css';
	import '@fontsource/poppins/300.css';
	import '@fontsource/poppins/400.css';
	import '@fontsource/poppins/500.css';
	import '@fontsource/poppins/600.css';
	import '@fontsource/poppins/700.css';
	import '@fontsource/poppins/800.css';
	import '@fontsource/poppins/900.css';
	// import '@fontsource/poppins/100-italic.css';
	// import '@fontsource/poppins/200-italic.css';
	// import '@fontsource/poppins/300-italic.css';
	// import '@fontsource/poppins/400-italic.css';
	// import '@fontsource/poppins/500-italic.css';
	// import '@fontsource/poppins/600-italic.css';
	// import '@fontsource/poppins/700-italic.css';
	// import '@fontsource/poppins/800-italic.css';
	// import '@fontsource/poppins/900-italic.css';

	import '@fontsource-variable/inter';

	import '@fontsource/calistoga';
	import '@fontsource/ibm-plex-mono';
	import '../styles/global.css';

	import Toast from './Toast.svelte';
	import Player from './Player.svelte';
	import MiniPlayer from '$lib/components/MiniPlayer.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import type { LayoutData } from './$types';
	import { type Snippet } from 'svelte';
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
	import { page, navigating } from '$app/stores';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();

	let p = setPage();
	let conf = setAppConfig(data.config, data.default_config);
	setManager();
	setCmds();
	setCtx();
	setLrc();
	setFilter();
	let search = setSearch();
	let media = setMedia(search);
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

	$effect(() => {
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

<div class="layout">
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
</div>
<Toast />
<Player />
<ContextMenu />

<style>
	.layout {
		display: flex;
		height: 100vh;
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
