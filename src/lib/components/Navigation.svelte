<script lang="ts">
	import Settings from 'lucide-svelte/icons/settings';
	import Library from 'lucide-svelte/icons/library';
	import ListMusic from 'lucide-svelte/icons/list-music';
	import Flame from 'lucide-svelte/icons/flame';
	import Plus from 'lucide-svelte/icons/plus';
	import Search from 'lucide-svelte/icons/search';

	let { pathId, platform }: { pathId: string; platform: string } = $props();
	import { _ } from 'svelte-i18n';
	import List from 'lucide-svelte/icons/list';
	import { getMedia } from '$lib/media.svelte';
	import { getList } from '$lib/playlist.svelte';
	import Commands from './Commands.svelte';

	let media = getMedia();
	let list = getList();
</script>

<div class="nav ns">
	{#if platform === 'macos'}
		<div class="dragzone" data-tauri-drag-region></div>
	{/if}
	<section class="quick-actions" data-tauri-drag-region>
		<Commands />
		<a href="/stats" class:active={pathId == '/stats'}>
			<Flame size={'20px'} />
		</a>
		<a href="/search" class:active={pathId == '/search'}>
			<Search size={'20px'} />
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
		<div class="links">
			{#each media.playlists as playlist}
				<a
					href="/list/{playlist.path_base64}"
					class:active={pathId == '/list/[id]' && list.activeList === playlist.path_base64}
				>
					<List size={'1em'} />
					{playlist.metadata['Name'] ?? '+£@&0m'}
				</a>
			{/each}
			<a class="create_list" href="/create_list">
				<div class="icon">
					<Plus size={'1em'} />
				</div>
				New playlist
			</a>
		</div>
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
	.quick-actions {
		display: flex;
		gap: 0.5em;
	}

	.quick-actions a {
		color: var(--fg);
		width: 2.5em;
		height: 2.5em;
		display: flex;
		justify-content: center;
		align-items: center;
		opacity: 0.8;
		transition: opacity 0.1s ease-in-out;
		cursor: pointer;
		padding: 0.2em;
		background: none;
		border: none;
		border-radius: 4px;
		border: 1px solid rgba(100, 100, 100, 0);
	}

	.quick-actions a:active {
		transform: scale(0.95);
	}

	.quick-actions a:hover {
		opacity: 1;
	}

	.quick-actions a.active {
		opacity: 1;
		border: 1px solid rgba(100, 100, 100, 0.18);
		background: rgba(100, 100, 100, 0.18);
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
		color: var(--fg);
		opacity: 1;
	}

	.create_list {
		display: flex;
		margin-top: 0.5em;
		background: var(--highlight);
	}

	.create_list:active {
		transform: scale(0.98);
	}
</style>
