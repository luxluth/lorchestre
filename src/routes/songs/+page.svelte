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
	import { setTitle } from '$lib/utils';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getFilter } from '$lib/filterq.svelte';
	import { getNav } from '$lib/nav.svelte';

	const manager = getManager();
	const media = getMedia();
	const ctx = getCtx();
	const filterquery = getFilter();
	const nav = getNav();

	//@ts-ignore
	let songsWidth: number = $state();
	//@ts-ignore
	let songsHeight: number = $state();

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
	let filteredTracks = $derived(applyFilters(media.getSongs()));
	$effect(() => {
		setTitle(`${$_('songs').toLowerCase()} — L'orchestre`);
		nav.pageName = $_('songs');
	});
</script>

<h1 class="ns">{$_('songs')}</h1>

<div class="container">
	<div class="songs" bind:clientWidth={songsWidth} bind:clientHeight={songsHeight}></div>
	<div class="more_info"></div>
</div>

<style>
	.more_info {
		border: 1px aquamarine dashed;
	}
	.songs {
		border: 1px orange dashed;
		height: 80vh;
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
