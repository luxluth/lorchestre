<script lang="ts">
	import { getMedia } from '$lib/media.svelte';
	import ImagePlus from 'lucide-svelte/icons/image-plus';
	import { open } from '@tauri-apps/plugin-dialog';
	import { getAppConfig } from '$lib/config.svelte';
	import type { Color } from '$lib/type';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';

	let playlistData: {
		metadata: Record<string, string>;
		tracks: string[];
	} = $state({
		metadata: {},
		tracks: []
	});

	const media = getMedia();
	const config = getAppConfig();
	let Name = $state(`Playlist N°${media.playlists.length + 1}`);
	let Description = $state('');
	let ImageData = $state('');
	let color: Color | null = $state(null);
	let loading = $state(false);

	async function choose_image_cover() {
		loading = true;
		ImageData = '';
		await open({
			multiple: false,
			directory: false,
			filters: [
				{
					name: 'Choose the playlist cover',
					extensions: ['png']
				}
			]
		})
			.then(async (file) => {
				if (file) {
					if (file.path) {
						const endpoint = config.getDaemonEndpoint();
						const response = await fetch(`http://${endpoint}/get_image`, {
							body: JSON.stringify({
								path: file?.path
							}),
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							}
						});

						if (response.ok) {
							const res = await response.json();
							color = res.color as Color;
							ImageData = `data:image/png;base64,${res.data}`;
						}
					}
				}
			})
			.catch((e) => {
				console.log(e);
			})
			.finally(() => {
				loading = false;
			});
	}
</script>

<div class="page ns" style={color ? `--clr: rgb(${color.r}, ${color.g}, ${color.b})` : ''}>
	<h1>Create a new playlist</h1>
	<div class="top">
		<div
			class="add_cover"
			style={ImageData.length > 0 ? `background-image: url("${ImageData}")` : ''}
			onclick={async () => {
				await choose_image_cover();
			}}
			onkeydown={async (e) => {
				if (e.key.toLowerCase() === 'enter') {
					await choose_image_cover();
				}
			}}
			role="button"
			tabindex="0"
		>
			{#if ImageData.length <= 0}
				<div class="icon" class:loading>
					{#if loading}
						<LoaderCircle size="1.5em" />
					{:else}
						<ImagePlus size={'1.5em'} />
					{/if}
				</div>
			{/if}
		</div>
		<div class="text_inputs">
			<input type="text" class="list_name" bind:value={Name} />
			<input type="text" class="desc" placeholder="playlist description" bind:value={Description} />
		</div>
	</div>
</div>

<style>
	.add_cover {
		width: 15vw;
		height: 15vw;
		background: var(--fg);
		aspect-ratio: 1/1;
		border-radius: 8px;
		box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px;
		color: var(--bg);
		position: relative;
		background-size: cover;
	}

	@keyframes rotate {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.add_cover .icon {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.icon.loading {
		animation: rotate 500ms infinite ease-in-out;
	}

	.add_cover:active {
		transform: scale(0.995);
	}

	.top {
		display: flex;
		gap: 1em;
	}

	h1 {
		padding-bottom: 1em;
	}

	.list_name {
		color: var(--fg);
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
		width: 100%;
		background: none;
		border: none;
	}

	.desc {
		background: none;
		border: none;
		width: 100%;
		color: var(--fg);
	}

	.list_name:focus,
	.desc:focus {
		outline: none;
	}
</style>
