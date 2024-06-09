<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';

	import ListStart from 'lucide-svelte/icons/list-start';
	import Play from 'lucide-svelte/icons/play';
	import ListEnd from 'lucide-svelte/icons/list-end';

	import { _ } from 'svelte-i18n';

	import type { PageData } from './$types';
	import {
		type Track,
		type ContextMenuItem,
		ContextMenuItemType,
		type ContextMenuEvent,
		type Album
	} from '$lib/type';
	import type Ctx from '$lib/ctx.svelte';
	import { getContext } from 'svelte';
	import type Manager from '$lib/manager.svelte';
	import { getCoverUri } from '$lib/utils';

	const { data }: { data: PageData } = $props();

	let manager = getContext<Manager>('manager');
	const album = data.album;
	const tracks = album ? sortTracks(album.tracks) : [];
	let ctx = getContext<Ctx>('ctx');

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function formatTime(time: number) {
		if (isNaN(time)) {
			return '--:--';
		}
		if (time >= 60 * 60) {
			return new Date(time * 1000).toISOString().substring(11, 16);
		} else {
			return new Date(time * 1000).toISOString().substring(14, 19);
		}
	}

	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	function play(t: Track) {
		manager.play(t);
	}

	function showContext(e: ContextMenuEvent, track: Track) {
		const items: ContextMenuItem[] = [
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					manager.addToQueue(track);
				},
				label: $_('album.page.ctx.add_queue'),
				icon: ListEnd
			}
		];
		ctx.x = e.x;
		ctx.y = e.y;
		ctx.items = items;
		ctx.visible = true;
	}

	async function playAlbum() {
		manager.queue = [];
		let track = (album as Album).tracks[0];
		await manager.play(track);

		tracks.forEach((track, i) => {
			if (i > 0) manager.addToQueue(track);
		});
	}
</script>

{#if album}
	<section class="head">
		<div
			class="cover"
			style="--clr: {album.tracks[0].color
				? `rgb(${album.tracks[0].color.r}, ${album.tracks[0].color.g}, ${album.tracks[0].color.b})`
				: 'rgb(255, 255, 255)'}; background-image: url('{getCoverUri(
				album.id,
				album.tracks[0].cover_ext
			)}');"
		>
			<button
				class="play"
				onclick={async () => {
					await playAlbum();
				}}
			>
				<Play size={'2em'} fill={'var(--fg)'} />
			</button>
		</div>
		<div class="data">
			<h1>{trim(album.name, 60)}</h1>
			<h3>{album.artist}</h3>
			<p>{album.year}</p>
		</div>
	</section>

	<h2 class="section-title">{$_('album.page.songs')}</h2>
	<section class="tracks ns">
		{#each tracks as track}
			<div
				class="track"
				oncontextmenu={(e) => {
					e.preventDefault();
					showContext(e, track);
				}}
				role="button"
				tabindex="0"
				onkeydown={() => {}}
			>
				<div class="trackn">
					<div class="no">{track.track > 0 ? track.track : ''}</div>
					<button
						onclick={() => {
							play(track);
						}}
					>
						<Play size={'1.5em'} fill={'var(--fg)'} />
					</button>
				</div>
				<div class="title">{track.title}</div>
				<div class="artists">{track.artists.join(', ')}</div>
				<div class="duration">{formatTime(track.duration)}</div>
			</div>
		{/each}
	</section>
{:else}
	Album non-existant
{/if}

<style>
	.section-title {
		padding-top: 2em;
		padding-bottom: 1em;
	}

	.track {
		display: flex;
		align-items: center;
		height: 2.5em;
		gap: 1em;
		padding-inline: 1em;
	}

	.track:nth-child(odd) {
		background-color: var(--highlight);
	}

	.track .title {
		font-weight: bold;
	}

	.track .artists {
		flex-grow: 1;
		text-align: right;
		opacity: 0.3;
	}

	.track .duration {
		opacity: 0.5;
	}

	.track .trackn {
		width: 2em;
		display: flex;
		align-items: center;
	}

	.track .trackn button {
		display: none;
		color: var(--fg);
		background: none;
		border: none;
	}

	.track:hover .trackn {
		opacity: 1;
	}

	.track:hover .trackn button {
		display: block;
	}

	.track:hover .trackn .no {
		display: none;
	}

	.trackn .no {
		opacity: 0.5;
	}

	.trackn,
	.duration {
		font-family: var(--font-mono);
		font-size: 0.875em;
		opacity: 0.6;
	}

	.head {
		display: flex;
		gap: 1em;
		width: 100%;
	}

	.head .data {
		align-self: flex-end;
	}

	.head .data h1 {
		font-size: 4rem;
		font-family: var(--font-fantasy);
		line-height: 1;
		height: 100%;
		padding-bottom: 0.1em;
	}

	.head .data h3 {
		opacity: 0.7;
	}

	.cover button.play {
		position: absolute;
		bottom: 1em;
		left: 1em;
		padding: 1em;
		background: var(--bg);
		color: var(--fg);
		border: 0px;
		display: flex;
		justify-content: center;
		align-items: center;
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
		border-radius: 50%;
		cursor: pointer;
	}

	button.play:active {
		transform: scale(0.98);
	}

	.cover:hover button.play {
		opacity: 1;
	}

	.cover {
		position: relative;
		height: 25em;
		width: 25em;
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		background-size: cover;

		box-shadow: rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
	}
</style>
