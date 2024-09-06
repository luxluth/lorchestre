<script lang="ts">
	import { FilterOrder, FilterType, PlayingMode, type Track } from '$lib/type';
	import Play from 'lucide-svelte/icons/play';
	import { _ } from 'svelte-i18n';
	import Shuffle from 'lucide-svelte/icons/shuffle';

	import { Select } from 'bits-ui';
	import { flyAndScale } from '$lib/utils/transitions';

	import Check from 'lucide-svelte/icons/check';
	import Filter from 'lucide-svelte/icons/filter';
	import ArrowDown10 from 'lucide-svelte/icons/arrow-down-1-0';
	import Song from '$lib/components/Song.svelte';
	import VirtualScroll from 'svelte-virtual-scroll-list';
	import { formatTime, getCoverUri, setTitle } from '$lib/utils';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getFilter } from '$lib/filterq.svelte';
	import { getNav } from '$lib/nav.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	const manager = getManager();
	const media = getMedia();
	const ctx = getCtx();
	const filterquery = getFilter();
	const nav = getNav();
	const config = getAppConfig();

	async function playAll() {
		manager.pmode = PlayingMode.Normal;
		let songs = applyFilters(media.getSongs());
		let song = songs.shift() as Track;
		await manager.play(song);
		manager.clearQueue();
		manager.addManyToQueue(songs);
	}

	async function playAllShuffle() {
		let songs = applyFilters(media.getSongs());
		await manager.shufflePlay(songs);
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

	function sortTracksByTitle(tracks: Track[]): Track[] {
		return tracks.slice().sort((a, b) => {
			const titleA = a.title.toLowerCase();
			const titleB = b.title.toLowerCase();
			if (titleA < titleB) return -1;
			if (titleA > titleB) return 1;
			return 0;
		});
	}

	function sortTracksByDate(tracks: Track[]): Track[] {
		return tracks.slice().sort((a, b) => {
			return b.created_at - a.created_at;
		});
	}

	async function play(i: number) {
		let tracks = filteredTracks.slice(i, filteredTracks.length);
		let song = tracks.shift();
		if (song) {
			manager.play(song);
			manager.clearQueue();
			manager.addManyToQueue(tracks);
		}
	}

	let hoveredTrack: Track | null = $state(null);

	function notify(track: Track) {
		hoveredTrack = track;
	}

	function applyFilterQuery(tracks: Track[]): Track[] {
		let r: Track[] = [];
		switch (filterquery.type) {
			case FilterType.Alphabetic:
				r = sortTracksByTitle(tracks);
				break;
			case FilterType.TimeBased:
				r = sortTracksByDate(tracks);
				break;
			default:
				r = tracks;
				break;
		}

		if (filterquery.order == FilterOrder.Descendant) {
			r = r.reverse();
		}

		return r;
	}

	function applyFilters(tracks: Track[]): Track[] {
		let t = applyFilterQuery(tracks);
		if (searchInput.trim().length > 0) {
			return applySearchFilter(t, searchInput);
		} else {
			return t;
		}
	}

	const filterTypes = [
		{ value: FilterType.Alphabetic, label: 'songs_page.filter.type.alpha' },
		{ value: FilterType.TimeBased, label: 'songs_page.filter.type.date' }
	];

	const filterOrders = [
		{ value: FilterOrder.Ascendant, label: 'songs_page.filter.order.asc' },
		{ value: FilterOrder.Descendant, label: 'songs_page.filter.order.desc' }
	];

	let searchInput = $state('');
	let filteredTracks = $derived(uniquefied(applyFilters(media.getSongs())));

	function uniquefied(xs: Track[]): (Track & { uniqueKey: number })[] {
		let tracks: (Track & { uniqueKey: number })[] = [];
		let i = 0;

		for (const t of xs) {
			tracks.push({
				...t,
				uniqueKey: i++
			});
		}

		return tracks;
	}

	$effect(() => {
		console.log(filteredTracks);
	});
	$effect(() => {
		setTitle(`${$_('songs').toLowerCase()} — L'orchestre`);
		nav.pageName = $_('songs');
	});
</script>

<h1 class="ns">{$_('songs')}</h1>

<div class="container">
	<div
		class="songs"
		style="--tracklist-index-column-width: {(filteredTracks.length.toString().length * 16) / 2}px"
	>
		<VirtualScroll data={filteredTracks} key="uniqueKey" let:data>
			<Song
				song={data as Track}
				i={data.uniqueKey as number}
				{ctx}
				{manager}
				searchq={searchInput}
				onPlay={play}
				{notify}
			/>
			<div class="space-splitter"></div>
		</VirtualScroll>
	</div>
	<div class="more_info">
		{#if hoveredTrack}
			<div
				class="cover"
				style="--clr: {`rgb(${hoveredTrack.color?.r}, ${hoveredTrack.color?.g}, ${hoveredTrack.color?.b})`}; background-image: url('{getCoverUri(
					hoveredTrack.album_id,
					hoveredTrack.cover_ext,
					config
				)}');"
			></div>
			<div class="grid-container">
				<div class="grid-item label">ARTISTS</div>
				<div class="grid-item value">{hoveredTrack.artists.join(', ')}</div>
				<div class="grid-item label">TITLE</div>
				<div class="grid-item value">{hoveredTrack.title}</div>
				<div class="grid-item label">GENRES</div>
				<div class="grid-item value">{hoveredTrack.genres.join(', ')}</div>
				<div class="grid-item label">DURATION</div>
				<div class="grid-item value">{formatTime(hoveredTrack.duration)}</div>
				<div class="grid-item label">ENCODER</div>
				<div class="grid-item value">{hoveredTrack.encoder}</div>
				<div class="grid-item label">BITRATE</div>
				<div class="grid-item value">{hoveredTrack.bitrate} kb/s</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.space-splitter {
		height: 0.5em;
	}

	.more_info {
		font-family: var(--font-mono);
	}

	.cover {
		position: relative;
		width: 100%;
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		background-size: cover;
		margin-inline: auto;
		box-shadow: rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
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

	.songs {
		height: 80vh;
		overflow: hidden;
	}
	.container {
		padding-top: 0.5vh;
		display: grid;
		grid-template-columns: 3fr 1.5fr;
		gap: 5vw;
	}

	h1 {
		font-size: clamp(2.8125rem, 0.9375rem + 3.75vw, 3.75rem);
		font-weight: 500;
		letter-spacing: -6%;
	}
</style>
