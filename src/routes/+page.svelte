<script lang="ts">
	import { convertFileSrc, invoke } from '@tauri-apps/api/core';
	import { player } from '$lib/events';
	import { Play } from 'lucide-svelte';
	import { type AudioMedia, type Media } from '$lib/type';
	import { browser } from '$app/environment';

	let media: Media | undefined = $state();

	if (browser) {
		(async () => {
			media = await invoke<Media>('index');
		})();
	}

	function play(audio: AudioMedia) {
		player.play(audio);
	}
</script>

<h1 class="__page_title ns">Bibliothèque</h1>

{#if media}
	<div class="__medias">
		{#each media.audios as audio}
			<div class="__audio">
				{#if audio.cover}
					<div
						class="cover ns"
						style="--clr: {audio.color
							? `rgb(${audio.color.r}, ${audio.color.g}, ${audio.color.b})`
							: 'rgb(255, 255, 255)'}; background-image: url('{convertFileSrc(audio.cover)}');"
					>
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
{/if}

<style>
	.__page_title {
		font-weight: 800;
		font-size: 4em;
		padding-bottom: 0.5em;
	}

	.__medias {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(20em, 1fr));
		gap: 2em;
		padding-top: 2em;
		/* padding-inline: 2em; */
		padding-bottom: 15em;
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

	.__audio .cover {
		aspect-ratio: 1/1;
		background-color: var(--clr);
		border-radius: 10px;
		margin-bottom: 0.3em;
		background-size: cover;
	}

	.__audio .artist {
		opacity: 0.5;
	}
</style>
