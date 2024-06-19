<script lang="ts">
	import {
		ContextMenuItemType,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { getContext } from 'svelte';
	import type Manager from '$lib/manager.svelte';
	import type Ctx from '$lib/ctx.svelte';

	import ListEnd from 'lucide-svelte/icons/list-end';
	import Play from 'lucide-svelte/icons/play';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';

	import type MediaState from '$lib/media.svelte';
	import { _ } from 'svelte-i18n';
	import { getCoverUri } from '$lib/utils';
	import type AlbumPageData from '$lib/album.svelte';

	let media = getContext<MediaState>('media');

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	let manager = getContext<Manager>('manager');
	let ctx = getContext<Ctx>('ctx');

	function showContext(e: ContextMenuEvent, tracks: Track[]) {
		let sortedTracks = sortTracks(tracks);
		const items: ContextMenuItem[] = [
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					let firstTrack = sortedTracks.shift() as Track;
					manager.queue = [];
					await manager.addManyToQueue(sortedTracks);
					await manager.play(firstTrack);
				},
				label: $_('ctx.play'),
				icon: Play
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					await manager.addManyToQueue(sortedTracks);
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
</script>

<h1 class="__page_title ns">{$_('albums')}</h1>

{#if media.loaded}
	<div class="__medias">
		{#each media.albums as album}
			<a
				class="__audio"
				data-id={album.id}
				href="/album/{album.id}"
				oncontextmenu={(e) => {
					e.preventDefault();
					showContext(e, album.tracks);
				}}
			>
				<div
					class="cover"
					style="--clr: {album.tracks[0].color
						? `rgb(${album.tracks[0].color.r}, ${album.tracks[0].color.g}, ${album.tracks[0].color.b})`
						: 'rgb(255, 255, 255)'}; background-image: url('{getCoverUri(
						album.id,
						album.tracks[0].cover_ext
					)}');"
				></div>
				<p class="title ns">{trim(album.name)}</p>
				<p class="artist ns">{album.artist}</p>
			</a>
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

	.__medias {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(15em, 1fr));
		column-gap: 1em;
		row-gap: 3em;
	}

	.__audio {
		width: 15em;
		text-decoration: none;
		color: var(--fg);
		cursor: pointer;
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
		background-size: cover;
	}

	.__audio .artist {
		opacity: 0.5;
		padding-top: 0.2em;
	}

	.__audio .title {
		font-weight: bold;
	}
</style>
