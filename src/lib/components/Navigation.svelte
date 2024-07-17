<script lang="ts">
	import Settings from 'lucide-svelte/icons/settings';
	import Library from 'lucide-svelte/icons/library';
	import ListMusic from 'lucide-svelte/icons/list-music';
	import Flame from 'lucide-svelte/icons/flame';

	let { pathId, platform }: { pathId: string; platform: string } = $props();
	import { _ } from 'svelte-i18n';
	import List from 'lucide-svelte/icons/list';
	import { goto } from '$app/navigation';
	import { getMedia } from '$lib/media.svelte';
	import { getList } from '$lib/playlist.svelte';
	import { getSearch } from '$lib/search.svelte';

	let media = getMedia();
	let list = getList();
	let search = getSearch();
</script>

<div class="nav ns">
	{#if platform === 'macos'}
		<div class="dragzone" data-tauri-drag-region></div>
	{/if}
	<section class="search" data-tauri-drag-region>
		<input
			type="search"
			name="search"
			placeholder={$_('search')}
			bind:value={search.query}
			onkeyup={() => {
				search.search();
			}}
			onkeydown={(e) => {
				if (e.key.toLowerCase() === 'enter') {
					search.search();
					goto('/search');
				}
			}}
		/>
		<a href="/stats" class:active={pathId == '/stats'}>
			<Flame size={'1em'} />
		</a>
	</section>
	<section>
		<h4>{$_('bib')}</h4>
		<div class="links">
			<a href="/" class:active={pathId == '/'}>
				<Library size={'1em'} />
				{$_('albums')}</a
			>
			<a href="/songs" class:active={pathId == '/songs'}>
				<ListMusic size={'1em'} />
				{$_('songs')}
			</a>
		</div>
	</section>
	<section>
		<h4>{$_('playlist')}</h4>
		{#if media.playlists.length === 0}
			<p class="np">{$_('no_playlist_found')}</p>
		{:else}
			<div class="links">
				{#each media.playlists as playlist}
					<a
						href="/list"
						class:active={pathId == '/list' && list.activeList?.id === playlist.id}
						onclick={() => {
							list.activeList = playlist;
						}}
					>
						<List size={'1em'} />
						{playlist.name}
					</a>
				{/each}
			</div>
		{/if}
	</section>
	<section class="settings">
		<div class="links">
			<a href="/settings" class:active={pathId == '/settings'}>
				<Settings size={'1em'} />
				{$_('settings')}</a
			>
		</div>
	</section>
</div>

<style>
	.np {
		padding: 1em;
	}

	.search {
		display: flex;
		gap: 0.5em;
	}

	.search a {
		color: var(--fg);
		opacity: 0.7;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 0.5em;
		padding-inline: 0.6em;
		padding-block: 0.6em;
		border-radius: 6px;
		transition: all ease-in-out 0.1s;
	}

	.search a.active {
		background: var(--highlight);
		opacity: 1;
		color: var(--brand-color);
	}

	.dragzone {
		height: 2em;
		background-color: none;
		width: 100%;
	}

	section.settings {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		justify-content: flex-end;
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
	}

	.nav {
		padding-block: 2em;
		padding-top: 1em;
		padding-inline: 1em;
		display: flex;
		flex-direction: column;
		gap: 1.5em;
		height: 100%;
		overflow: auto;
		position: relative;
	}
	h4 {
		opacity: 0.5;
		padding-bottom: 0.5em;
	}

	section div.links {
		display: flex;
		flex-direction: column;
		gap: 0.3em;
	}

	section div.links a {
		color: var(--fg);
		opacity: 0.7;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 0.5em;
		padding-inline: 1em;
		padding-block: 0.6em;
		border-radius: 6px;
		transition: all ease-in-out 0.1s;
	}

	section div.links a:hover {
		opacity: 0.9;
	}

	section div.links a.active {
		background: var(--highlight);
		color: var(--brand-color);
		opacity: 1;
	}
</style>
