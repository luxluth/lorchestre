<script lang="ts">
	import Slider from '$lib/components/Slider.svelte';
	import LrcManager from '$lib/lrc.svelte';
	import type Manager from '$lib/manager.svelte';
	import { type Track, type Line } from '$lib/type';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { X, Play, Pause, FastForward, Rewind, Volume, Volume1, Volume2 } from 'lucide-svelte';

	import { getContext } from 'svelte';

	let manager = getContext<Manager>('manager');
	let lrcMngr = getContext<LrcManager>('lm');

	//@ts-ignore
	let lyricsParent: HTMLElement = $state<HTMLElement>();
	//@ts-ignore
	let sound: HTMLAudioElement = $state<HTMLAudioElement>();

	let active = $state<boolean>(false);
	let playing = $state<boolean>(false);
	let srcUrl = $state<string>('');
	let percentage = $derived((manager.currentTime * 100) / manager.duration);

	$effect(() => {
		lrcMngr.update(manager.currentTime);
	});

	lrcMngr.oncuechange = () => {
		const activeLines = lrcMngr.activeLines;
		if (activeLines.length > 0) {
			let child = lyricsParent.children[activeLines[0].id];
			if (isElementVisible(child as HTMLElement) || !active) {
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	};

	function isElementVisible(element: HTMLElement) {
		if (!element) {
			return false;
		}
		const rect = element.getBoundingClientRect();
		const style = window.getComputedStyle(element);

		const isInDOM = element.offsetParent !== null;
		const hasSize = rect.width > 0 && rect.height > 0;
		const isDisplayed = style.display !== 'none';
		const isVisible = style.visibility !== 'hidden' && style.opacity !== '0';
		const isInViewport =
			rect.top >= 0 &&
			rect.left >= 0 &&
			rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
			rect.right <= (window.innerWidth || document.documentElement.clientWidth);

		return isInDOM && hasSize && isDisplayed && isVisible && isInViewport;
	}

	function formatTime(time: number) {
		if (isNaN(time)) {
			return '--:--';
		}
		if (time >= 60 * 60) {
			return new Date(time * 1000).toISOString().substring(11, 16);
		} else {
			return new Date(time * 1000).toISOString().substring(14, 19);
		}
	}

	manager.onplayat = (time: number) => {
		sound.currentTime = time;
	};
	manager.ontooglepp = async () => {
		await toggleMediaPlayState();
	};
	manager.onvolumechange = (vol: number) => {
		sound.volume = vol;
	};

	manager.ontimeupdate = (time: number) => {
		sound.currentTime = time;
	};

	manager.onPlayerActivate = () => {
		active = true;
	};

	manager.onPlayerDeactivate = () => {
		active = false;
	};

	manager.onplay = async (track: Track) => {
		manager.currentTrack = track;
		await getSrc(track.file_path);
		lrcMngr.reset(track.duration, track.lyrics);

		if (track.lyrics.length > 0) {
			lyricsParent.scrollTop = 0;
		}

		if (sound) {
			sound.pause();
		}
		sound.src = srcUrl;
		sound.currentTime = 0;
		sound.onpause = () => {
			manager.paused = true;
		};

		sound.onplay = () => {
			manager.paused = false;
		};

		sound.ontimeupdate = () => {
			if (sound.currentTime) {
				manager.currentTime = sound.currentTime;
			}
		};

		sound.onvolumechange = () => {
			manager.volume = sound.volume;
		};

		sound.onended = async () => {
			await manager.next();
		};

		await sound.play();
	};

	async function getSrc(path: string) {
		let blob = await (await fetch(convertFileSrc(path))).blob();
		srcUrl = URL.createObjectURL(blob);
	}

	async function toggleMediaPlayState() {
		if (sound) {
			if (sound.paused) {
				await sound.play();
			} else {
				sound.pause();
			}
		}
	}

	function playAt(time: number) {
		if (sound) {
			sound.currentTime = time;
		}

		setTimeout(() => {
			const activeLines = lrcMngr.activeLines;
			if (activeLines.length > 0) {
				let child = lyricsParent.children[activeLines[0].id];
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}, 30);
	}
</script>

<div
	class:active
	class:playing
	class="__player"
	style="--clr: {manager.currentTrack?.color
		? `rgb(${manager.currentTrack?.color.r}, ${manager.currentTrack?.color.g}, ${manager.currentTrack?.color.b})`
		: 'var(--bg)'}; --text: {manager.currentTrack?.is_light ? '#181818' : '#ffffff'}; --r: {manager
		.currentTrack?.color?.r}; --g: {manager.currentTrack?.color?.g}; --b: {manager.currentTrack
		?.color?.b}; --percent: {percentage}%; --rd: {manager.currentTrack?.is_light
		? '24'
		: '255'}; --gd: {manager.currentTrack?.is_light ? '24' : '255'}; --bd: {manager.currentTrack
		?.is_light
		? '24'
		: '255'};"
>
	{#if manager.currentTrack}
		<section class="player">
			{#if manager.currentTrack.cover}
				<div
					class="cover"
					style="background-image: url({convertFileSrc(manager.currentTrack.cover)});"
				>
					<div class="actions">
						<button
							class="close"
							onclick={() => {
								active = false;
							}}
						>
							<X size={'3em'} />
						</button>
						<div class="volume">
							<div class="vol-icon">
								{#if manager.volume === 0}
									<Volume size={'1.5em'} />
								{:else if manager.volume >= 0.7}
									<Volume2 size={'1.5em'} />
								{:else if manager.volume > 0}
									<Volume1 size={'1.5em'} />
								{/if}
							</div>
							<Slider
								value={manager.volume}
								style="thick"
								color={'var(--text)'}
								thumbColor={'var(--text)'}
								backgroundColor="rgba(var(--rd), var(--gd), var(--bd), 0.2);"
								oninput={(data) => {
									sound.volume = data;
								}}
							/>
						</div>
					</div>
				</div>
			{/if}
			<div class="infos">
				<h2 class="ns">{manager.currentTrack.title ?? 'Titre inconnu'}</h2>
				<p class="artist ns">{manager.currentTrack.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</div>
			<div class="controls" style="--percent: {percentage}%;">
				<div class="actions">
					<button>
						<Rewind
							fill={'var(--text)'}
							color={'var(--text)'}
							size={'2.5em'}
							onclick={async () => {
								await manager.prev();
							}}
						/>
					</button>
					<button class="playpause" onclick={toggleMediaPlayState}>
						{#if manager.paused}
							<Play fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
						{:else}
							<Pause fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
						{/if}
					</button>
					<button>
						<FastForward
							fill={'var(--text)'}
							color={'var(--text)'}
							size={'2.5em'}
							onclick={async () => {
								await manager.next();
							}}
						/>
					</button>
				</div>
				<div class="progress-area">
					<div class="time current ns">
						<span>
							{formatTime(manager.currentTime)}
						</span>
					</div>
					<div class="progressbar">
						<Slider
							value={percentage / 100}
							color={'var(--text)'}
							thumbColor={'var(--text)'}
							style="thick"
							backgroundColor="rgba(var(--rd), var(--gd), var(--bd), 0.2);"
							oninput={(data) => {
								playAt(data * manager.duration);
							}}
						/>
					</div>
					<div class="time ns">
						<span>
							{formatTime(manager.duration)}
						</span>
					</div>
				</div>
			</div>
			<audio crossorigin="anonymous" bind:this={sound}></audio>
		</section>
		{#if manager.currentTrack.lyrics.length > 0}
			<section class="lrc" bind:this={lyricsParent}>
				{#each lrcMngr.lines as { text, startTime, id }}
					<div
						class="line ns"
						data-time={startTime}
						class:active={lrcMngr.activeLines.find((i) => i.id === id)}
						onclick={() => playAt(startTime)}
						onkeydown={() => {}}
						role="button"
						tabindex="0"
					>
						{text}
					</div>
				{/each}
			</section>
		{/if}
	{/if}
</div>

<style>
	.__player .controls {
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 1em;
		width: 100%;
	}

	.__player .controls .progress-area {
		width: 100%;
		height: 100%;
		display: flex;
		gap: 1em;
		justify-content: space-evenly;
		align-items: center;
	}

	.__player .controls .progress-area div.time {
		position: relative;
		opacity: 0.5;
	}

	.__player .controls .progress-area div.time span {
		position: absolute;
		bottom: 50%;
		left: 50%;
		transform: translate(-50%, 50%);
	}

	.__player .controls .progressbar {
		flex-grow: 1;
		width: 100%;
		padding-inline: 2em;
	}

	.__player .controls .actions {
		display: flex;
		gap: 1em;
	}

	.__player .controls .actions button {
		background: none;
		border: none;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0.5;
		transition: all 0.1s ease-in-out;
		cursor: pointer;
	}

	.__player .controls .actions button:hover {
		opacity: 1;
	}

	.__player .controls .actions button:active {
		opacity: 1;
		transform: scale(0.98);
	}

	.__player .cover .actions .volume {
		position: absolute;
		bottom: 1em;
		width: 100%;
		left: 0;
		padding-inline: 2em;
		display: flex;
		gap: 1em;
		align-items: center;
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
		z-index: 100;
		top: 0;
		left: 0;
		display: flex;
		justify-content: space-evenly;
		align-items: center;
		padding-block: 3em;
		padding-inline: 2em;
		gap: 2em;
		transform: translateY(200%);
		transition: all 0.3s ease-in-out;
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
		max-width: 50em;
		position: relative;
	}

	/* .__player .lrc::before { */
	/* 	content: ''; */
	/* 	position: fixed; */
	/* 	top: 0; */
	/* 	width: 50em; */
	/* 	height: 5em; */
	/* 	background: linear-gradient(var(--clr), rgba(var(--r), var(--g), var(--b), 0)); */
	/* } */
	/**/
	/* .__player .lrc::after { */
	/* 	content: ''; */
	/* 	position: fixed; */
	/* 	bottom: 0; */
	/* 	width: 50em; */
	/* 	height: 5em; */
	/* 	background: linear-gradient(360deg, var(--clr), rgba(var(--r), var(--g), var(--b), 0)); */
	/* } */

	.__player .lrc .line:first-child {
		margin-top: 50%;
	}

	.__player .lrc .line:last-child {
		margin-bottom: 50%;
	}

	.__player .lrc .line {
		font-size: 3em;
		padding: 0.25em;
		font-weight: 800;
		opacity: 0.3;
		transition: all 0.2s ease-in-out;
		cursor: pointer;
		line-height: 1;
		border-radius: 8px;
	}

	.__player .lrc .line.active {
		opacity: 1;
	}

	.__player .lrc .line:active {
		transform: scale(0.98);
	}

	.__player .lrc .line:hover {
		opacity: 0.5;
		background: rgba(var(--rd), var(--gd), var(--bd), 0.2);
	}

	.__player .lrc .line.active:hover {
		opacity: 1;
		background: rgba(var(--rd), var(--gd), var(--bd), 0.2);
	}

	.__player .cover {
		width: 40em;
		aspect-ratio: 1/1;
		box-shadow: rgba(17, 12, 46, 0.15) 0px 48px 100px 0px;
		border-radius: 10px;
		/* box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px; */
		background-size: cover;
		position: relative;
		overflow: hidden;
	}

	.__player .cover .actions {
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		position: absolute;
		background: rgba(var(--r), var(--g), var(--b), 0.7);
		opacity: 0;
		transition: opacity 0.3s ease-in-out;
	}

	.__player .cover:hover .actions {
		opacity: 1;
	}

	.__player .infos {
		padding-top: 1em;
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.__player .infos h2 {
		font-size: 2em;
	}

	.__player .infos p {
		font-size: 1.5em;
		opacity: 0.5;
	}

	.__player.active {
		transform: translateY(0%);
	}
</style>
