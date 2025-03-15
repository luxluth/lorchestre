<script lang="ts">
	import {
		FilterOrder,
		FilterType,
		PlayingMode,
		ToastKind,
		type Playlist,
		type QueueTrack,
		type Track
	} from '$lib/type';
	import { _ } from 'svelte-i18n';
	import Play from 'lucide-svelte/icons/play';
	import Shuffle from 'lucide-svelte/icons/shuffle';
	import { Select } from 'bits-ui';
	import Filter from 'lucide-svelte/icons/filter';
	import Check from 'lucide-svelte/icons/check';
	import ListPlus from 'lucide-svelte/icons/list-plus';
	import Plus from 'lucide-svelte/icons/plus';
	import Bolt from 'lucide-svelte/icons/bolt';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import { flyAndScale } from '$lib/utils/transitions';
	import ArrowDown10 from 'lucide-svelte/icons/arrow-down-1-0';
	import Song from '$lib/components/Song.svelte';
	import { getCoverUri, setRandomId, setTitle, sortTracksByDate } from '$lib/utils';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getList } from '$lib/playlist.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { page } from '$app/state';
	import ShallowSong from '$lib/components/ShallowSong.svelte';
	import { getSearch } from '$lib/search.svelte';
	import { getAppConfig } from '$lib/config.svelte';
	import type { ListPayload } from '$lib/listCreate.svelte';
	import { getToastManager } from '$lib/toast.svelte';
	import { goto } from '$app/navigation';
	import { dndzone, type DndEvent } from 'svelte-dnd-action';
	import { flip } from 'svelte/animate';

	type DisplayMode = 'normal' | 'edit';

	const list = getList();
	const manager = getManager();
	const ctx = getCtx();
	const media = getMedia();
	const search = getSearch();
	const config = getAppConfig();
	const tm = getToastManager();
	let filterquery = list.filters;
	let mode: DisplayMode = $state('normal');

	let playlistData: Playlist | null = $derived(page.data.list);
	let selections: string[] = $state([]);

	$effect(() => {
		list.activeList = playlistData?.path_base64 ?? null;
	});

	let tracks: QueueTrack[] = $state([]);

	$effect(() => {
		tracks = playlistData
			? playlistData.tracks
					.map((track) => {
						return setRandomId(media.getTrack(track) as Track);
					})
					.filter((f) => typeof f != 'undefined')
			: [];
	});

	async function playAll() {
		let songs = [...applyFilters(tracks)];
		let song = songs.shift() as Track;
		manager.pmode = PlayingMode.Normal;
		manager.play(song);
		manager.clearQueue();
		manager.addManyToQueue(songs);
	}

	async function playAllShuffle() {
		let songs = applyFilters(tracks);
		await manager.shufflePlay(songs);
	}

	function handleDndConsiderColumns(e: CustomEvent<DndEvent<QueueTrack>>) {
		tracks = e.detail.items;
	}
	function handleDndFinalizeColumns(e: CustomEvent<DndEvent<QueueTrack>>) {
		tracks = e.detail.items;
	}

	const flipDurationMs = 200;

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

	function applyFilterQuery(tracks: Track[]): Track[] {
		let r = [];
		switch (filterquery.type) {
			case FilterType.Alphabetic:
				r = sortTracksByTitle(tracks);
				break;
			case FilterType.TimeBased:
				r = sortTracksByDate(tracks);
				break;
			case FilterType.NoFilter:
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
		{ value: FilterType.NoFilter, label: 'songs_page.filter.type.no_filter' },
		{ value: FilterType.Alphabetic, label: 'songs_page.filter.type.alpha' },
		{ value: FilterType.TimeBased, label: 'songs_page.filter.type.date' }
	];

	const filterOrders = [
		{ value: FilterOrder.Ascendant, label: 'songs_page.filter.order.asc' },
		{ value: FilterOrder.Descendant, label: 'songs_page.filter.order.desc' }
	];

	let searchInput = $state('');
	let filteredTracks = $derived(applyFilters(tracks));

	async function play(i: number) {
		let tracks = filteredTracks.slice(i, filteredTracks.length);
		let song = tracks.shift();
		if (song) {
			manager.play(song);
			manager.clearQueue();
			manager.addManyToQueue(tracks);
		}
	}

	$effect(() => {
		setTitle(
			`${$_('playlist').toLowerCase()} — ${playlistData ? (playlistData.metadata['Name'] ?? '+£@&0m') : ''} - L'orchestre`
		);
	});

	function toggleSelect(path: string) {
		if (selections.includes(path)) {
			selections = selections.filter((s) => s !== path);
		} else {
			selections = [...selections, path];
		}
	}

	document.addEventListener('pagechanged', () => {
		mode = 'normal';
		selections = [];
	});

	/// /playlist/update/:path
	async function saveChanges() {
		const endpoint = config.getDaemonEndpoint();
		let req_url = `http://${endpoint}/playlist/update/${playlistData?.path_base64}`;
		if (listName.length > 0) {
			updateting = true;
			let loadingToastId = tm.new(
				ToastKind.Loading,
				`${$_('playlist_page.updating_message')} \`` + listName + '`'
			);
			const payload: ListPayload = {
				meta: [],
				tracks: []
			};

			payload.meta.push(['Name', listName]);
			payload.meta.push(['Description', listDesc]);

			for (const track of tracks) {
				payload.tracks.push(track.file_path);
			}

			await fetch(req_url, {
				method: 'PUT',
				body: JSON.stringify(payload),
				headers: {
					'Content-Type': 'application/json'
				}
			})
				.then(async (response) => {
					if (response.ok) {
						tm.new(ToastKind.Simple, $_('playlist_page.success_update'));
						await goto('/list/' + playlistData?.path_base64, {
							replaceState: true,
							invalidateAll: true
						});
					}
				})

				.catch((error) => {
					tm.close(loadingToastId);
					tm.new(ToastKind.Error, `${$_('playlist_page.failed_update')}: ${error}`);
				})
				.finally(() => {
					tm.close(loadingToastId);
					updateting = false;
				});
		}
	}

	function removeSelections() {
		tracks = tracks.filter((t) => !selections.includes(t.file_path));
		selections = [];
	}

	let updateting = $state(false);
	let listDesc = $state('');
	let listName = $state('+£@&0m');

	$effect(() => {
		listDesc = playlistData?.metadata['Description'] ?? listDesc;
		listName = playlistData?.metadata['Name'] ?? listName;
	});

	let searchQuery = $state('');
</script>

<div class="page ns">
	<button
		class="edit btn"
		class:active={mode === 'edit'}
		onclick={async () => {
			if (mode == 'edit') {
				await saveChanges();
				mode = 'normal';
			} else {
				mode = 'edit';
			}
		}}
	>
		<div class="icon">
			<Bolt />
		</div>
	</button>
	{#if playlistData}
		{#if mode === 'normal'}
			<h1>{playlistData.metadata['Name'] ?? '+£@&0m'}</h1>
			<p class="track_counts">
				{tracks.length}
				{tracks.length > 1 ? $_('stats_page.songs') : $_('stats_page.song')}
			</p>

			<p class="description">{playlistData.metadata['Description'] ?? ''}</p>
		{:else if mode === 'edit'}
			<input type="text" class="list_name" bind:value={listName} />
			<input type="text" class="desc" placeholder="playlist description" bind:value={listDesc} />
		{/if}
		{#if mode == 'edit'}
			<button
				data-kind="desctructive"
				class="remove_selection btn"
				class:inactive={selections.length == 0}
				onclick={() => removeSelections()}
			>
				<div class="icon">
					<Trash2 size={'1em'} />
				</div>
				{$_('playlist_page.remove_selections')} [{selections.length}]</button
			>
		{/if}
		{#if mode === 'normal'}
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
						type="single"
						items={filterTypes}
						value={filterquery.type}
						onValueChange={(e: any) => {
							if (e) {
								filterquery.type = e;
							}
						}}
					>
						<Select.Trigger class="select-trigger" aria-label="Choisissez un type de filtre">
							<Filter class="icon" />
							<div class="text">{$_('songs_page.filter.filter_to_apply')}</div>
						</Select.Trigger>
						<Select.Portal>
							<Select.Content class="select-content" sideOffset={8}>
								{#each filterTypes as filter}
									<Select.Item class="select-item" value={filter.value} label={$_(filter.label)}>
										{#snippet children({ selected })}
											{$_(filter.label)}
											{#if selected}
												<Check />
											{/if}
										{/snippet}
									</Select.Item>
								{/each}
							</Select.Content>
						</Select.Portal>
					</Select.Root>
				</div>
				<div class="filter">
					<Select.Root
						type="single"
						items={filterOrders}
						value={filterquery.order}
						onValueChange={(e: any) => {
							if (e) filterquery.order = e;
						}}
					>
						<Select.Trigger class="select-trigger" aria-label="Choisissez l'ordre du tri">
							<ArrowDown10 class="icon" />
							<div class="text">{$_('songs_page.filter.sort_order')}</div>
						</Select.Trigger>
						<Select.Portal>
							<Select.Content class="select-content" sideOffset={8}>
								{#each filterOrders as filter}
									<Select.Item class="select-item" value={filter.value} label={$_(filter.label)}>
										{#snippet children({ selected })}
											{$_(filter.label)}
											{#if selected}
												<Check />
											{/if}
										{/snippet}
									</Select.Item>
								{/each}
							</Select.Content>
						</Select.Portal>
					</Select.Root>
				</div>
				<input bind:value={searchInput} type="search" name="search" placeholder={$_('search')} />
			</div>
		{/if}

		<div
			class="songlist"
			style="--tracklist-index-column-width: {(tracks.length.toString().length * 16) / 2}px"
		>
			{#if mode === 'normal'}
				{#each filteredTracks as song, i}
					<Song
						{song}
						{i}
						{ctx}
						{manager}
						bind:searchq={searchInput}
						onPlay={play}
						state="normal"
					/>
				{/each}
			{:else if mode === 'edit'}
				<section
					style="width: 100%; height: 100%;"
					use:dndzone={{
						items: tracks,
						flipDurationMs,
						type: 'columns',
						dropTargetStyle: {
							backgroundColor: 'var(--bg)',
							opacity: '0.5'
						}
					}}
					onconsider={handleDndConsiderColumns}
					onfinalize={handleDndFinalizeColumns}
				>
					{#each tracks as song (song.id)}
						<div class="wrapper" style="width: 100%;" animate:flip={{ duration: flipDurationMs }}>
							<ShallowSong
								{song}
								selected={selections.includes(song.file_path)}
								toggleSelection={(p) => {
									toggleSelect(p);
								}}
							/>
						</div>
					{/each}
				</section>
			{/if}
		</div>
		{#if mode === 'edit'}
			<section class="adding_tracks">
				<h3><ListPlus /> {$_('playlist_page.add_tracks')}</h3>

				<div class="search">
					<input
						type="search"
						name="search"
						class="search_tracks"
						placeholder={$_('search_page.no_ipt')}
						bind:value={searchQuery}
						onkeyup={() => {
							search.localSearch(searchQuery);
						}}
						onkeydown={(e) => {
							if (e.key.toLowerCase() === 'enter') {
								search.localSearch(searchQuery);
							}
						}}
					/>
				</div>
				{#if search.local_results.tracks.length > 0}
					<div class="search_response">
						{#each search.local_results.tracks as track}
							{#if typeof tracks.find((t) => t.file_path == track.file_path) == 'undefined'}
								<div
									class="track"
									ondblclick={() => {
										tracks.push(setRandomId(track));
									}}
									onkeydown={(e) => {
										if (e.key.toLowerCase() == 'enter') {
											tracks.push(setRandomId(track));
										}
									}}
									role="button"
									tabindex="0"
								>
									<div
										class="cover"
										style="background-image: url({getCoverUri(
											track.album_id,
											track.cover_ext,
											config
										)});"
									></div>
									<div class="infos">
										<div class="title">{track.title}</div>
										<div class="artist">{track.artists[0]}</div>
									</div>
									<button
										tabindex="-1"
										onclick={() => {
											tracks.push(setRandomId(track));
										}}><Plus color={'var(--fg)'} /></button
									>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</section>
		{/if}
	{/if}
</div>

<style>
	.remove_selection {
		margin-top: 1em;
	}

	input[type='search'].search_tracks {
		-webkit-appearance: none;
		appearance: none;
		padding-inline: 0.5em;
		padding-block: 0.7em;
		border-radius: 4px;
		border: 0px;
		background: var(--highlight);
		color: var(--fg);
		width: 100%;
		margin-block: 2em;
	}

	.search_response {
		width: 100%;
	}

	.track {
		display: flex;
		gap: 1em;
		padding: 0.5em;
		align-items: center;
		border-radius: 8px;
		padding-right: 1.5em;
		margin-bottom: 1em;
	}

	.track:hover {
		background: rgba(100, 100, 100, 0.18);
	}

	.track .cover {
		min-width: 3em;
		aspect-ratio: 1/1;
		background-size: cover;
		border-radius: 4px;
	}

	.track .infos {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.track button {
		cursor: pointer;
		background: none;
		border: none;
		-webkit-appearance: none;
		appearance: none;
	}

	.description {
		opacity: 0.7;
		padding-bottom: 2em;
	}
	.edit {
		top: 4.5em;
		right: 1em;
		position: absolute;
	}

	.edit.active {
		background: var(--fg);
		color: var(--bg);
	}

	.adding_tracks {
		margin-top: 2em;
		width: 100%;
	}

	.desc {
		background: none;
		border: none;
		width: 100%;
		color: var(--fg);
	}

	.adding_tracks h3 {
		display: flex;
		align-items: center;
		gap: 0.2em;
	}

	h1 {
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
	}

	.list_name {
		color: var(--fg);
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
		width: 100%;
		background: none;
		border: none;
	}

	.list_name:focus,
	.desc:focus {
		outline: none;
	}

	.track_counts {
		padding-bottom: 0.5em;
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

	:global(.select-item[data-highlighted]) {
		background: var(--highlight);
	}

	:global(.select-item:hover) {
		background: var(--highlight);
	}

	:global(.select-content) {
		width: var(--bits-select-anchor-width);
		min-width: var(--bits-select-anchor-width);
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
