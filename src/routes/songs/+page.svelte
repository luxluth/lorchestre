<script lang="ts">
	import type Ctx from '$lib/ctx.svelte';
	import type Manager from '$lib/manager.svelte';
	import type MediaState from '$lib/media.svelte';
	import {
		ContextMenuItemType,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { formatTime } from '$lib/utils';
	import ListEnd from 'lucide-svelte/icons/list-end';
	import Play from 'lucide-svelte/icons/play';
	import Disc from 'lucide-svelte/icons/disc';
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import Shuffle from 'lucide-svelte/icons/shuffle';

	let manager = getContext<Manager>('manager');
	let media = getContext<MediaState>('media');
	let ctx = getContext<Ctx>('ctx');

	async function playAll() {
		let songs = media.getSongs();
		let song = songs.shift() as Track;
		await manager.play(song);
		manager.clearQueue();
		manager.addManyToQueue(songs);
	}

	async function play(track: Track) {
		await manager.play(track);
	}

	function trim(text: string, len = 20) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
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

	function applySearchFilter(tracks: Track[], q: string): Track[] {
		const lowerCaseQuery = q.toLowerCase();
		return tracks.filter((track) => {
			const titleMatch = track.title.toLowerCase().includes(lowerCaseQuery);
			const artistMatch = track.artists.some((artist) =>
				artist.toLowerCase().includes(lowerCaseQuery)
			);
			const albumMatch = track.album.toLowerCase().includes(lowerCaseQuery);

			return titleMatch || artistMatch || albumMatch;
		});
	}

	function sortTracksByTitleAZ(tracks: Track[]): Track[] {
		return tracks.slice().sort((a, b) => {
			const titleA = a.title.toLowerCase();
			const titleB = b.title.toLowerCase();
			if (titleA < titleB) return -1;
			if (titleA > titleB) return 1;
			return 0;
		});
	}

	function applyFilters(tracks: Track[]): Track[] {
		if (searchInput.trim().length > 0) {
			return applySearchFilter(tracks, searchInput);
		} else {
			return tracks;
		}
	}

	let searchInput = $state('');
</script>

<h1 class="__page_title ns">{$_('songs')}</h1>

<div class="quick-actions">
	<button
		onclick={async () => {
			await playAll();
		}}
	>
		<div class="icon">
			<Play fill={'var(--fg)'} size={'1em'} />
		</div>
		{$_('songs_page.listen')}
	</button>
	<button>
		<div class="icon">
			<Shuffle size={'1em'} />
		</div>
		{$_('songs_page.shuffle')}
	</button>
</div>

<div class="filters">
	<input bind:value={searchInput} type="search" name="search" placeholder={$_('search')} />
</div>

<div class="songlist">
	{#each applyFilters(media.getSongs()) as song, i}
		<div
			class="track ns"
			data-id={song.id}
			role="presentation"
			draggable
			oncontextmenu={(e) => {
				e.preventDefault();
				showContext(e, song);
			}}
		>
			<div
				class="count"
				class:iscurrent={manager.currentTrack?.id === song.id}
				aria-colindex="1"
				role="gridcell"
			>
				{#if manager.currentTrack?.id === song.id}
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
				<h4 class="title">{song.title}</h4>
			</div>
			<div class="artist" aria-colindex="3" role="gridcell">{trim(song.artists.join(', '))}</div>
			<div class="album" aria-colindex="4" role="gridcell">
				<a href="/album/{song.album_id}">{trim(song.album)}</a>
			</div>
			<div class="duration" aria-colindex="5" role="gridcell">{formatTime(song.duration)}</div>
		</div>
	{/each}
</div>

<style>
	.filters {
		padding-top: 2em;
		display: flex;
		justify-content: flex-end;
		align-items: center;
	}

	input[type='search'] {
		-webkit-appearance: none;
		appearance: none;
		padding-inline: 0.5em;
		padding-block: 0.7em;
		border-radius: 4px;
		border: 0px;
		background: var(--highlight);
		color: var(--fg);
	}

	.quick-actions button {
		display: flex;
		justify-content: center;
		align-content: center;
		gap: 0.5em;
		font-size: 1.1em;
		font-weight: bold;
		border: none;
		background-color: var(--highlight);
		cursor: pointer;
		padding: 0.6em;
		border-radius: 8px;
		color: var(--fg);
	}

	.quick-actions button .icon {
		height: 100%;
		display: flex;
		align-items: center;
	}

	.quick-actions {
		display: flex;
		gap: 1em;
	}

	.quick-actions button:active {
		transform: scale(0.98);
	}

	.__page_title {
		font-weight: 800;
		font-size: 1.5em;
		padding-bottom: 2em;
		text-align: center;
	}

	.songlist {
		width: 100%;
		padding-top: 2em;
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
		background-color: var(--fg);
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
