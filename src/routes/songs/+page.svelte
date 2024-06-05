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
	{#each media.getSongs() as song}
		<div class="track ns" data-id={song.id} role="presentation" draggable>
			<div class="details-group" aria-colindex="1" role="gridcell">
				{#if song.cover}
					<div
						class="cover"
						style="--clr: {song.color
							? `rgb(${song.color.r}, ${song.color.g}, ${song.color.b})`
							: 'rgb(255, 255, 255)'}; background-image: url('{convertFileSrc(song.cover)}');"
					>
						<button
							class="play"
							onclick={async () => {
								await play(song);
							}}
						>
							<Play fill={'#ffffff'} size={'16px'} />
						</button>
					</div>
				{/if}
				<div class="details">
					<h4 class="title">{song.title}</h4>
				</div>
			</div>
			<div class="artist" aria-colindex="2" role="gridcell">{song.artists.join(', ')}</div>
			<div class="album" aria-colindex="3" role="gridcell">
				<a href="/album/{song.album_id}">{song.album}</a>
			</div>
			<div class="duration" aria-colindex="4" role="gridcell">{formatTime(song.duration)}</div>
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
		color: var(--fg);
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
			[first] minmax(120px, var(--col1, 6fr))
			[var1] minmax(120px, var(--col2, 4fr)) [var2] minmax(120px, var(--col3, 3fr)) [last] minmax(120px, var(--col4, 1fr));
		align-items: center;
		grid-gap: 16px;
		padding-block: 4px;
		padding-inline: 5px;
	}

	.track:nth-child(odd) {
		background-color: var(--highlight);
	}

	.cover {
		height: 40px;
		width: 40px;
		background-color: var(--clr);
		border-radius: 3px;
		background-size: cover;
		background-position: center;
		color: #ffffff;
		position: relative;
		overflow: hidden;
	}

	.cover button {
		background: rgba(0, 0, 0, 0.2);
		border: none;
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: none;
	}

	.track:hover .cover button {
		display: block;
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
		text-align: left;
		font-size: 0.875em;
		opacity: 0.6;
	}

	a {
		text-decoration: none;
		color: var(--fg);
		text-overflow: ellipsis;
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
