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
	import type { Snippet } from 'svelte';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();
</script>

<div class="layout">
	<section class="__navigation">
		<Navigation pathId={data.route as string} />
	</section>
	<section class="__content">
		<header>
			<MiniPlayer />
		</header>
		<main>
			{@render children()}
		</main>
	</section>
</div>
<Toast />
<Player />

<style>
	/* .layout { */
	/* 	padding-inline: 3em; */
	/* 	padding-block: 3em; */
	/* 	padding-top: 0; */
	/* } */

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
		background: var(--bg);
	}

	main {
		padding-block: 8em;
		padding-inline: 3em;
	}
</style>
