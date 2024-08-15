<script lang="ts">
	import type List from '$lib/playlist.svelte';
	import type { Playlist } from '$lib/type';
	import ListIcon from 'lucide-svelte/icons/list';
	import { _ } from 'svelte-i18n';

	let { playlist, list }: { playlist: Playlist; list: List } = $props();
</script>

<a
	class="playlist"
	href="/list/{playlist.path_base64}"
	onclick={() => {
		list.activeList = playlist.path_base64;
	}}
>
	<div class="holder">
		<ListIcon size={'2em'} />
	</div>
	<div class="details">
		<h3>{playlist.metadata['Name'] ?? '+£@&0m'}</h3>
		<p>
			{playlist.tracks.length}
			{playlist.tracks.length > 1 ? $_('stats_page.songs') : $_('stats_page.song')}
		</p>
	</div>
</a>

<style>
	a {
		text-decoration: none;
	}
	.playlist {
		width: fit-content;
		display: flex;
		align-items: flex-end;
		padding: 1em;
		border-radius: 12px;
		gap: 1em;
		background-color: var(--highlight);
	}

	.holder {
		background: var(--fg);
		color: var(--bg);
		width: 150px;
		height: 150px;
		display: flex;
		border-radius: 8px;
		justify-content: center;
		align-items: center;
	}

	.details {
		color: var(--fg);
	}

	.details p {
		opacity: 0.7;
	}
</style>
