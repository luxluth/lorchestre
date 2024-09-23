<script lang="ts">
	import ImagePlus from 'lucide-svelte/icons/image-plus';
	import { open } from '@tauri-apps/plugin-dialog';
	import { getAppConfig } from '$lib/config.svelte';
	import type { Color } from '$lib/type';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import Plus from 'lucide-svelte/icons/plus';
	import { getListCreator } from '$lib/listCreate.svelte';
	import { getSearch } from '$lib/search.svelte';
	import { _ } from 'svelte-i18n';
	import { getCoverUri } from '$lib/utils';
	import Song from '$lib/components/Song.svelte';
	import { getManager } from '$lib/manager.svelte';
	import { getCtx } from '$lib/ctx.svelte';
	import { getMedia } from '$lib/media.svelte';

	const lc = getListCreator();
	const search = getSearch();
	let searchQuery = $state('');
	const manager = getManager();
	const ctx = getCtx();
	const media = getMedia();

	const config = getAppConfig();
	let color: Color | null = $state(null);
	let loading = $state(false);

	async function choose_image_cover() {
		loading = true;
		lc.ImageData = '';
		try {
			let file = await open({
				multiple: false,
				directory: false,
				filters: [
					// TODO: localize here
					{
						name: 'Choose the playlist cover',
						extensions: ['png']
					}
				]
			});

			if (file) {
				lc.xCoverPath = file;
				const endpoint = config.getDaemonEndpoint();
				const response = await fetch(`http://${endpoint}/get_image`, {
					body: JSON.stringify({
						path: file
					}),
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					}
				});

				if (response.ok) {
					const res = await response.json();
					color = res.color as Color;
					lc.ImageData = `data:image/png;base64,${res.data}`;
				}
			}
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	}

	async function play(id: number) {
		const track = media.getTrack(lc.addedTracks[id]);
		if (track) await manager.play(track);
	}
</script>

<div
	class="page ns"
	style={color ? `--clr: rgb(${color.r}, ${color.g}, ${color.b})` : ''}
	class:creating={lc.creating}
>
	<h1>Create a new playlist</h1>
	<div class="top">
		<div
			class="add_cover"
			style={lc.ImageData.length > 0 ? `background-image: url("${lc.ImageData}")` : ''}
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
			{#if lc.ImageData.length <= 0}
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
			<input type="text" class="list_name" bind:value={lc.Name} />
			<input
				type="text"
				class="desc"
				placeholder="playlist description"
				bind:value={lc.Description}
			/>
			<!-- TODO: localize -->
			<div class="submit">
				<button
					class="btn"
					class:inactive={lc.addedTracks.length <= 0}
					onclick={async () => {
						await lc.create();
					}}>Create</button
				>
			</div>
		</div>
	</div>

	<div class="selectedTracks">
		{#each lc.addedTracks as track_path, i}
			{@const song = media.getTrack(track_path)}
			{#if song}
				<Song
					{song}
					{i}
					{ctx}
					{manager}
					searchq={''}
					onPlay={play}
					state="remove"
					onRemove={(path) => {
						lc.addedTracks = lc.addedTracks.filter((p) => p != path);
					}}
				/>
			{/if}
		{/each}
	</div>

	<div class="search">
		<input
			type="search"
			name="search"
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
				{#if !lc.addedTracks.includes(track.file_path)}
					<div
						class="track"
						ondblclick={() => {
							lc.addedTracks.push(track.file_path);
						}}
						onkeydown={(e) => {
							if (e.key.toLowerCase() == 'enter') {
								lc.addedTracks.push(track.file_path);
							}
						}}
						role="button"
						tabindex="0"
					>
						<div
							class="cover"
							style="background-image: url({getCoverUri(track.album_id, track.cover_ext, config)});"
						></div>
						<div class="infos">
							<div class="title">{track.title}</div>
							<div class="artist">{track.artists[0]}</div>
						</div>
						<button
							tabindex="-1"
							onclick={() => {
								lc.addedTracks.push(track.file_path);
							}}><Plus color={'var(--fg)'} /></button
						>
					</div>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.submit {
		display: flex;
		justify-content: flex-end;
		padding-top: 1em;
	}

	.creating {
		pointer-events: none;
		opacity: 0.5;
	}
	.selectedTracks {
		padding-top: 2em;
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
