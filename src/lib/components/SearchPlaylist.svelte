<script lang="ts">
	import { goto } from '$app/navigation';
	import type List from '$lib/playlist.svelte';
	import type { Playlist } from '$lib/type';
	import ListIcon from 'lucide-svelte/icons/list';
	import { _ } from 'svelte-i18n';

	let { playlist, list }: { playlist: Playlist; list: List } = $props();
	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}
</script>

<div
	class="playlist"
	role="button"
	tabindex="0"
	onkeydown={(e) => {
		let key = e.key.toLowerCase();
		if (key === ' ' || key === 'enter') {
			list.activeList = playlist;
			goto('/list');
		}
	}}
	onclick={() => {
		list.activeList = playlist;
		goto('/list');
	}}
>
	<div class="holder">
		<ListIcon size={'2em'} />
	</div>
	<div class="details">
		<h3>{playlist.name}</h3>
		<p>
			{playlist.tracks.length}
			{playlist.tracks.length > 1 ? $_('stats_page.songs') : $_('stats_page.song')}
		</p>
	</div>
</div>

<style>
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

	.details p {
		opacity: 0.7;
	}
</style>
