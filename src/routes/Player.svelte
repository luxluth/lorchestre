<script lang="ts">
	import Slider from '$lib/components/Slider.svelte';
	import LrcManager from '$lib/lrc.svelte';
	import type Manager from '$lib/manager.svelte';
	import { type Track, type Line } from '$lib/type';

	import X from 'lucide-svelte/icons/x';
	import Play from 'lucide-svelte/icons/play';
	import Pause from 'lucide-svelte/icons/pause';
	import FastForward from 'lucide-svelte/icons/fast-forward';
	import Rewind from 'lucide-svelte/icons/rewind';
	import Volume from 'lucide-svelte/icons/volume';
	import Volume1 from 'lucide-svelte/icons/volume-1';
	import Volume2 from 'lucide-svelte/icons/volume-2';

	import { Howl, Howler } from 'howler';
	import { getContext } from 'svelte';
	import { getAudioUri, getCoverUri } from '$lib/utils';
	import { convertFileSrc } from '@tauri-apps/api/core';

	let manager = getContext<Manager>('manager');
	let lrcMngr = getContext<LrcManager>('lm');

	//@ts-ignore
	let lyricsParent: HTMLElement = $state<HTMLElement>();

	let sound = $state<Howl>();
	let dotScale = $state('scale(1)');

	$effect(() => {
		//@ts-ignore
		window.__player_audio = sound;
	});

	let active = $state<boolean>(false);
	let playing = $state<boolean>(false);
	// let srcUrl = $state<string>('');
	let percentage = $derived((manager.currentTime * 100) / manager.duration);
	let frameHandle: number = $state(0);

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

	function roundDecimal(num: number | string): number {
		if (typeof num === 'string') {
			return Math.round(Number(num) * 1000) / 1000;
		} else {
			return Math.round(num * 1000) / 1000;
		}
	}

	async function getSrc(path: string) {
		let blob = await (await fetch(convertFileSrc(path))).blob();
		console.debug('[getSrc]', blob.type);
		return URL.createObjectURL(blob);
	}

	function tick() {
		if (sound) {
			manager.currentTime = roundDecimal(sound.seek() / sound.duration());
		}
		// if (analyser && canvas && canvasCtx) {
		// let buffer = analyser.frequencyBinCount;
		// let data = new Uint8Array(buffer);
		// let width = canvas.width;
		// let height = canvas.height;
		// analyser.getByteFrequencyData(data);
		// let barWidth = (width / buffer) * 2;
		// let barHeight;
		// let grd = canvasCtx.createLinearGradient(0, height, 0, height / 2);
		// grd.addColorStop(0, 'rgba(0,0,200,0.2)');
		// grd.addColorStop(1, 'rgba(255,0,0,0.2)');
		//
		// if (playing || song.playing()) {
		// 	canvasCtx.clearRect(0, 0, width, height);
		// 	let x = 0;
		//
		// 	for (let i = 0; i < buffer; i++) {
		// 		barHeight = data[i];
		// 		canvasCtx.fillStyle = grd;
		// 		canvasCtx.fillRect(x, height, barWidth, -(barHeight / 2));
		// 		x += barWidth + 1;
		// 	}
		// } else {
		// }
		// requestAnimationFrame(tick);
		// }
		frameHandle = requestAnimationFrame(tick);
	}

	manager.onplayat = (time: number) => {
		if (sound) {
			sound.seek(time);
		}
	};
	manager.ontooglepp = async () => {
		await toggleMediaPlayState();
	};
	manager.onvolumechange = (vol: number) => {
		if (sound) {
			sound.volume(vol);
		}
	};

	manager.ontimeupdate = (time: number) => {
		if (sound) {
			sound.seek(time);
		}
	};

	manager.onPlayerActivate = () => {
		active = true;
	};

	manager.onPlayerDeactivate = () => {
		active = false;
	};

	manager.onplay = async (track: Track) => {
		manager.currentTrack = track;
		// await getSrc(track.file_path);
		// srcUrl = getAudioUri(track.id);
		lrcMngr.reset(track.duration, track.lyrics);

		if (track.lyrics.length > 0) {
			if (lyricsParent) {
				lyricsParent.scrollTop = 0;
			}
		}

		sound = new Howl({
			xhr: {
				method: 'GET',
				headers: {
					'Access-Control-Allow-Origin': '*',
					'Content-Type': track.mime
				}
			},
			html5: true,
			format: track.mime.split('/')[1],
			src: [getAudioUri(track.id), await getSrc(track.file_path)],
			loop: false,
			onload: () => {
				// loaded = true;
				// Audio Context
				// ctx = Howler.ctx;
				// analyser = ctx.createAnalyser();
				// analyser.fftSize = 128;
				// Howler.masterGain.connect(analyser);
			},
			onloaderror: (e) => {
				console.error('[howler::loadError]', e);
				// loadError = true;
			},
			onend: () => {
				playing = false;
				manager.currentTime = 0;
				(async () => {
					await manager.next();
				})();
				cancelAnimationFrame(frameHandle);
				// canvasCtx?.clearRect(0, 0, canvas.width, canvas.height);
				// if (loop) song.play();
			},
			onpause: () => {
				playing = false;
				manager.paused = true;
				cancelAnimationFrame(frameHandle);
			},
			onplay: () => {
				manager.paused = false;
				frameHandle = requestAnimationFrame(tick);
			}
		});

		sound.play();
	};

	// async function getSrc(path: string) {
	// 	let blob = await (await fetch(convertFileSrc(path))).blob();
	// 	srcUrl = URL.createObjectURL(blob);
	// }

	async function toggleMediaPlayState() {
		if (sound) {
			if (manager.paused) {
				sound.play();
			} else {
				sound.pause();
			}
		}
	}

	function playAt(time: number) {
		if (sound) {
			sound.seek(time);
		}

		setTimeout(() => {
			const activeLines = lrcMngr.activeLines;
			if (activeLines.length > 0) {
				let child = lyricsParent.children[activeLines[0].id];
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}, 70);
	}

	let layers = [0, 0, 0];
	function getRandomAngle() {
		return Math.floor(Math.random() * 360);
	}

	let randdeg = $state(getRandomAngle());

	let styles = $derived({
		clr: manager.currentTrack?.color
			? `rgb(${manager.currentTrack?.color.r}, ${manager.currentTrack?.color.g}, ${manager.currentTrack?.color.b})`
			: 'var(--bg)',
		text: '#ffffff',
		r: manager.currentTrack?.color?.r,
		g: manager.currentTrack?.color?.g,
		b: manager.currentTrack?.color?.b,
		percent: `${percentage}%`,
		rd: '255',
		gd: '255',
		bd: '255',
		'random-degree': `${randdeg}deg`,
		brightness: manager.currentTrack?.is_light ? '70%' : '1'
	});

	let styleString = $derived(
		Object.entries(styles)
			.map(([key, value]) => `--${key}: ${value}`)
			.join(';')
	);
</script>

<div class:active class:playing class="__player" style={styleString}>
	<div class="background-images">
		{#each layers as _, index}
			<div
				class:front={index === 0}
				class:back={index === 1}
				class:back_center={index === 2}
				style="
          background-image: url({getCoverUri(manager.currentTrack?.album_id as string, manager.currentTrack?.cover_ext as string)});"
			></div>
		{/each}
	</div>
	<section class="player">
		<div
			class="cover"
			style="background-image: url({getCoverUri(
				manager.currentTrack?.album_id as string,
				manager.currentTrack?.cover_ext as string
			)});"
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
						style="classic"
						color={'var(--text)'}
						thumbColor={'var(--text)'}
						backgroundColor="rgba(var(--rd), var(--gd), var(--bd), 0.2);"
						oninput={(data) => {
							sound?.volume(data);
						}}
					/>
				</div>
				<!-- <div class="bitrate ns">{manager.currentTrack.bitrate}Kb/s</div> -->
			</div>
		</div>
		<div class="infos">
			<h2 class="ns">{manager.currentTrack?.title ?? 'Titre inconnu'}</h2>
			<p class="artist ns">{manager.currentTrack?.artists.join(', ') ?? 'Artiste inconnu'}</p>
		</div>
		<div class="controls">
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
				<!-- <div class="time current ns"> -->
				<!-- 	<span> -->
				<!-- 		{formatTime(manager.currentTime)} -->
				<!-- 	</span> -->
				<!-- </div> -->
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
				<!-- <div class="time ns"> -->
				<!-- 	<span> -->
				<!-- 		{formatTime(manager.duration)} -->
				<!-- 	</span> -->
				<!-- </div> -->
			</div>
		</div>
	</section>
	{#if manager.currentTrack}
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
						class:instrumental={text == '♪'}
					>
						{#if text == '♪'}
							<div class="dot" style="transform: {dotScale};"></div>
						{:else}
							{text}
						{/if}
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

	/* .__player .controls .progress-area div.time { */
	/* 	position: relative; */
	/* 	opacity: 0.5; */
	/* } */
	/**/
	/* .__player .controls .progress-area div.time span { */
	/* 	position: absolute; */
	/* 	bottom: 50%; */
	/* 	left: 50%; */
	/* 	transform: translate(-50%, 50%); */
	/* } */

	.__player .controls .progressbar {
		flex-grow: 1;
		width: 100%;
		padding-inline: 1em;
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
		background-size: cover;
		/* background: var(--clr); */
		color: var(--text);
		width: 100vw;
		height: 100vh;
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

	.__player .background-images {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		z-index: -2;
		overflow: hidden;
	}

	div.front {
		right: 0;
		top: 0;
		z-index: 2;
	}

	div.back_center {
		animation-direction: reverse;
		right: -50%;
		top: -20%;
		width: 300%;
		height: 300%;
		z-index: 0;
	}

	div.back {
		animation-direction: reverse;
		bottom: 0;
		left: 0;
		z-index: 1;
	}

	.__player .background-images > div {
		animation: rot 35s linear infinite;
		border-radius: 100em;
		filter: blur(64px) brightness(var(--brightness));
		position: absolute;
		width: 200%;
		height: 200%;
		object-fit: cover;
		background-size: cover;
	}

	@keyframes rot {
		0% {
			transform: rotate(var(--random-degree));
		}
		100% {
			transform: rotate(calc(var(--random-degree) + 1turn));
		}
	}

	.__player::before {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		background-size: cover;
		background-color: rgba(var(--r), var(--g), var(--b), 0.5);
		backdrop-filter: blur(14px);
		-webkit-backdrop-filter: blur(14px);
		z-index: -1;
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
		font-weight: 600;
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
		background: rgba(255, 255, 255, 0.2);
	}

	.__player .lrc .line.active:hover {
		opacity: 1;
		background: rgba(255, 255, 255, 0.2);
	}

	.line.instrumental {
		padding: 0;
		scale: 0;
		line-height: 0;
		display: flex;
		gap: 0.2em;
		height: 0;
		transform-origin: top left;
		transition: all 0.2s cubic-bezier(0.455, 0.03, 0.515, 0.955);
	}

	.instrumental .dot {
		background-color: var(--text);
		opacity: 1;
		height: 0.5em;
		width: 0.5em;
		border-radius: 50%;
	}

	.line.active.instrumental {
		scale: 1;
		height: auto;
	}

	.__player .cover {
		width: 60vh;
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
		background: rgba(0, 0, 0, 0.3);
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
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 1em;
	}

	/* .bitrate { */
	/* 	position: absolute; */
	/* 	bottom: 1em; */
	/* 	right: 1em; */
	/* 	color: rgb(var(--r), var(--g), var(--b)); */
	/* 	background: var(--text); */
	/* 	width: fit-content; */
	/* 	padding: 0.2em; */
	/* 	font-weight: bold; */
	/* 	border-radius: 4px; */
	/* } */

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
