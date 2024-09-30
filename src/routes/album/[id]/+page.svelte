<script lang="ts">
	import Play from 'lucide-svelte/icons/play';
	import ListEnd from 'lucide-svelte/icons/list-end';
	import { formatTime, setTitle } from '$lib/utils';

	import { _ } from 'svelte-i18n';

	import { page } from '$app/stores';
	import {
		type Track,
		type ContextMenuItem,
		ContextMenuItemType,
		type ContextMenuEvent,
		type Album,
		QueueAddMode,
		PlayingMode
	} from '$lib/type';
	import { getCoverUri } from '$lib/utils';
	import ListStart from 'lucide-svelte/icons/list-start';
	import Disc from 'lucide-svelte/icons/disc';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getAppConfig } from '$lib/config.svelte';
	import { onMount } from 'svelte';

	let manager = getManager();
	let media = getMedia();

	function getTracks(album: Album) {
		let tracks = [];
		for (const path of album.tracks) {
			let track = media.getTrack(path);
			if (track) tracks.push(track);
		}

		return tracks;
	}

	let album = $derived($page.data.album);
	let tracks = $derived(album ? sortTracks(getTracks(album)) : []);
	let discs = $derived(putIntoDisks(tracks));

	let ctx = getCtx();
	let config = getAppConfig();

	function putIntoDisks(tracks: Track[]) {
		let diskMap = new Map<number, Track[]>();
		for (const track of tracks) {
			if (diskMap.has(track.disc)) {
				let mapts = diskMap.get(track.disc) as Track[];
				mapts.push(track);
				mapts = sortTracks(mapts);
				diskMap.set(track.disc, mapts);
			} else {
				diskMap.set(track.disc, [track]);
			}
		}

		return diskMap;
	}

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	async function play(t: Track) {
		await manager.play(t);
	}

	function showContext(e: ContextMenuEvent, track: Track) {
		const items: ContextMenuItem[] = [
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
			}
		];
		ctx.x = e.x;
		ctx.y = e.y;
		ctx.items = items;
		ctx.visible = true;
	}

	function getTrack(path: string) {
		return media.getTrack(path) as Track;
	}

	async function playAlbum() {
		manager.clearQueue();
		manager.pmode = PlayingMode.Normal;
		let track = tracks[0];
		await manager.play(track);
		let toAddToTheQue = [...tracks];
		console.log(toAddToTheQue);
		toAddToTheQue.shift();
		manager.addManyToQueue(toAddToTheQue);
	}

	async function playfrom(i: number) {
		manager.pmode = PlayingMode.Normal;
		let songs = tracks.slice(i, tracks.length);
		let song = songs.shift();
		if (song) {
			manager.play(song);
			manager.clearQueue();
			manager.addManyToQueue(songs);
		}
	}

	$effect(() => {
		console.log(discs);
	});

	function getDuration() {
		return tracks.reduce((pt, track) => pt + track.duration, 0);
	}

	function avgbitrate() {
		let d = tracks.length;
		let sm = 0;
		for (const track of tracks) {
			sm += track.bitrate;
		}

		if (d > 0) {
			return sm / d;
		} else {
			return 0;
		}
	}

	onMount(() => {
		setTitle(`${album ? album.name : 'Album not found'} — L'orchestre`);
	});
</script>

<svelte:head>
	<title>mu - {album ? album.name : ''}</title>
</svelte:head>

{#if album}
	<section class="page-grid ns">
		<div class="info">
			<div
				class="cover"
				style="--clr: {getTrack(album.tracks[0])
					? `rgb(${getTrack(album.tracks[0]).color?.r}, ${getTrack(album.tracks[0]).color?.g}, ${getTrack(album.tracks[0]).color?.b})`
					: 'rgb(255, 255, 255)'}; background-image: url('{getCoverUri(
					album.id,
					getTrack(album.tracks[0]).cover_ext,
					config
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
			<div class="grid-container details">
				<div class="grid-item label">NAME</div>
				<div class="grid-item value">{album.name}</div>
				<div class="grid-item label">ARTIST</div>
				<div class="grid-item value">{album.artist}</div>
				<div class="grid-item label">GENRES</div>
				<div class="grid-item value">{album.genres.join(', ')}</div>
				<div class="grid-item label">TRACKS</div>
				<div class="grid-item value">
					{album.tracks_count >= 10 ? album.tracks_count : `0${album.tracks_count}`}
				</div>
				<div class="grid-item label">DISKS</div>
				<div class="grid-item value disc_total">
					{album.disc_total >= 10 ? album.disc_total : `0${album.disc_total}`}
				</div>
				<div class="grid-item label">DURATION</div>
				<div class="grid-item value">{formatTime(getDuration())}</div>
				<div class="grid-item label">ENCODER</div>
				<div class="grid-item value">{album.encoder}</div>
				<div class="grid-item label">AVG. BITRATE</div>
				<div class="grid-item value">{avgbitrate().toFixed(0)} kb/s</div>
			</div>
		</div>
		<div class="songs">
			{#if album.disc_total > 1}
				{#each discs as [no, tracks]}
					<h3 class="disc"><Disc /> disc {no}</h3>
					<section class="tracks ns">
						{#each tracks as track, i}
							<div
								class="track"
								oncontextmenu={(e) => {
									e.preventDefault();
									showContext(e, track);
								}}
								role="button"
								tabindex="0"
								ondblclick={async () => {
									await play(track);
								}}
								onkeydown={() => {}}
							>
								<div class="trackn">
									<div class="no">{track.track > 0 ? track.track : ''}</div>
									<button
										onclick={async () => {
											await playfrom(i);
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
				{/each}
			{:else}
				<section class="tracks ns">
					{#each tracks as track, i}
						<div
							class="track"
							oncontextmenu={(e) => {
								e.preventDefault();
								showContext(e, track);
							}}
							role="button"
							tabindex="0"
							ondblclick={async () => {
								await play(track);
							}}
							onkeydown={() => {}}
						>
							<div class="trackn">
								<div class="no">{track.track > 0 ? track.track : ''}</div>
								<button
									onclick={async () => {
										await playfrom(i);
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
			{/if}
		</div>
	</section>
{:else}
	Album non-existant
{/if}

<style>
	.disc {
		padding-block: 1em;
		opacity: 0.6;
		width: 100%;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		gap: 0.5em;
	}

	.grid-container {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 10px;
		padding: 20px;
		max-width: 100%;
	}

	.grid-item {
		padding: 0px 0;
	}

	.label {
		text-transform: uppercase;
		opacity: 0.7;
	}

	.value {
		font-weight: bold;
	}

	.details {
		font-family: var(--font-mono);
	}

	.page-grid {
		padding-bottom: 4em;
	}

	/* h1 { */
	/* 	font-size: clamp(2.8125rem, 0.9375rem + 3.75vw, 3.75rem); */
	/* 	font-weight: 500; */
	/* 	letter-spacing: -6%; */
	/* } */

	.page-grid {
		display: grid;
		grid-template-columns: 1.5fr 3fr;
		gap: 1em;
		width: 100%;
		margin-bottom: 10em;
	}

	.songs {
		width: 100%;
		margin-bottom: 10em;
	}

	.track {
		display: flex;
		align-items: center;
		height: 2.5em;
		gap: 1em;
		padding-inline: 1em;
		border-radius: 4px;
	}

	.track:nth-child(odd) {
		background-color: #292929;
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
		width: 100%;
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		background-size: cover;

		box-shadow: rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
	}
</style>
