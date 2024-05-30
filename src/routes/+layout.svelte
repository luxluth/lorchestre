<script lang="ts">
	import '@fontsource-variable/noto-sans-kr';
	import '@fontsource-variable/noto-sans-jp';
	import '@fontsource-variable/noto-sans-sc';
	import '@fontsource-variable/readex-pro';
	import '../styles/global.css';

	import Toast from './Toast.svelte';
	import Player from './Player.svelte';
	import MiniPlayer from '$lib/components/MiniPlayer.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import type { LayoutData } from './$types';
	import { setContext, type Snippet } from 'svelte';
	import Manager from '$lib/manager.svelte';
	import Ctx from '$lib/ctx.svelte';
	import ContextMenu from './ContextMenu.svelte';
	import Cmds from '$lib/commands.svelte';
	import Commands from '$lib/components/Commands.svelte';
	import Queue from '$lib/components/Queue.svelte';
	import Lrc from '$lib/components/Lrc.svelte';
	import LrcManager from '$lib/lrc.svelte';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();
	setContext<Manager>('manager', new Manager());
	setContext<Ctx>('ctx', new Ctx());
	setContext<Cmds>('cmds', new Cmds());
	setContext<LrcManager>('lm', new LrcManager(0, []));
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
		width: 300px;
		flex-shrink: 0; /* Prevents shrinking of the navigation section */
		border-right: 1px solid rgba(100, 100, 100, 0.18);
		background: var(--bg);
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
		z-index: 10;
		width: 100%;
		display: grid;
		grid-template-columns: 3fr 1fr;
		gap: 2em;
		border-bottom: 1px solid rgba(100, 100, 100, 0.18);
	}

	main {
		padding-block: 8em;
		padding-inline: 3em;
	}
</style>
