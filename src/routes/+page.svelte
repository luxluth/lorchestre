<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';
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
					sortedTracks.forEach((track) => {
						manager.addToQueue(track);
					});
					await manager.play(firstTrack);
				},
				label: $_('ctx.play'),
				icon: Play
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					sortedTracks.forEach((track) => {
						manager.addToQueue(track);
					});
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

{#if !media.loading}
	<div class="__medias">
		{#each media.albums as { tracks, artist, name, id }}
			<a
				class="__audio"
				data-id={id}
				href="album/{id}"
				oncontextmenu={(e) => {
					e.preventDefault();
					showContext(e, tracks);
				}}
			>
				{#if tracks[0].cover}
					<div
						class="cover"
						style="--clr: {tracks[0].color
							? `rgb(${tracks[0].color.r}, ${tracks[0].color.g}, ${tracks[0].color.b})`
							: 'rgb(255, 255, 255)'}; background-image: url('{convertFileSrc(tracks[0].cover)}');"
					></div>
				{/if}
				<p class="title ns">{trim(name)}</p>
				<p class="artist ns">{artist}</p>
			</a>
		{/each}
	</div>
{:else}
	<div class="msg">
		<div class="icon">
			<LoaderCircle />
			{$_('indexing_msg')}
		</div>
		<div class="data">{media.loading_data}</div>
		<div class="data">{media.treatedFilesCount}/{media.files_count}</div>
	</div>
{/if}

<style>
	.msg {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5em;
	}

	.msg .data {
		font-size: 0.5em;
		font-family: var(--font-mono);
	}

	.msg .icon {
		font-size: 2rem;
		font-weight: bold;
	}

	.msg .icon :global(svg) {
		animation: rotate 0.5s linear infinite;
	}

	@keyframes rotate {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.__page_title {
		font-weight: 800;
		font-size: 1.5em;
		padding-bottom: 2em;
		text-align: center;
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
