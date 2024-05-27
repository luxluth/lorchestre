<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { toast, player } from '$lib/events';
	import { Play } from 'lucide-svelte';
	import { ToastKind, type AudioMedia, type Media } from '$lib/type';

	let media = $state<Media>();

	$effect(() => {
		(async () => {
			toast.show("Librarie en cours d'indexation", ToastKind.Loading);
			media = await invoke<Media>('index');
		})();
	});

	function play(audio: AudioMedia) {
		player.play(audio);
	}
</script>

<h1 class="__page_title ns">Bibliothèque</h1>
<p class="__page_subtitle ns">Explorer votre musique</p>

{#if media}
	<div class="__medias">
		{#each media.audios as audio}
			<div class="__audio">
				{#if audio.cover}
					<div class="cover ns" style="--clr: {audio.color ?? 'rgb(255, 255, 255)'};">
						<img src={convertFileSrc(audio.cover)} alt="" />
						<button
							onclick={() => {
								play(audio);
							}}
						>
							<Play fill={'var(--bg)'} color={'var(--bg)'} />
						</button>
					</div>
				{/if}
				<p class="title ns">{audio.title ?? 'Titre inconnu'}</p>
				<p class="artist ns">{audio.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</div>
		{/each}
	</div>
{:else}
	Aucune musique trouvée ou indexation non terminé
{/if}

<style>
	.__page_title {
		font-weight: 800;
	}

	.__page_subtitle {
		opacity: 0.5;
		padding-top: 0.3em;
	}

	.__medias {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(20em, 1fr));
		gap: 2em;
		padding-top: 2em;
		padding-inline: 2em;
	}

	.__audio {
		width: 20em;
	}

	.__audio .cover {
		width: 100%;
		position: relative;
	}

	.__audio .cover:hover button {
		opacity: 1;
	}

	.__audio .cover button {
		position: absolute;
		bottom: 50%;
		left: 50%;
		transform: translate(-50%, 50%);
		background-color: var(--fg);
		border: none;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2em;
		border-radius: 50%;
		opacity: 0;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
	}

	.__audio .cover button:active {
		transform: translate(-50%, 50%) scale(0.9);
		opacity: 0.5;
	}

	/* .__audio .cover .__img_loader { */
	/* 	width: 100%; */
	/* 	height: 100%; */
	/* 	aspect-ratio: 1/1; */
	/* 	background-color: var(--clr); */
	/* } */

	.__audio .cover img {
		width: 100%;
		height: 100%;
		border-radius: 10px;
	}

	.__audio .artist {
		opacity: 0.5;
	}
</style>
