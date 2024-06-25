<script lang="ts">
	import { goto } from '$app/navigation';
	import type Ctx from '$lib/ctx.svelte';
	import type Manager from '$lib/manager.svelte';
	import {
		ContextMenuItemType,
		QueueAddMode,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { formatTime } from '$lib/utils';
	import Disc from 'lucide-svelte/icons/disc';
	import ListEnd from 'lucide-svelte/icons/list-end';
	import ListStart from 'lucide-svelte/icons/list-start';
	import Play from 'lucide-svelte/icons/play';
	import { _ } from 'svelte-i18n';

	function trim(text: string, len = 20) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

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

	let {
		song,
		i,
		ctx,
		manager,
		searchq = $bindable()
	}: { song: Track; i: number; ctx: Ctx; manager: Manager; searchq: string } = $props();

	function replaceTextWithMarker(text: string) {
		const regex = new RegExp(searchq, 'gi');
		return text.replaceAll(regex, (match) => `<mark>${match}</mark>`);
	}
</script>

<div
	class="track ns"
	role="presentation"
	draggable
	oncontextmenu={(e) => {
		e.preventDefault();
		showContext(e, song);
	}}
	ondblclick={async () => {
		await manager.play(song);
	}}
>
	<div
		class="count"
		class:iscurrent={manager.currentTrack?.file_path === song.file_path}
		aria-colindex="1"
		role="gridcell"
	>
		{#if manager.currentTrack?.file_path === song.file_path}
			<div class="playing" class:paused={manager.paused}>
				<div class="bar"></div>
				<div class="bar"></div>
				<div class="bar"></div>
			</div>
		{:else}
			<div class="x">
				{i + 1}
			</div>
		{/if}
		<button
			class="play"
			onclick={async () => {
				await play(song);
			}}
		>
			<Play fill={'var(--fg)'} size={'16px'} />
		</button>
	</div>
	<div class="title-part" aria-colindex="2" role="gridcell">
		<h4 class="title">{@html replaceTextWithMarker(trim(song.title, 30))}</h4>
	</div>
	<div class="artist" aria-colindex="3" role="gridcell">
		{@html replaceTextWithMarker(trim(song.artists.join(', ')))}
	</div>
	<div class="album" aria-colindex="4" role="gridcell">
		<a href="/album/{song.album_id}">{@html replaceTextWithMarker(trim(song.album))}</a>
	</div>
	<div class="duration" aria-colindex="5" role="gridcell">{formatTime(song.duration)}</div>
</div>

<style>
	:global(mark) {
		background-color: #1e90ff;
		color: var(--bg);
		border-radius: 3px;
	}

	.track {
		display: grid;
		grid-template-columns:
			[index] var(--tracklist-index-column-width, 16px)
			[first] minmax(120px, var(--col1, 6fr))
			[var1] minmax(120px, var(--col2, 4fr)) [var2] minmax(120px, var(--col3, 3fr)) [last] minmax(120px, var(--col4, 1fr));
		align-items: center;
		grid-gap: 16px;
		padding-block: 4px;
		padding-inline: 20px;
		height: 2.5em;
		border-radius: 4px;
	}

	.track:hover .count {
		opacity: 1;
	}

	.track:hover .count .x {
		display: none;
	}

	.track:hover .count .play {
		display: block;
	}

	.track .count .play {
		background: none;
		border: none;
	}

	.count .play {
		display: none;
	}

	.count {
		display: flex;
		justify-content: center;
		text-align: center;
		justify-self: end;
		opacity: 0.5;
		width: 100%;
	}

	.iscurrent {
		opacity: 1;
	}

	.track:hover .count .playing {
		display: none;
	}

	.playing {
		display: flex;
		height: 100%;
		gap: 2px;
		justify-content: center;
		align-items: center;
	}

	.playing .bar {
		width: 2px;
		height: 2px;
		border-radius: 1px;
		background-color: var(--brand-color);
	}
	.bar:first-child {
		animation-delay: 300ms;
	}

	.bar:last-child {
		animation-delay: 500ms;
	}

	.playing:not(.paused) .bar {
		animation-name: baranim;
		animation-duration: 500ms;
		animation-timing-function: cubic-bezier(0.455, 0.03, 0.515, 0.955);
		animation-iteration-count: infinite;
		animation-direction: alternate;
	}

	@keyframes baranim {
		from {
			height: 2px;
		}
		to {
			height: 10px;
		}
	}

	.track:nth-child(odd) {
		background-color: var(--highlight);
	}

	.title {
		font-size: 1em;
		font-weight: 400;
		margin: 0;
	}

	.artist {
		font-size: 0.875em;
		opacity: 0.3;
		margin: 0;
	}

	.album,
	.duration {
		font-size: 0.875em;
		opacity: 0.6;
	}

	.count,
	.duration {
		font-family: var(--font-mono);
	}

	a {
		text-decoration: none;
		color: var(--fg);
	}

	a:hover {
		text-decoration: underline;
	}

	.album {
		padding: 0 10px;
	}

	.duration {
		text-align: right;
	}
</style>
