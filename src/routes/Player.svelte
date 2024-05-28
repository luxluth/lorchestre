<script lang="ts">
	import { player } from '$lib/events';
	import LrcManager from '$lib/lrc.svelte';
	import { PlayerDispatchKind, type Track, type Line } from '$lib/type';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { X, Play, Pause, FastForward, Rewind } from 'lucide-svelte';

	//@ts-ignore
	let lyricsParent: HTMLElement = $state<HTMLElement>();
	//@ts-ignore
	let sound: HTMLAudioElement = $state<HTMLAudioElement>();

	let track = $state<Track>();
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
		console.log('time update');
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

	async function play(e: CustomEvent<Track>) {
		player.dispatch({ kind: PlayerDispatchKind.NewMedia, data: e.detail });

		track = e.detail;
		active = true;
		await getSrc(track.file_path);
		duration = track.duration;
		lrcMngr.reset(track.duration, track.lyrics);
		activeLines = [];

		if (track.lyrics.length > 0) {
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
			player.dispatch({ kind: PlayerDispatchKind.PlayPause, data: 'paused' });
		};

		sound.onplay = () => {
			paused = false;
			player.dispatch({ kind: PlayerDispatchKind.PlayPause, data: 'play' });
		};

		sound.ontimeupdate = () => {
			player.dispatch({ kind: PlayerDispatchKind.TimeUpdate, data: currentTime });
		};

		await sound.play();
	}

	async function getSrc(path: string) {
		let blob = await (await fetch(convertFileSrc(path))).blob();
		srcUrl = URL.createObjectURL(blob);
	}

	function activate() {
		active = true;
	}

	$effect(() => {
		console.log('player mounted');
		//@ts-ignore
		document.addEventListener(player.PLAY_EV, play);
		//@ts-ignore
		document.addEventListener(player.PLAY_AT, playAtEv);
		document.addEventListener(player.PLAYER_ACTIVATE, activate);

		return () => {
			//@ts-ignore
			document.removeEventListener(player.PLAY_EV, play);
			//@ts-ignore
			document.removeEventListener(player.PLAY_AT, playAtEv);
			document.removeEventListener(player.PLAYER_ACTIVATE, activate);
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

	function playAtEv(e: CustomEvent<number>) {
		playAt(e.detail);
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
	style="--clr: {track?.color
		? `rgb(${track?.color.r}, ${track?.color.g}, ${track?.color.b})`
		: 'var(--bg)'}; --text: {track?.is_light ? '#181818' : '#ffffff'}; --r: {track?.color
		?.r}; --g: {track?.color?.g}; --b: {track?.color
		?.b}; --percent: {percentage}%; --rd: {track?.is_light ? '24' : '255'}; --gd: {track?.is_light
		? '24'
		: '255'}; --bd: {track?.is_light ? '24' : '255'};"
>
	<button
		class="close"
		onclick={() => {
			active = false;
		}}
	>
		<X size={'3em'} />
	</button>
	{#if track}
		<section class="player">
			{#if track.cover}
				<div class="cover" style="background-image: url({convertFileSrc(track.cover)});"></div>
			{/if}
			<div class="infos">
				<h2 class="ns">{track.title ?? 'Titre inconnu'}</h2>
				<p class="artist ns">{track.artists.join(', ') ?? 'Artiste inconnu'}</p>
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
		{#if track.lyrics.length > 0}
			<section class="lrc" bind:this={lyricsParent}>
				{#each track.lyrics as { text, start_time }, i}
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

	.__player .controls .progressbar .progress {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		overflow: hidden;
		border-radius: 5px;
	}

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
