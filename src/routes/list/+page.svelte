<script lang="ts">
	import { FilterOrder, FilterType, type Track } from '$lib/type';
	import { _ } from 'svelte-i18n';
	import Play from 'lucide-svelte/icons/play';
	import Shuffle from 'lucide-svelte/icons/shuffle';
	import { Select } from 'bits-ui';
	import Filter from 'lucide-svelte/icons/filter';
	import Check from 'lucide-svelte/icons/check';
	import { flyAndScale } from '$lib/utils/transitions';
	import ArrowDown10 from 'lucide-svelte/icons/arrow-down-1-0';
	import Song from '$lib/components/Song.svelte';
	import { setTitle } from '$lib/utils';
	import { getManager } from '$lib/manager.svelte';
	import { getList } from '$lib/playlist.svelte';
	import { getCtx } from '$lib/ctx.svelte';

	let list = getList();
	let manager = getManager();
	let ctx = getCtx();
	let filterquery = list.filters;

	async function playAll() {
		let songs = applyFilters(list.tracks);
		let song = songs.shift() as Track;
		manager.play(song);
		manager.clearQueue();
		manager.addManyToQueue(songs);
	}

	async function playAllShuffle() {
		let songs = applyFilters(list.tracks);
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
			const dateA = a.created_at.secs_since_epoch * 1e9 + a.created_at.nanos_since_epoch;
			const dateB = b.created_at.secs_since_epoch * 1e9 + b.created_at.nanos_since_epoch;
			return dateB - dateA;
		});
	}

	function applyFilterQuery(tracks: Track[]): Track[] {
		let r = [];
		switch (filterquery.type) {
			case FilterType.Alphabetic:
				r = sortTracksByTitle(tracks);
				break;
			case FilterType.TimeBased:
				r = sortTracksByDate(tracks);
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

	$effect(() => {
		setTitle(
			`${$_('playlist').toLowerCase()} — ${list.activeList ? list.activeList.name : ''} - L'orchestre`
		);
	});
</script>

<div class="page ns">
	{#if list.activeList}
		<h1>{list.activeList.name}</h1>
		<p class="track_counts">
			{list.activeList.tracks.length}
			{list.activeList.tracks.length > 1 ? $_('stats_page.songs') : $_('stats_page.song')}
		</p>

		<div class="quick-actions">
			<button
				onclick={async () => {
					await playAll();
				}}
			>
				<div class="icon">
					<Play fill={'var(--fg)'} size={'1em'} />
				</div>
				{$_('songs_page.listen')}
			</button>
			<button
				onclick={async () => {
					await playAllShuffle();
				}}
			>
				<div class="icon">
					<Shuffle size={'1em'} />
				</div>
				{$_('songs_page.shuffle')}
			</button>
		</div>

		<div class="filters">
			<div class="filter">
				<Select.Root
					items={filterTypes}
					selected={filterTypes.find((l) => l.value === filterquery.type)}
					onSelectedChange={(e) => {
						if (e) {
							filterquery.type = e.value;
						}
					}}
				>
					<Select.Trigger class="select-trigger" aria-label="Choisissez un type de filtre">
						<Filter class="icon" />
						<div class="text">{$_('songs_page.filter.filter_to_apply')}</div>
					</Select.Trigger>
					<Select.Content class="select-content" sideOffset={8} transition={flyAndScale}>
						{#each filterTypes as filter}
							<Select.Item class="select-item" value={filter.value} label={$_(filter.label)}>
								{$_(filter.label)}
								<Select.ItemIndicator class="ml-auto" asChild={false}>
									<Check />
								</Select.ItemIndicator>
							</Select.Item>
						{/each}
					</Select.Content>
					<Select.Input name="favoriteFruit" />
				</Select.Root>
			</div>
			<div class="filter">
				<Select.Root
					items={filterOrders}
					selected={filterOrders.find((l) => l.value === filterquery.order)}
					onSelectedChange={(e) => {
						if (e) {
							filterquery.order = e.value;
						}
					}}
				>
					<Select.Trigger class="select-trigger" aria-label="Choisissez l'ordre du tri">
						<ArrowDown10 class="icon" />
						<div class="text">{$_('songs_page.filter.sort_order')}</div>
					</Select.Trigger>
					<Select.Content class="select-content" sideOffset={8} transition={flyAndScale}>
						{#each filterOrders as filter}
							<Select.Item class="select-item" value={filter.value} label={$_(filter.label)}>
								{$_(filter.label)}
								<Select.ItemIndicator class="ml-auto" asChild={false}>
									<Check />
								</Select.ItemIndicator>
							</Select.Item>
						{/each}
					</Select.Content>
					<Select.Input name="favoriteFruit" />
				</Select.Root>
			</div>
			<input bind:value={searchInput} type="search" name="search" placeholder={$_('search')} />
		</div>

		<div
			class="songlist"
			style="--tracklist-index-column-width: {(list.activeList.tracks.length.toString().length *
				16) /
				2}px"
		>
			{#each applyFilters(list.tracks) as song, i}
				<Song {song} {i} {ctx} {manager} bind:searchq={searchInput} />
			{/each}
		</div>
	{/if}
</div>

<style>
	h1 {
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
	}

	.track_counts {
		padding-bottom: 2em;
	}

	.filters {
		padding-top: 2em;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		gap: 1em;
	}

	.filter :global(.select-trigger) {
		-webkit-appearance: none;
		appearance: none;
		display: flex;
		justify-content: center;
		align-content: center;
		gap: 0.5em;
		padding-inline: 2em;
		padding-block: 0.3em;
		font-size: 0.875em;
		border: none;
		background-color: var(--highlight);
		cursor: pointer;
		border-radius: 8px;
		color: var(--fg);
		width: fit-content;
		height: 2.5em;
	}

	:global(.select-trigger .text) {
		height: 100%;
		width: 100%;
		display: flex;
		align-items: center;
	}

	:global(.select-trigger svg.icon) {
		opacity: 0.4;
	}

	.filter :global(.select-trigger:active) {
		transform: scale(0.99);
	}

	:global(.select-item) {
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: space-between;
		padding: 0.2em;
		padding-inline: 0.5em;
		height: 2em;
		border-radius: 4px;
	}

	:global(.select-item:hover) {
		background: var(--highlight);
	}

	:global(.select-content) {
		width: fit-content;
		border-radius: 9px;
		border: 2px solid rgba(100, 100, 100, 0.18);
		background: var(--bg);
		padding: 0.3em;
	}

	input[type='search'] {
		-webkit-appearance: none;
		appearance: none;
		padding-inline: 0.5em;
		padding-block: 0.7em;
		border-radius: 4px;
		border: 0px;
		background: var(--highlight);
		color: var(--fg);
	}

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

	.quick-actions button .icon {
		height: 100%;
		display: flex;
		align-items: center;
	}

	.quick-actions {
		display: flex;
		gap: 1em;
	}

	.quick-actions button:active {
		transform: scale(0.98);
	}

	.songlist {
		width: 100%;
		padding-top: 2em;
	}
</style>
