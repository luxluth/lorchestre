<script lang="ts">
	import { _ } from 'svelte-i18n';
	import FolderSync from 'lucide-svelte/icons/folder-sync';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import { invoke } from '@tauri-apps/api/core';
	import { getMedia } from '$lib/media.svelte';

	let media = getMedia();
</script>

<section class="lang ns">
	<h4>{$_('settings_page.section_mlib.sync.title')}</h4>
	<button
		class="btn"
		class:inactive={media.updatingmedia}
		class:syncing={media.updatingmedia}
		onclick={async () => {
			await invoke('sync_music');
		}}
	>
		<div class="icon">
			{#if media.updatingmedia}
				<LoaderCircle size={'1em'} />
			{:else}
				<FolderSync size={'1em'} />
			{/if}
		</div>
		{$_('settings_page.section_mlib.sync.sync_btn')}
	</button>
</section>

<style>
	h4 {
		padding-top: 1.5em;
		padding-bottom: 0.5em;
		opacity: 0.5;
	}

	button.syncing .icon {
		animation: rotate 500ms infinite ease-in-out;
	}

	@keyframes rotate {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
