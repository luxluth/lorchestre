<script lang="ts">
	import Slider from '$lib/components/Slider.svelte';
	import { getLrc } from '$lib/lrc.svelte';
	import { type QueueTrack } from '$lib/type';

	import X from 'lucide-svelte/icons/x';
	import Play from 'lucide-svelte/icons/play';
	import Pause from 'lucide-svelte/icons/pause';
	import FastForward from 'lucide-svelte/icons/fast-forward';
	import Rewind from 'lucide-svelte/icons/rewind';
	import Volume from 'lucide-svelte/icons/volume';
	import Volume1 from 'lucide-svelte/icons/volume-1';
	import Volume2 from 'lucide-svelte/icons/volume-2';
	import Search from 'lucide-svelte/icons/search';

	import { formatTime, getAudioUri, getCoverUri } from '$lib/utils';
	import Marquee from '$lib/components/Marquee.svelte';
	import { getManager } from '$lib/manager.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	import { _ } from 'svelte-i18n';
	import Line from './Line.svelte';
	import { onDestroy } from 'svelte';

	let manager = getManager();
	let lrcMngr = getLrc();
	let config = getAppConfig();

	let blurActive = $derived(
		config.config.global?.enable_blur ?? config.defaults.global.enable_blur
	);

	let lyricsParent: HTMLElement = $state<HTMLElement>()!;
	let sound: HTMLAudioElement = $state<HTMLAudioElement>()!;
	let hasLyrics = $derived(lrcMngr.lines.length > 0);

	$effect(() => {
		//@ts-ignore
		window.__player_audio = sound;
	});

	let active = $state<boolean>(false);
	let playing = $state<boolean>(false);
	let percentage = $derived((manager.currentTime * 100) / manager.duration);

	const hookRemove = lrcMngr.oncuechange(() => {
		const activeLines = lrcMngr.activeLines;
		if (activeLines.length > 0) {
			let child = lyricsParent.children[activeLines[0].id];
			if (isElementVisible(child as HTMLElement) || !active) {
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	});

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

	manager.onseekto = (time: number) => {
		if (sound) {
			sound.currentTime = time;
		}
	};
	manager.ontogglepp = async () => {
		await toggleMediaPlayState();
	};

	manager.onPlayerActivate = () => {
		active = true;
	};

	manager.onPlayerDeactivate = () => {
		active = false;
	};

	manager.onstop = () => {
		sound.pause();
		sound.src = '';
	};

	const defaultSkipTime = 5;

	const actionHandlers: [MediaSessionAction, MediaSessionActionHandler][] = [
		[
			'play',
			async () => {
				await manager.togglepp();
			}
		],
		[
			'pause',
			async () => {
				await manager.togglepp();
			}
		],
		[
			'previoustrack',
			async () => {
				await manager.prev();
			}
		],
		[
			'nexttrack',
			async () => {
				await manager.next();
			}
		],
		[
			'seekbackward',
			(details) => {
				const skipTime = details.seekOffset || defaultSkipTime;
				manager.currentTime = Math.max(manager.currentTime - skipTime, 0);
				updatePositionState();
			}
		],
		[
			'seekforward',
			(details) => {
				const skipTime = details.seekOffset || defaultSkipTime;
				manager.currentTime = Math.min(sound.currentTime + skipTime, manager.duration);
				updatePositionState();
			}
		],
		[
			'seekto',
			(details) => {
				if (details.fastSeek && 'fastSeek' in sound) {
					sound.fastSeek(details.seekTime as number);
					return;
				}
				manager.currentTime = details.seekTime || 0;
				updatePositionState();
			}
		]
	];

	manager.onplay = async (track: QueueTrack) => {
		await lrcMngr.reset(track.duration, track);
		manager.currentTrack = track;

		sound.src = getAudioUri(track.path_base64, config);

		sound.onended = () => {
			playing = false;
			manager.currentTime = 0;
			(async () => {
				await manager.next();
			})();
		};
		sound.onpause = () => {
			playing = false;
			manager.paused = true;
			navigator.mediaSession.playbackState = 'paused';
		};

		sound.onplay = () => {
			manager.paused = false;
			navigator.mediaSession.playbackState = 'playing';
		};

		sound.ontimeupdate = () => {
			lrcMngr.update(sound?.currentTime ?? 0);
		};

		if (manager.initialized) {
			await sound.play();
		}
		if ('mediaSession' in navigator) {
			navigator.mediaSession.metadata = new MediaMetadata({
				title: track.title,
				artist: track.album_artist ?? track.artists[0],
				album: track.album,
				artwork: [
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 96),
						sizes: '96x96',
						type: 'image/png'
					},
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 128),
						sizes: '128x128',
						type: 'image/png'
					},
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 192),
						sizes: '192x192',
						type: 'image/png'
					},
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 256),
						sizes: '256x256',
						type: 'image/png'
					},
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 384),
						sizes: '384x384',
						type: 'image/png'
					},
					{
						src: getCoverUri(track.album_id, track.cover_ext, config, 512),
						sizes: '512x512',
						type: 'image/png'
					}
				]
			});

			for (const [action, handler] of actionHandlers) {
				try {
					navigator.mediaSession.setActionHandler(action, handler);
				} catch (error) {
					console.log(`The media session action "${action}" is not supported yet.`);
				}
			}
		}
	};

	function updatePositionState() {
		if ('setPositionState' in navigator.mediaSession) {
			navigator.mediaSession.setPositionState({
				duration: manager.duration,
				playbackRate: sound.playbackRate,
				position: manager.currentTime
			});
		}
	}

	const unregister = manager.afterplay(() => {
		if (lyricsParent) {
			setTimeout(() => {
				lyricsParent.scrollTo({ behavior: 'smooth', top: 0 });
			}, 70);
		}
	});

	onDestroy(() => {
		hookRemove();
		unregister();
	});

	function afterSeek() {
		setTimeout(() => {
			const activeLines = lrcMngr.activeLines;
			if (activeLines.length > 0) {
				let child = lyricsParent.children[activeLines[0].id];
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}, 70);
	}

	async function toggleMediaPlayState() {
		if (sound) {
			if (manager.paused) {
				await sound.play();
			} else {
				sound.pause();
			}
		}
	}

	let layers = [0, 0, 0];
	function getRandomAngle() {
		return Math.floor(Math.random() * 360);
	}

	function getColor() {
		if (manager.currentTrack?.color) {
			if (manager.currentTrack?.is_light) {
				return '#181818';
			} else {
				return '#ffffff';
			}
		} else {
			return 'var(--fg)';
		}
	}

	function getColorParts() {
		if (manager.currentTrack?.color) {
			if (manager.currentTrack?.is_light) {
				return '24';
			} else {
				return '255';
			}
		}

		return '255';
	}

	let randdeg = $state(getRandomAngle());

	let styles = $derived({
		clr: manager.currentTrack?.color
			? `rgb(${manager.currentTrack?.color.r}, ${manager.currentTrack?.color.g}, ${manager.currentTrack?.color.b})`
			: 'var(--bg)',
		text: blurActive ? '#ffffff' : `${getColor()}`,
		r: manager.currentTrack?.color?.r,
		g: manager.currentTrack?.color?.g,
		b: manager.currentTrack?.color?.b,
		percent: `${percentage.toFixed(0)}%`,
		rd: blurActive ? '255' : getColorParts(),
		gd: blurActive ? '255' : getColorParts(),
		bd: blurActive ? '255' : getColorParts(),
		'random-degree': `${randdeg}deg`,
		brightness: manager.currentTrack?.is_light ? '70%' : '1'
	});

	let styleString = $derived(
		Object.entries(styles)
			.map(([key, value]) => `--${key}: ${value}`)
			.join(';')
	);
</script>

<div
	class:active
	class:playing
	class:no_lyrics={!hasLyrics}
	class="__player ns"
	style={styleString}
	class:blurActive
>
	<div class="dragzone" data-tauri-drag-region></div>
	<!-- WARN: Can slow the application down  -->
	{#if blurActive}
		<div class="background-images">
			{#each layers as _, index}
				<div class:front={index === 0} class:back={index === 1} class:back_center={index === 2}>
					<img
						src={getCoverUri(
							manager.currentTrack?.album_id as string,
							manager.currentTrack?.cover_ext as string,
							config,
							30
						)}
						alt=""
					/>
				</div>
			{/each}
		</div>
	{/if}
	<section class="player">
		<div
			class="cover"
			style="background-image: url({getCoverUri(
				manager.currentTrack?.album_id as string,
				manager.currentTrack?.cover_ext as string,
				config
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
				<div class="controls">
					<button>
						<Rewind
							fill={'var(--text)'}
							color={'var(--text)'}
							size={'5em'}
							onclick={async () => {
								await manager.prev();
							}}
						/>
					</button>
					<button class="playpause" onclick={toggleMediaPlayState}>
						{#if manager.paused}
							<Play fill={'var(--text)'} color={'var(--text)'} size={'5em'} />
						{:else}
							<Pause fill={'var(--text)'} color={'var(--text)'} size={'5em'} />
						{/if}
					</button>
					<button>
						<FastForward
							fill={'var(--text)'}
							color={'var(--text)'}
							size={'5em'}
							onclick={async () => {
								await manager.next();
							}}
						/>
					</button>
				</div>
				{#if !hasLyrics}
					<button
						class="search"
						disabled={lrcMngr.searching}
						onclick={async () => {
							if (manager.currentTrack) {
								await lrcMngr.searchLyrics(manager.currentTrack);
							}
						}}
					>
						<Search size={'1em'} />
						{lrcMngr.searching ? $_('lrc_related.searching') : $_('lrc_related.search_btn')}</button
					>
				{/if}
				<div class="volume">
					<div class="vol-icon">
						{#if manager.volume <= 0}
							<Volume size={'1.5em'} />
						{:else if manager.volume >= 0.7}
							<Volume2 size={'1.5em'} />
						{:else if manager.volume > 0}
							<Volume1 size={'1.5em'} />
						{:else}
							<Volume size={'1.5em'} />
						{/if}
					</div>
					<Slider
						bind:value={manager.volume}
						style="classic"
						color={'var(--text)'}
						thumbColor={'var(--text)'}
						backgroundColor="rgba(var(--rd), var(--gd), var(--bd), 0.2);"
					/>
				</div>
			</div>
		</div>
		<div class="infos">
			<div class="playing_info">
				<time class="currtime ns">{formatTime(manager.currentTime)}</time>
				<div class="progress-area">
					<div class="progressbar">
						<Slider
							max={manager.duration}
							bind:value={manager.currentTime}
							color={'var(--text)'}
							thumbColor={'var(--text)'}
							style="thick"
							backgroundColor="rgba(var(--rd), var(--gd), var(--bd), 0.2);"
						/>
					</div>
				</div>
				<time class="remaintime ns">-{formatTime(manager.duration - manager.currentTime)}</time>
			</div>
			<Marquee width={'40vw'}>
				<h2 class="ns">{manager.currentTrack?.title ?? 'Titre inconnu'}</h2>
			</Marquee>
			<Marquee width={'40vw'}>
				<p class="artist ns">{manager.currentTrack?.artists.join(', ') ?? 'Artiste inconnu'}</p>
			</Marquee>
		</div>
	</section>
	{#if hasLyrics}
		<section class="lrc" bind:this={lyricsParent}>
			{#each lrcMngr.lines as line, i}
				<Line {line} {lrcMngr} {manager} idx={i} {blurActive} />
			{/each}
		</section>
	{/if}
</div>
<audio
	bind:this={sound}
	bind:currentTime={manager.currentTime}
	bind:volume={manager.volume}
	onseeked={afterSeek}
	hidden
></audio>

<style>
	.__player .controls {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2em;
		height: 100%;
		width: 100%;
	}

	.__player .playing_info {
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1em;
	}

	.__player .playing_info time {
		font-family: var(--font-mono);
		opacity: 0.5;
		user-select: none;
		display: flex;
		justify-content: space-between;
	}

	.__player .playing_info .progress-area {
		width: 100%;
		height: 100%;
		display: flex;
		gap: 1em;
		justify-content: space-evenly;
		align-items: center;
	}

	.__player .playing_info .progressbar {
		width: 100%;
	}

	.__player .controls button {
		background: none;
		border: none;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0.7;
		transition: all 0.2s ease-in-out;
		cursor: pointer;
	}

	.__player .controls button:hover {
		opacity: 1;
	}

	.__player .controls:has(button:hover) button:not(:hover) {
		opacity: 0.3;
	}

	.__player .controls button:active {
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

	.cover .search {
		font-weight: bold;
		position: absolute;
		top: 2em;
		right: 2em;
		background: none;
		border: none;
		opacity: 0.5;
		cursor: pointer;
		transition: all 0.1s ease-in-out;
		background-color: var(--text);
		color: rgb(var(--r), var(--g), var(--b));
		padding-block: 0.2em;
		padding-inline: 0.3em;
		border-radius: 10px;
		display: flex;
		gap: 0.2em;
		justify-content: center;
		align-items: center;
	}

	.search:hover {
		opacity: 1;
	}

	.search:active {
		transform: scale(0.95);
	}

	.__player.blurActive .cover .search {
		color: var(--text);
		background: rgba(255, 255, 255, 0.25);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		border: 1px solid rgba(255, 255, 255, 0.18);
	}

	.__player {
		background-size: cover;
		color: var(--text);
		width: 100vw;
		height: 100vh;
		position: fixed;
		z-index: var(--player-z-index);
		top: 0;
		left: 0;
		display: grid;
		grid-template-columns: auto 1fr;
		align-items: center;
		gap: 2em;
		transform: translateY(200%);
		transition: transform 0.3s ease-in-out;

		padding-inline: 7vw;
		padding-block: 7vh;
	}

	.__player .dragzone {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 7vh;
		z-index: 1000;
	}

	.__player.no_lyrics {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.__player .background-images {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		z-index: -2;
		overflow: hidden;
		filter: brightness(var(--brightness)) blur(64px);
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
		position: absolute;
		width: 200%;
		height: 200%;
		object-fit: cover;
		background-size: cover;
	}

	.__player .background-images > div img {
		width: 100%;
		height: 100%;
	}

	@keyframes rot {
		0% {
			transform: rotate(var(--random-degree));
		}
		100% {
			transform: rotate(calc(var(--random-degree) + 1turn));
		}
	}

	.__player.blurActive::before {
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

	.__player:not(.blurActive) {
		background: var(--clr);
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
		width: 100%;
		overflow-y: auto;
		position: relative;
		padding-inline: 1em;
		scroll-behavior: smooth;

		mask: linear-gradient(
			180deg,
			transparent 0,
			black 6em,
			black calc(100% - 6em),
			transparent 100%
		);
	}

	.__player::-webkit-scrollbar {
		display: none;
	}

	.__player .lrc::-webkit-scrollbar {
		display: none;
	}

	.__player .cover {
		width: 60cqh;
		aspect-ratio: 1/1;
		box-shadow: rgba(17, 12, 46, 0.08) 0px 48px 100px 0px;
		border-radius: 10px;
		/* box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px; */
		background-size: cover;
		position: relative;
		overflow: hidden;
	}

	.__player.blurActive .cover .actions {
		background: rgba(0, 0, 0, 0.6);
	}

	.__player:not(.blurActive) .cover .actions {
		background: rgba(var(--r), var(--g), var(--b), 0.7);
	}

	.__player .cover .actions {
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		position: absolute;
		opacity: 0;
		transition: opacity 0.3s ease-in-out;
	}

	.__player .cover:hover .actions {
		opacity: 1;
	}

	.__player .infos {
		text-align: center;
		display: flex;
		align-items: center;
		justify-content: center;
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
