<script lang="ts">
	import { player } from '$lib/events';
	import LrcManager from '$lib/lrc.svelte';
	import type { AudioMedia, Line } from '$lib/type';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { X, Play, Pause, FastForward, Rewind } from 'lucide-svelte';

	//@ts-ignore
	let lyricsParent: HTMLElement = $state<HTMLElement>();

	//@ts-ignore
	let sound: HTMLAudioElement = $state<HTMLAudioElement>();

	let media = $state<AudioMedia>();
	let active = $state<boolean>(false);
	let playing = $state<boolean>(false);
	let srcUrl = $state<string>('');
	let duration = $state(0);
	let currentTime = $state(0);
	let percentage = $derived(((currentTime * 100) / duration).toFixed(3));
	let paused = $state<boolean>(true);
	let lrcMngr = new LrcManager(duration, []);
	let activeLines: Line[] = $state([]);

	$effect(() => {
		lrcMngr.update(currentTime);
	});

	lrcMngr.oncuechange = () => {
		activeLines = lrcMngr.activeLines;
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

	async function play(e: CustomEvent<AudioMedia>) {
		media = e.detail;
		active = true;
		await getSrc(media.file_path);
		duration = media.duration;

		lrcMngr.reset(media.duration, media.lyrics);
		activeLines = [];

		if (media.lyrics.length > 0) {
			lyricsParent.scrollTop = 0;
		}

		if (sound) {
			sound.pause();
		}
		sound.src = srcUrl;
		sound.volume = 0.1;
		sound.currentTime = 0;
		sound.onpause = () => {
			paused = true;
		};

		sound.onplay = () => {
			paused = false;
		};
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

	async function toggleMediaPlayState() {
		if (sound) {
			if (sound.paused) {
				await sound.play();
			} else {
				sound.pause();
			}
		}
	}

	function isActiveLine(id: number) {
		for (let i = 0; i < activeLines.length; i++) {
			const element = activeLines[i];
			if (element.id == id) {
				return true;
			}
		}
		return false;
	}

	function playAt(time: number) {
		if (sound) {
			sound.currentTime = time;
		}
	}
</script>

<div
	class:active
	class:playing
	class="__player"
	style="--clr: {media?.color
		? `rgb(${media?.color.r}, ${media?.color.g}, ${media?.color.b})`
		: 'var(--bg)'}; --text: {media?.is_light ? '#181818' : '#ffffff'}; --r: {media?.color
		?.r}; --g: {media?.color?.g}; --b: {media?.color
		?.b}; --percent: {percentage}%; --rd: {media?.is_light ? '24' : '255'}; --gd: {media?.is_light
		? '24'
		: '255'}; --bd: {media?.is_light ? '24' : '255'};"
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
			{#if media.cover}
				<div class="cover" style="background-image: url({convertFileSrc(media.cover)});"></div>
			{/if}
			<div class="infos">
				<h2 class="ns">{media.title ?? 'Titre inconnu'}</h2>
				<p class="artist ns">{media.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</div>
			<div class="controls" style="--percent: {percentage}%;">
				<div class="actions">
					<button>
						<Rewind fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
					</button>
					<button class="playpause" onclick={toggleMediaPlayState}>
						{#if paused}
							<Play fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
						{:else}
							<Pause fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
						{/if}
					</button>
					<button>
						<FastForward fill={'var(--text)'} color={'var(--text)'} size={'2.5em'} />
					</button>
				</div>
				<div class="progress-area">
					<div class="time current ns">
						<span>
							{formatTime(currentTime)}
						</span>
					</div>
					<div class="progressbar">
						<div class="progress">
							<div class="shadow"></div>
						</div>
					</div>
					<div class="time ns">
						<span>
							{formatTime(duration)}
						</span>
					</div>
				</div>
			</div>
			<audio crossorigin="anonymous" bind:paused bind:this={sound} bind:currentTime></audio>
		</section>
		{#if media.lyrics.length > 0}
			<section class="lrc" bind:this={lyricsParent}>
				{#each media.lyrics as { text, start_time }, i}
					<div
						class="line ns"
						data-line={i}
						data-time={start_time}
						class:active={isActiveLine(i)}
						onclick={() => playAt(start_time / 1000)}
						onkeydown={() => {}}
						role="button"
					>
						{text}
					</div>
				{/each}
			</section>
		{/if}
	{/if}
</div>

<div
	class="__mini_player"
	class:__is_light={media?.is_light}
	class:active={!active && typeof media === 'object'}
	style="--clr: {media?.color
		? `rgb(${media?.color.r}, ${media?.color.g}, ${media?.color.b})`
		: 'var(--bg)'}; --text: {media?.is_light ? '#181818' : '#ffffff'}; --r: {media?.color
		?.r}; --g: {media?.color?.g}; --b: {media?.color?.b}; --percent: {percentage}%;"
>
	{#if media}
		<div class="infos">
			{#if media.cover}
				<div
					class="cover"
					onclick={() => {
						active = true;
					}}
					onkeydown={(e) => {
						if (e.key.toLowerCase() === 'enter') {
							active = true;
						}
					}}
					role="button"
					tabindex="0"
					style="background-image: url('{convertFileSrc(media.cover)}');"
				></div>
			{/if}
			<div class="a-n">
				<p class="title ns">{media.title ?? 'Titre inconnu'}</p>
				<p class="artist ns">{media.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</div>
		</div>
		<div class="controls">
			<div class="actions">
				<button>
					<Rewind fill={'var(--fg)'} color={'var(--fg)'} size={'2.5em'} />
				</button>
				<button class="playpause" onclick={toggleMediaPlayState}>
					{#if paused}
						<Play fill={'var(--fg)'} color={'var(--fg)'} size={'2.5em'} />
					{:else}
						<Pause fill={'var(--fg)'} color={'var(--fg)'} size={'2.5em'} />
					{/if}
				</button>
				<button>
					<FastForward fill={'var(--fg)'} color={'var(--fg)'} size={'2.5em'} />
				</button>
			</div>
			<div class="progress-area">
				<div class="time current ns">{formatTime(currentTime)}</div>
				<div class="progressbar">
					<div class="progress">
						<div class="shadow"></div>
					</div>
				</div>
				<div class="time ns">{formatTime(duration)}</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.__mini_player {
		width: 90%;
		height: 5em;
		left: 5em;
		right: 5em;
		bottom: 1em;
		padding-block: 0.6em;
		padding-inline: 0.6em;
		color: var(--fg);
		position: fixed;
		transform: translateY(200%);
		transition: transform 0.5s ease-in-out;
		background: rgba(24, 24, 24, 0.9);
		/* background: rgba(var(--r), var(--g), var(--b), 0.9); */
		/* box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37); */
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.18);
	}

	.__mini_player {
		display: flex;
		gap: 1em;
	}

	.__mini_player .infos {
		width: 20em;
		overflow: hidden;
	}

	.__mini_player .infos p {
		word-break: keep-all;
		white-space: pre;
	}

	.__mini_player .controls {
		flex-grow: 1;
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 0.5em;
		width: 100%;
		height: 100%;
	}

	.__player .controls {
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 1em;
		width: 100%;
	}

	.__mini_player .controls .progress-area,
	.__player .controls .progress-area {
		width: 100%;
		height: 100%;
		display: flex;
		gap: 1em;
		justify-content: space-evenly;
		align-items: center;
	}

	.__mini_player .controls .progress-area div.time {
		height: 1em;
		width: 100%;
		display: flex;
		align-items: center;
		opacity: 0.5;
		font-size: 0.9em;
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

	.__mini_player .controls .progress-area div.time.current {
		justify-content: flex-end;
	}

	.__mini_player .controls .progressbar,
	.__player .controls .progressbar {
		flex-grow: 1;
		width: 100%;
		height: 0.5em;
		border-radius: 5px;
		background-color: #333;
		position: relative;
	}

	.__player .controls .progressbar {
		background-color: rgba(var(--rd), var(--gd), var(--bd), 0.2);
	}

	.__player .controls .progressbar {
		margin-inline: 1em;
	}

	.__mini_player .controls .progressbar .progress,
	.__player .controls .progressbar .progress {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		overflow: hidden;
		border-radius: 5px;
	}

	.__mini_player .controls .progressbar .progress .shadow,
	.__player .controls .progressbar .shadow {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: var(--percent);
		background-color: var(--fg);
	}

	.__player .controls .progressbar .shadow {
		background-color: var(--text);
	}

	.__mini_player .controls .actions,
	.__player .controls .actions {
		display: flex;
		gap: 1em;
	}

	.__mini_player .controls .actions button,
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

	.__mini_player .controls .actions button:hover,
	.__player .controls .actions button:hover {
		opacity: 1;
	}

	.__mini_player .controls .actions button:active,
	.__player .controls .actions button:active {
		opacity: 1;
		transform: scale(0.98);
	}

	.__mini_player.active {
		transform: translateY(0%);
	}

	.__mini_player .cover {
		height: 100%;
		aspect-ratio: 1/1;
		cursor: pointer;
		border-radius: 8px;
		background-size: cover;
	}

	.__mini_player .infos {
		display: flex;
		height: 100%;
		align-items: center;
		gap: 0.6em;
	}

	.__mini_player .infos .a-n .title {
		font-weight: 800;
	}

	.__mini_player .infos .a-n .artist {
		opacity: 0.5;
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
		transition: transform 0.3s ease-in-out;
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
		background-position: cover;
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
