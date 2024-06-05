<script lang="ts">
	import type Manager from '$lib/manager.svelte';
	import type MediaState from '$lib/media.svelte';
	import type { Track } from '$lib/type';
	import { formatTime } from '$lib/utils';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import Play from 'lucide-svelte/icons/play';
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';

	let manager = getContext<Manager>('manager');
	let media = getContext<MediaState>('media');

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
</script>

<h1 class="__page_title ns">{$_('songs')}</h1>

<div class="quick-actions">
	<button
		onclick={async () => {
			await playAll();
		}}
	>
		<Play fill={'var(--fg)'} size={'1.5em'} />
		{$_('songs_page.listen')}
	</button>
</div>

<div class="songlist">
	{#each media.getSongs() as song, i}
		<div class="track ns" data-id={song.id} role="presentation" draggable>
			<div class="count" aria-colindex="1" role="gridcell">
				<div class="x">
					{i + 1}
				</div>
				<button
					class="play"
					onclick={async () => {
						await play(song);
					}}
				>
					<Play fill={'var(--fg)'} size={'16px'} />
				</button>
			</div>
			<div class="details-group" aria-colindex="2" role="gridcell">
				{#if song.cover}
					<div
						class="cover"
						style="--clr: {song.color
							? `rgb(${song.color.r}, ${song.color.g}, ${song.color.b})`
							: 'rgb(255, 255, 255)'}; background-image: url('{convertFileSrc(song.cover)}');"
					></div>
				{/if}
				<div class="details">
					<h4 class="title">{song.title}</h4>
					<p class="artist">{song.artists.join(', ')}</p>
				</div>
			</div>
			<div class="album" aria-colindex="3" role="gridcell">{song.album}</div>
			<div class="date" aria-colindex="4" role="gridcell">xxx:xxx:xxx</div>
			<div class="duration" aria-colindex="5" role="gridcell">{formatTime(song.duration)}</div>
		</div>
	{/each}
</div>

<style>
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
		padding-top: 3em;
	}

	.track {
		display: grid;
		grid-template-columns:
			[index] var(--tracklist-index-column-width, 16px) [first] minmax(120px, var(--col1, 6fr))
			[var1] minmax(120px, var(--col2, 4fr)) [var2] minmax(120px, var(--col3, 3fr)) [last] minmax(120px, var(--col4, 1fr));
		align-items: center;
		padding-inline: 16px;
		padding-block: 8px;
		margin-block: 0.5em;
		grid-gap: 16px;
		border-radius: 8px;
	}

	.track:hover {
		background-color: var(--highlight);
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

	.cover {
		height: 50px;
		width: 50px;
		background-color: var(--clr);
		border-radius: 4px;
		background-size: cover;
		background-position: center;
	}

	.details-group {
		display: flex;
	}

	.details {
		display: flex;
		flex-direction: column;
		justify-content: center;
		padding: 0 10px;
	}

	.title {
		font-size: 1em;
		font-weight: 600;
		margin: 0;
	}

	.artist {
		font-size: 0.875em;
		opacity: 0.6;
		margin: 0;
	}

	.album,
	.date,
	.duration {
		text-align: left;
		font-size: 0.875em;
		opacity: 0.6;
	}

	.album {
		padding: 0 10px;
	}

	.date,
	.duration {
		text-align: center;
	}
</style>
