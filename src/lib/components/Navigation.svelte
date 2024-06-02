<script lang="ts">
	import Clock from 'lucide-svelte/icons/clock';
	import Settings from 'lucide-svelte/icons/settings';
	import Library from 'lucide-svelte/icons/library';

	let { pathId, platform }: { pathId: string; platform: string } = $props();
	import { _ } from 'svelte-i18n';
</script>

<div class="nav ns" style="--top-by: {platform === 'macos' ? '1em' : '1em'}">
	<!-- {#if platform === 'macos'} -->
	<!-- 	<div class="dragzone" data-tauri-drag-region></div> -->
	<!-- {/if} -->
	<section class="search">
		<input type="search" name="search" placeholder={$_('search')} />
	</section>
	<section>
		<h4>{$_('bib')}</h4>
		<div class="links">
			<a href="/recently-added" class:active={pathId == '/recently-added'}>
				<Clock size={'1em'} />
				{$_('recent-added')}</a
			>
			<a href="/" class:active={pathId == '/'}>
				<Library size={'1em'} />
				{$_('albums')}</a
			>
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
	.dragzone {
		z-index: 1000;
		position: absolute;
		top: 0;
		left: 0;
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
		padding-top: var(--top-by);
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
		opacity: 1;
	}
</style>
