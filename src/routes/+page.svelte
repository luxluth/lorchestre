<script lang="ts">
	import { convertFileSrc, invoke } from '@tauri-apps/api/core';
	import {
		ContextMenuItemType,
		type ContextMenuEvent,
		type ContextMenuItem,
		type Track
	} from '$lib/type';
	import { getContext } from 'svelte';
	import type Manager from '$lib/manager.svelte';
	import type Ctx from '$lib/ctx.svelte';
	import { ListEnd, Play, LoaderCircle } from 'lucide-svelte';
	import type MediaState from '$lib/media.svelte';

	let media = getContext<MediaState>('media');

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
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
				label: 'Lancer la lecture',
				icon: Play
			},
			{
				type: ContextMenuItemType.Action,
				action: async (_data: any) => {
					sortedTracks.forEach((track) => {
						manager.addToQueue(track);
					});
				},
				label: "Ajouter la file d'attente",
				icon: ListEnd
			}
		];
		ctx.x = e.x;
		ctx.y = e.y;
		ctx.items = items;
		ctx.visible = true;
	}
</script>

<h1 class="__page_title ns">Albums</h1>

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
				<p class="title ns">{name}</p>
				<p class="artist ns">{artist}</p>
			</a>
		{/each}
	</div>
{:else}
	<div class="msg">
		<div class="icon"><LoaderCircle /></div>
		Indexation des fichiers locaux...
	</div>
{/if}

<style>
	.msg {
		height: 100%;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 2rem;
		font-weight: bold;
		gap: 0.5em;
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

	.__audio:active {
		transform: scale(0.95);
	}

	/* .__audio .cover:hover button { */
	/* 	opacity: 1; */
	/* } */
	/**/
	/* .__audio .cover button { */
	/* 	position: absolute; */
	/* 	bottom: 50%; */
	/* 	left: 50%; */
	/* 	transform: translate(-50%, 50%); */
	/* 	background-color: var(--fg); */
	/* 	border: none; */
	/* 	display: flex; */
	/* 	align-items: center; */
	/* 	justify-content: center; */
	/* 	padding: 2em; */
	/* 	border-radius: 50%; */
	/* 	opacity: 0; */
	/* 	transition: all 0.15s ease-in-out; */
	/* 	cursor: pointer; */
	/* } */
	/**/
	/* .__audio .cover button:active { */
	/* 	transform: translate(-50%, 50%) scale(0.9); */
	/* 	opacity: 0.5; */
	/* } */

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
