<script lang="ts">
	import { _, locale } from 'svelte-i18n';
	import type AppConfig from '$lib/config.svelte';
	import { getContext } from 'svelte';
	import type MediaState from '$lib/media.svelte';
	import FolderSync from 'lucide-svelte/icons/folder-sync';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';

	let { appConf }: { appConf: AppConfig } = $props();
	let media = getContext<MediaState>('media');
</script>

<section class="lang ns">
	<h4>Sync the music library with local files</h4>
	<button
		class="btn"
		class:inactive={media.updatingmedia}
		class:syncing={media.updatingmedia}
		onclick={() => {
			(() => {
				let endpoint = appConf.getMUDEndpoint();
				fetch(`http://${endpoint}/updatemusic`, { method: 'PUT' }).catch((error) => {
					console.error('Fetch error:', error);
				});
			})();
		}}
	>
		<div class="icon">
			{#if media.updatingmedia}
				<LoaderCircle size={'1em'} />
			{:else}
				<FolderSync size={'1em'} />
			{/if}
		</div>
		Start the sync
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
