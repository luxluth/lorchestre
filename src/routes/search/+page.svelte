<script lang="ts">
	import SearchAlbum from '$lib/components/SearchAlbum.svelte';
	import SearchPlaylist from '$lib/components/SearchPlaylist.svelte';
	import Track from '$lib/components/SearchTrack.svelte';
	import type AppConfig from '$lib/config.svelte';
	import type Ctx from '$lib/ctx.svelte';
	import type Manager from '$lib/manager.svelte';
	import type MediaState from '$lib/media.svelte';
	import type List from '$lib/playlist.svelte';
	import type SearchSupervisor from '$lib/search.svelte';
	import { setTitle } from '$lib/utils';
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';

	let search = getContext<SearchSupervisor>('ss');
	let config = getContext<AppConfig>('appconf');
	let media = getContext<MediaState>('media');
	let list = getContext<List>('list');
	let manager = getContext<Manager>('manager');
	let ctx = getContext<Ctx>('ctx');

	$effect(() => {
		setTitle(
			`L'orchestre -- ${$_('search')} ${search.query.length > 0 ? '-- ' + search.query : ''}`
		);
	});
</script>

<div class="search ns" class:active={search.query.length > 0}>
	<h1>{$_('search')}</h1>
	{#if search.query.length > 0}
		<p class="tip">Résultat pour le terme <b>{search.query}</b></p>
	{:else}
		<p class="tip">Commencer à écrire pour chercher...</p>
	{/if}

	<div class="searchres">
		{#if search.isEmpty() && search.query.length > 0}
			Aucun résultat
		{:else if !search.isEmpty()}
			<div class="results">
				{#if search.results.tracks.length > 0}
					<h2>{$_('songs')}</h2>
					<div class="tracks">
						{#each search.results.tracks as track}
							<Track {track} {config} {ctx} {manager} />
						{/each}
					</div>
				{/if}
				{#if search.results.albums.length > 0}
					<h2>{$_('albums')}</h2>
					<div class="albums">
						{#each search.results.albums as album}
							<SearchAlbum {album} {config} {media} {ctx} {manager} />
						{/each}
					</div>
				{/if}
				{#if search.results.playlists.length > 0}
					<h2>{$_('playlists')}</h2>
					<div class="playlists">
						{#each search.results.playlists as playlist}
							<SearchPlaylist {playlist} {list} />
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.results h2 {
		padding-top: 2em;
		padding-bottom: 1em;
	}

	.albums {
		display: flex;
		flex-wrap: wrap;
		gap: 1em;
	}

	.tracks {
		display: grid;
		grid-template-columns: repeat(auto-fill, 400px);
		column-gap: 1em;
		row-gap: 1em;
		padding-bottom: 2em;
	}

	h1 {
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
	}

	.tip {
		font-style: italic;
		text-wrap: wrap;
		word-wrap: break-word;
		width: 30vw;
	}

	.searchres {
		margin-top: 2em;
	}
</style>
