<script lang="ts">
	import { GalleryHorizontalEnd, Clock } from 'lucide-svelte';

	let { pathId }: { pathId: string } = $props();
	console.log(pathId);
</script>

<div class="nav ns">
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
</div>

<style>
	.nav {
		padding-block: 2em;
		padding-inline: 1em;
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
		padding-block: 0.3em;
		border-radius: 8px;
		transition: all ease-in-out 0.1s;
	}

	section div.links a:hover {
		opacity: 0.9;
	}

	section div.links a:active {
		transform: scale(0.99);
	}

	section div.links a.active {
		background: rgba(45, 45, 45, 0.25);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		opacity: 1;
	}
</style>
