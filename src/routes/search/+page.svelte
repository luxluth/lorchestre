<script lang="ts">
	import SearchAlbum from '$lib/components/SearchAlbum.svelte';
	import Track from '$lib/components/SearchTrack.svelte';
	import { getAppConfig } from '$lib/config.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getManager } from '$lib/manager.svelte';
	import { getMedia } from '$lib/media.svelte';
	import { getSearch } from '$lib/search.svelte';
	import { setTitle } from '$lib/utils';
	import { _ } from 'svelte-i18n';

	let search = getSearch();
	let config = getAppConfig();
	let media = getMedia();
	let manager = getManager();
	let ctx = getCtx();

	$effect(() => {
		setTitle(`${$_('search')} ${search.query.length > 0 ? '— ' + search.query : ''} — L'orchestre`);
	});
</script>

<div class="search ns" class:active={search.query.length > 0}>
	<input
		type="search"
		name="search"
		placeholder={$_('search_page.no_ipt')}
		bind:value={search.query}
		onkeyup={() => {
			search.search();
		}}
		onkeydown={(e) => {
			if (e.key.toLowerCase() === 'enter') {
				search.search();
			}
		}}
		autofocus
	/>
	{#if search.query.length > 0}
		<p class="tip">{$_('search_page.res_msg')} <b>{search.query}</b></p>
	{/if}

	<div class="searchres">
		{#if search.isEmpty() && search.query.length > 0}
			{$_('search_page.no_res')}
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

	input[type='search'] {
		-webkit-appearance: none;
		appearance: none;
		padding-inline: 0.5em;
		padding-block: 0.7em;
		border-radius: 4px;
		border: 0px;
		background: var(--highlight);
		color: var(--fg);
		width: 100%;
		margin-bottom: 2em;
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
