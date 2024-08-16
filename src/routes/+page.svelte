<script lang="ts">
	import {
		ContextMenuItemType,
		QueueAddMode,
		type Album,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';

	import ListEnd from 'lucide-svelte/icons/list-end';
	import Play from 'lucide-svelte/icons/play';

	import { _ } from 'svelte-i18n';
	import { getCoverUri, setTitle } from '$lib/utils';
	import ListStart from 'lucide-svelte/icons/list-start';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	let manager = getManager();
	let media = getMedia();

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	let ctx = getCtx();
	let config = getAppConfig();

	function showContext(e: ContextMenuEvent, tracks: Track[]) {
		let sortedTracks = sortTracks(tracks);
		const items: ContextMenuItem[] = [
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					let firstTrack = sortedTracks.shift() as Track;
					manager.clearQueue();
					await manager.play(firstTrack);
					manager.addManyToQueue(sortedTracks);
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

	$effect(() => {
		setTitle(`${$_('albums').toLowerCase()} — L'orchestre`);
	});

	function getTrack(path: string) {
		return media.getTrack(path) as Track;
	}

	function getTracks(album: Album) {
		let tracks = [];
		for (const path of album.tracks) {
			let track = media.getTrack(path);
			if (track) tracks.push(track);
		}

		return tracks;
	}

	function sort(albums: Album[]): Album[] {
		return albums.sort((a, b) => a.name.localeCompare(b.name));
	}

	type CategorizedAlbum = {
		char: string;
		albums: Album[];
	};

	function processAlbums(a: Album[]): CategorizedAlbum[] {
		let albums = [...a];
		let categorizedAlbums: CategorizedAlbum[] = [];
		let aCode = 'A'.charCodeAt(0);
		for (let i = 0; i < 26; i++) {
			let currentChar = String.fromCharCode(aCode + i);
			let toRemove: number[] = [];
			let collectedAlbums = albums.filter((album, idx) => {
				if (album.name[0].toUpperCase() == currentChar) {
					toRemove.push(idx);
					return true;
				} else {
					return false;
				}
			});

			toRemove.forEach((i) => {
				delete albums[i];
			});

			albums = albums.filter((a) => typeof a !== 'undefined');

			if (collectedAlbums.length > 0) {
				categorizedAlbums.push({
					char: currentChar,
					albums: collectedAlbums
				});
			}
		}

		// TODO: improve categorization
		if (albums.length > 0) {
			categorizedAlbums.push({
				char: '#',
				albums
			});
		}

		return categorizedAlbums;
	}
</script>

<h1 class="__page_title ns">{$_('albums')}</h1>

{#if media.loaded}
	<div class="__medias">
		{#each processAlbums(sort(media.albums)) as data}
			<section>
				<h3 class="glass ns">{data.char}</h3>
				<div class="__albums_container">
					{#each data.albums as album}
						<a
							class="__audio"
							data-id={album.id}
							href="/album/{album.id}"
							oncontextmenu={(e) => {
								e.preventDefault();
								showContext(e, getTracks(album));
							}}
						>
							<div
								class="cover"
								style="--clr: {getTrack(album.tracks[0])
									? `rgb(${getTrack(album.tracks[0]).color?.r}, ${getTrack(album.tracks[0]).color?.g}, ${getTrack(album.tracks[0]).color?.b})`
									: 'rgb(255, 255, 255)'};"
							>
								<img
									class="ns"
									src={getCoverUri(album.id, getTrack(album.tracks[0]).cover_ext, config, -1)}
									alt=""
									loading="lazy"
								/>
							</div>
							<p class="title ns">{trim(album.name)}</p>
							<p class="artist ns">{album.artist}</p>
						</a>
					{/each}
				</div>
			</section>
		{/each}
	</div>
{:else}
	<div class="loading">
		<div class="dot"></div>
	</div>
{/if}

<style>
	.loading {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100%;
		width: 100%;
	}

	.dot {
		height: 2em;
		width: 2em;
		border-radius: 50%;
		background-color: var(--fg);
	}

	.__page_title {
		font-weight: 800;
		padding-bottom: 2em;
	}

	.__albums_container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(15em, 1fr));
		column-gap: 1em;
		row-gap: 3em;
	}

	section h3 {
		text-align: right;
		position: -webkit-sticky;
		position: sticky;
		height: 2em;
		width: 80%;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		padding-inline: 1em;
		margin-bottom: 2em;
		margin-top: 0.5em;
		top: 0;
		margin-inline: auto;
		border-radius: 12px;
		border: 1px solid var(--highlight);
		box-shadow: rgba(0, 0, 0, 0.1) 0px 4px 12px;
		z-index: 30;
	}

	.__audio {
		width: 15em;
		text-decoration: none;
		color: var(--fg);
		cursor: pointer;
	}

	.__audio .title {
		word-wrap: break-word;
	}

	.__audio .cover {
		width: 100%;
		position: relative;
	}

	.__audio .cover {
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		overflow: hidden;
	}

	.__audio .cover img {
		width: 100%;
		height: 100%;
	}

	.__audio .artist {
		opacity: 0.5;
		padding-top: 0.2em;
	}

	.__audio .title {
		font-weight: bold;
	}
</style>
