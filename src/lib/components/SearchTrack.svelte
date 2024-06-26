<script lang="ts">
	import { goto } from '$app/navigation';
	import type AppConfig from '$lib/config.svelte';
	import type Ctx from '$lib/ctx.svelte';
	import type Manager from '$lib/manager.svelte';
	import {
		ContextMenuItemType,
		QueueAddMode,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { getCoverUri } from '$lib/utils';
	import Disc from 'lucide-svelte/icons/disc';
	import ListEnd from 'lucide-svelte/icons/list-end';
	import ListStart from 'lucide-svelte/icons/list-start';
	import Play from 'lucide-svelte/icons/play';
	import { _ } from 'svelte-i18n';

	let {
		track,
		config,
		manager,
		ctx
	}: { track: Track; config: AppConfig; manager: Manager; ctx: Ctx } = $props();

	async function play(track: Track) {
		await manager.play(track);
	}

	function showContext(e: ContextMenuEvent, track: Track) {
		const items: ContextMenuItem[] = [
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					await manager.play(track);
				},
				label: $_('songs_page.listen'),
				icon: Play
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					manager.addToQueue(track, QueueAddMode.Top);
				},
				label: $_('ctx.top_of_q'),
				icon: ListStart
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					manager.addToQueue(track);
				},
				label: $_('album.page.ctx.add_queue'),
				icon: ListEnd
			},
			{
				type: ContextMenuItemType.Separator
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					goto(`/album/${track.album_id}`);
				},
				label: $_('misc.goto_album_page'),
				icon: Disc
			}
		];
		ctx.x = e.x;
		ctx.y = e.y;
		ctx.items = items;
		ctx.visible = true;
	}
</script>

<div
	class="track"
	tabindex="0"
	role="button"
	oncontextmenu={(e) => {
		e.preventDefault();
		showContext(e, track);
	}}
	ondblclick={async () => {
		await play(track);
	}}
>
	<div class="cover">
		<img src={getCoverUri(track.album_id, track.cover_ext, config, 70)} alt="" />
	</div>
	<div class="details">
		<h3>{track.title}</h3>
		<p>{track.album_artist ?? track.artists[0]}</p>
	</div>
</div>

<style>
	.details {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	.details p {
		opacity: 0.5;
	}
	.track {
		display: grid;
		gap: 1em;
		grid-template-columns: auto 1fr;
		align-items: center;
		background: var(--highlight);
		width: 400px;

		padding: 0.5em;
		border-radius: 8px;
	}

	.track:active {
		transform: scale(0.98);
	}

	.cover {
		border-radius: 6px;
		overflow: hidden;
		justify-content: center;
		aspect-ratio: 1/1;
		height: 70px;
		width: 70px;
		display: flex;
	}
</style>
