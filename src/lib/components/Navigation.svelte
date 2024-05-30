<script lang="ts">
	import { GalleryHorizontalEnd, Clock, Settings } from 'lucide-svelte';
	let { pathId, platform }: { pathId: string; platform: string } = $props();
</script>

<div class="nav ns" style="--top-by: {platform === 'macos' ? '3em' : '1em'}">
	<section class="search">
		<input type="search" name="search" placeholder="Recherche" />
	</section>
	<section>
		<h4>Bibliothèque</h4>
		<div class="links">
			<a href="/recently-added" class:active={pathId == '/recently-added'}>
				<Clock size={'1em'} />
				Ajouts récents</a
			>
			<a href="/" class:active={pathId == '/'}>
				<GalleryHorizontalEnd size={'1em'} />
				Albums</a
			>
		</div>
	</section>
	<section class="settings">
		<div class="links">
			<a href="/settings" class:active={pathId == '/settings'}>
				<Settings size={'1em'} />
				Réglages</a
			>
		</div>
	</section>
</div>

<style>
	section.settings {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		justify-content: flex-end;
	}

	input[type='search'] {
		-webkit-appearance: none;
		appearance: none;
		padding: 0.5em;
		border-radius: 4px;
		border: 0px;
		background: rgba(100, 100, 100, 0.18);
		color: var(--fg);
		width: 100%;
	}

	.nav {
		padding-block: 2em;
		padding-top: var(--top-by);
		padding-inline: 1em;
		display: flex;
		flex-direction: column;
		gap: 1.5em;
		height: 100%;
		overflow: auto;
	}
	h4 {
		opacity: 0.5;
		padding-bottom: 0.5em;
	}

	section div.links {
		display: flex;
		flex-direction: column;
		gap: 0.3em;
	}

	section div.links a {
		color: var(--fg);
		opacity: 0.7;
		font-weight: bold;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 0.5em;
		padding-inline: 0.5em;
		padding-block: 0.5em;
		border-radius: 6px;
		transition: all ease-in-out 0.1s;
	}

	section div.links a:hover {
		opacity: 0.9;
	}

	section div.links a:active {
		transform: scale(0.99);
	}

	section div.links a.active {
		background: rgba(100, 100, 100, 0.18);
		opacity: 1;
	}
</style>
