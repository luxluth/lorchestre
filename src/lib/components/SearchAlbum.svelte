<script lang="ts">
	import { goto } from '$app/navigation';
	import type AppConfig from '$lib/config.svelte';
	import type Ctx from '$lib/ctx.svelte';
	import type Manager from '$lib/manager.svelte';
	import type MediaState from '$lib/media.svelte';
	import {
		ContextMenuItemType,
		QueueAddMode,
		type Album,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { getCoverUri } from '$lib/utils';
	import ListEnd from 'lucide-svelte/icons/list-end';
	import ListStart from 'lucide-svelte/icons/list-start';
	import Play from 'lucide-svelte/icons/play';
	import { _ } from 'svelte-i18n';

	let {
		album,
		config,
		media,
		ctx,
		manager
	}: { album: Album; config: AppConfig; media: MediaState; ctx: Ctx; manager: Manager } = $props();

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function showContext(e: ContextMenuEvent, tracks: Track[]) {
		let sortedTracks = sortTracks(tracks);
		const items: ContextMenuItem[] = [
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					let firstTrack = sortedTracks.shift() as Track;
					manager.queue = [];
					manager.addManyToQueue(sortedTracks);
					await manager.play(firstTrack);
				},
				label: $_('ctx.play'),
				icon: Play
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					manager.addManyToQueue(sortedTracks, QueueAddMode.Top);
				},
				label: $_('ctx.top_of_q'),
				icon: ListStart
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					manager.addManyToQueue(sortedTracks);
				},
				label: $_('ctx.inqueue'),
				icon: ListEnd
			}
		];
		ctx.x = e.x;
		ctx.y = e.y;
		ctx.items = items;
		ctx.visible = true;
	}

	function getTracks(album: Album) {
		let tracks = [];
		for (const path of album.tracks) {
			let track = media.getTrack(path);
			if (track) tracks.push(track);
		}

		return tracks;
	}
</script>

<div
	class="album"
	role="button"
	tabindex="0"
	onkeydown={(e) => {
		let key = e.key.toLowerCase();
		if (key === ' ' || key === 'enter') goto('/album/' + album.id);
	}}
	onclick={() => {
		goto('/album/' + album.id);
	}}
	oncontextmenu={(e) => {
		e.preventDefault();
		showContext(e, getTracks(album));
	}}
>
	<div class="cover">
		<img
			src={getCoverUri(album.id, media.getTrack(album.tracks[0])?.cover_ext ?? '.png', config, 400)}
			alt=""
		/>
		<div class="details">
			<div class="data">
				<h3>{album.name}</h3>
				<p>{album.artist}</p>
			</div>
		</div>
	</div>
</div>

<style>
	.data {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		justify-content: flex-end;
		padding: 1em;
		gap: 0.1em;
		color: white;
	}

	.details {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		left: 0;
		background-color: rgba(0, 0, 0, 0.5);
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
	}

	.album:hover .details {
		opacity: 1;
	}

	.album {
		gap: 1em;
		background: var(--highlight);
		width: fit-content;

		padding: 1em;
		border-radius: 10px;
	}

	.cover {
		position: relative;
		border-radius: 8px;
		overflow: hidden;
		justify-content: center;
		aspect-ratio: 1/1;
		height: 400px;
		width: 400px;
		display: flex;
	}

	.cover img {
		width: 100%;
		height: 100%;
	}
</style>
