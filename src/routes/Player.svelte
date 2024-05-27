<script lang="ts">
	import { player } from '$lib/events';
	import type { AudioMedia } from '$lib/type';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { X } from 'lucide-svelte';

	let media = $state<AudioMedia>();
	let active = $state<boolean>(false);
	let playing = $state<boolean>(false);
	let srcUrl = $state<string>('');
	//@ts-ignore
	let sound: HTMLAudioElement = $state<HTMLAudioElement>();

	async function play(e: CustomEvent<AudioMedia>) {
		media = e.detail;
		active = true;
		await getSrc(media.file_path);

		if (sound) {
			sound.pause();
		}
		sound.src = srcUrl;
		sound.currentTime = 0;
		await sound.play();
	}

	async function getSrc(path: string) {
		let blob = await (await fetch(convertFileSrc(path))).blob();
		srcUrl = URL.createObjectURL(blob);
	}

	$effect(() => {
		//@ts-ignore
		document.addEventListener(player.PLAY_EV, play);

		return () => {
			//@ts-ignore
			document.removeEventListener(player.PLAY_EV, play);
		};
	});
</script>

<div class="__mini_player" class:active={!active}>lul</div>

<div
	class:active
	class:playing
	class="__player"
	style="--clr: {media?.color ?? 'var(--bg)'}; --text: {media?.is_light ? '#181818' : '#ffffff'};"
>
	<button
		class="close"
		onclick={() => {
			active = false;
		}}
	>
		<X size={'3em'} />
	</button>
	{#if media}
		<section class="player">
			<div class="cover">
				{#if media.cover}
					<div class="cover">
						<img class="ns" src={convertFileSrc(media.cover)} alt="" />
					</div>
				{/if}
			</div>
			<div class="infos">
				<h2 class="ns">{media.title ?? 'Titre inconnu'}</h2>
				<p class="artist ns">{media.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</div>
			<audio controls crossorigin="anonymous" bind:this={sound}></audio>
		</section>
		{#if media.lyrics.length > 0}
			<section class="lrc">
				{#each media.lyrics as line}
					<div class="line">{line.text}</div>
				{/each}
			</section>
		{/if}
	{/if}
</div>

<style>
	.__mini_player {
		width: 100%;
		left: 0;
		top: 0;
		position: fixed;
	}

	.close {
		color: var(--text);
		position: absolute;
		top: 2em;
		left: 2em;
		background: none;
		border: none;
		opacity: 0.5;
		cursor: pointer;
		transition: all 0.1s ease-in-out;
	}

	.close:hover {
		opacity: 1;
	}

	.close:active {
		transform: scale(0.95);
	}

	.__player {
		background: var(--clr);
		color: var(--text);
		width: 100%;
		height: 100%;
		position: fixed;
		top: 0;
		left: 0;
		display: flex;
		justify-content: space-evenly;
		align-items: center;
		padding-block: 3em;
		padding-inline: 2em;
		gap: 2em;
		transform: translateY(200%);
	}

	.__player .player {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 2em;
	}

	.__player .lrc {
		height: 100%;
		overflow-y: auto;
		width: 50em;
	}

	.__player .lrc .line:first-child {
		padding-top: 50%;
	}

	.__player .lrc .line:last-child {
		padding-bottom: 50%;
	}

	.__player .lrc .line {
		font-size: 3em;
		padding-block: 0.25em;
		font-weight: 800;
		opacity: 0.5;
		transition: all 0.1s ease-in-out;
		cursor: pointer;
		line-height: 1;
	}

	.__player .lrc .line:hover {
		opacity: 1;
	}

	.__player .cover {
		width: 40em;
		height: 40em;
		overflow: hidden;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.__player .cover img {
		border-radius: 10px;
		width: 100%;
		height: 100%;
	}

	.__player.active {
		transform: translateY(0%);
	}
</style>
