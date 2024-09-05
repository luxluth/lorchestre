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
	import { getNav } from '$lib/nav.svelte';
	import { onMount } from 'svelte';

	let manager = getManager();
	let media = getMedia();

	function sortTracks(t: Track[]) {
		return t.sort((a, b) => a.track - b.track);
	}

	function trim(text: string, len = 40) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}

	let ctx = getCtx();
	let conf = getAppConfig();

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

	function getRandomAlbums(quantity = 3) {
		let albums = [...media.albums];

		for (let i = albums.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[albums[i], albums[j]] = [albums[j], albums[i]];
		}

		return albums.slice(0, quantity);
	}

	let albums = getRandomAlbums();
	const n = getNav();

	onMount(() => {
		n.pageName = 'Recommendation';
	});
</script>

<h1 class="ns">Listen To Next</h1>
{#if media.loaded}
	<div class="content">
		<div class="albums">
			{#each albums as album}
				<a
					class="album"
					oncontextmenu={(e) => {
						e.preventDefault();
						showContext(e, getTracks(album));
					}}
					href="/album/{album.id}"
				>
					<div
						class="cover"
						style="background-image: url({getCoverUri(
							album.id,
							getTrack(album.tracks[0]).cover_ext,
							conf
						)}); --clr: {getTrack(album.tracks[0])
							? `rgb(${getTrack(album.tracks[0]).color?.r}, ${getTrack(album.tracks[0]).color?.g}, ${getTrack(album.tracks[0]).color?.b})`
							: 'rgb(255, 255, 255)'};
            "
					></div>
					<div class="info">
						<h4 class="ww">{album.name}</h4>
						<p class="ww">{album.artist}</p>
					</div>
				</a>
			{/each}
		</div>
	</div>
{/if}

<style>
	h1 {
		font-size: clamp(2.8125rem, 0.9375rem + 3.75vw, 3.75rem);
		font-weight: 500;
		letter-spacing: -6%;
	}

	a {
		text-decoration: none;
		color: rgba(255, 255, 255, 0.5);
	}

	.info {
		padding-top: 1em;
	}

	.ww {
		text-align: center;
		margin-top: 4px;
	}

	.content {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 90%;
	}

	.albums {
		margin-inline: auto;
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 9vw;
		width: max-content;
	}

	.album {
		width: 20vw;
	}

	.album .cover {
		background-color: var(--clr);
		width: 20vw;
		aspect-ratio: 1/1;
		border-radius: 10px;
		/* box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px; */
		background-size: cover;
		position: relative;
		overflow: hidden;
	}
</style>
