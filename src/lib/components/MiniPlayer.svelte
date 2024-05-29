<script lang="ts">
	import { type Track, type PlayerDispatch, PlayerDispatchKind } from '$lib/type';
	import { player } from '$lib/events';
	import {
		Shuffle,
		Repeat,
		Play,
		Pause,
		FastForward,
		Rewind,
		Music3,
		Maximize2,
		Volume,
		Volume1,
		Volume2
	} from 'lucide-svelte';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import Slider from './Slider.svelte';

	let track = $state<Track>();
	let currentTime = $state(0);
	let duration = $derived(track ? track.duration : 0);
	let percentage = $derived((currentTime * 100) / duration);
	let paused = $state(true);
	let volume = $state(1);

	function formatTime(time: number) {
		if (isNaN(time)) {
			return '-:--';
		}
		if (time >= 60 * 60) {
			return new Date(time * 1000).toISOString().substring(11, 16);
		} else {
			return new Date(time * 1000).toISOString().substring(14, 19);
		}
	}

	function handleDispatch(e: CustomEvent<PlayerDispatch>) {
		const ev = e.detail;

		switch (ev.kind) {
			case PlayerDispatchKind.TimeUpdate:
				currentTime = ev.data;
				break;
			case PlayerDispatchKind.NewTrack:
				track = ev.data;
				break;
			case PlayerDispatchKind.PlayPause:
				if (ev.data == 'play') {
					paused = false;
				} else {
					paused = true;
				}
				break;
			case PlayerDispatchKind.VolumeChange:
				volume = ev.data;
				break;
		}
	}

	function toggleMediaPlayState() {
		player.togglePlayPause();
	}

	function playAt(pos: number) {
		player.setTimeTo(pos * duration);
	}

	$effect(() => {
		//@ts-ignore
		document.addEventListener(player.PLAYER_DISPATCH, handleDispatch);

		return () => {
			//@ts-ignore
			document.removeEventListener(player.PLAYER_DISPATCH, handleDispatch);
		};
	});
</script>

<div class="mp" class:dead={typeof track === 'undefined'}>
	{#if track}
		<section class="controls">
			<button>
				<Shuffle size={'1.5em'} />
			</button>
			<div class="actions">
				<button>
					<Rewind fill={'var(--fg)'} size={'2.5em'} />
				</button>
				<button class="playpause" onclick={toggleMediaPlayState}>
					{#if paused}
						<Play fill={'var(--fg)'} size={'2.5em'} />
					{:else}
						<Pause fill={'var(--fg)'} size={'2.5em'} />
					{/if}
				</button>
				<button>
					<FastForward fill={'var(--fg)'} size={'2.5em'} />
				</button>
			</div>
			<button>
				<Repeat size={'1.5em'} />
			</button>
		</section>
		<section
			class="player"
			style="--clr: {track?.color
				? `rgb(${track?.color.r}, ${track?.color.g}, ${track?.color.b})`
				: 'var(--bg)'};"
		>
			{#if track.cover}
				<div class="cover" style="background-image: url({convertFileSrc(track.cover)});">
					<div
						class="expand"
						onclick={() => {
							player.activate();
						}}
						role="button"
						tabindex="0"
						onkeydown={() => {}}
					>
						<Maximize2 />
					</div>
				</div>
			{:else}
				<div class="fakecover">
					<Music3 />
					<div
						class="expand"
						onclick={() => {
							player.activate();
						}}
						role="button"
						tabindex="0"
						onkeydown={() => {}}
					>
						<Maximize2 />
					</div>
				</div>
			{/if}
			<div class="details">
				<div class="track-infos">
					<h4>{track.title}</h4>
					<p>
						<span>{track.artists.join(', ')}</span> <span>—</span>
						<span><a href="/album/{track.album_id}">{track.album}</a></span>
					</p>
				</div>
				<time class="currtime ns">{formatTime(currentTime)}</time>
				<div class="progressbar" style="--percent: {percentage}%;">
					<Slider
						value={percentage / 100}
						style="minimal"
						oninput={(data) => {
							playAt(data);
						}}
					/>
					<!-- <div class="progress"> -->
					<!-- 	<div class="shadow"></div> -->
					<!-- </div> -->
				</div>
				<time class="remaintime ns">-{formatTime(duration - currentTime)}</time>
			</div>
		</section>
		<section class="volume">
			<div class="vol-icon">
				{#if volume === 0}
					<Volume size={'1.5em'} />
				{:else if volume >= 0.7}
					<Volume2 size={'1.5em'} />
				{:else if volume > 0}
					<Volume1 size={'1.5em'} />
				{/if}
			</div>
			<Slider
				value={volume}
				oninput={(data) => {
					player.setVolumeTo(data);
				}}
			/>
		</section>
	{:else}
		<section class="player_shell">
			<div class="fakecover">
				<Music3 />
			</div>
			<div class="icon">Aucune chanson en cours de lecture</div>
		</section>
	{/if}
</div>

<style>
	.mp {
		width: 100%;
		height: 5em;
		padding: 0.3em;
		display: flex;
		justify-content: space-evenly;
		gap: 0.5em;
	}

	.mp.dead {
		pointer-events: none;
		justify-content: center;
	}

	.volume {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5em;
	}

	.vol-icon {
		opacity: 0.5;
	}

	.controls {
		display: flex;
		gap: 1em;
		justify-content: center;
		align-items: center;
		gap: 2em;
	}

	.controls .actions {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1em;
	}

	.controls button {
		background: none;
		border: none;
		opacity: 0.5;
		cursor: pointer;
		transition: opacity 0.2s ease-in-out;
		color: var(--fg);
	}

	.controls button:hover {
		opacity: 1;
	}

	.controls button:active {
		transform: scale(0.95);
	}

	.player,
	.player_shell {
		width: 40em;
		border-radius: 3px;
		overflow: hidden;
		background-color: var(--bg);
		border: 1px solid rgba(100, 100, 100, 0.18);
		display: flex;
		align-items: center;
	}

	.cover {
		height: 100%;
		aspect-ratio: 1/1;
		background-size: cover;
		background-color: var(--clr);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.cover,
	.fakecover {
		position: relative;
		border-right: 0.5px solid rgba(100, 100, 100, 0.18);
	}

	.expand {
		cursor: pointer;
		background: rgba(0, 0, 0, 0.5);
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
	}

	.expand:active :global(svg) {
		transform: scale(0.95);
	}

	.expand :global(svg) {
		opacity: 1;
		color: var(--dark-fg);
	}

	.cover:hover .expand,
	.fakecover:hover .expand {
		opacity: 1;
	}

	.fakecover {
		height: 100%;
		aspect-ratio: 1/1;
		background: rgba(100, 100, 100, 0.18);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.fakecover :global(svg) {
		opacity: 0.3;
	}

	.player .details {
		flex-grow: 1;
		position: relative;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.player .details .track-infos {
		text-align: center;
	}

	.player .details .track-infos p {
		opacity: 0.5;
	}

	.player .details .track-infos a {
		text-decoration: none;
		color: var(--fg);
	}

	.player .details .track-infos a:hover {
		text-decoration: underline;
	}

	.player .details .progressbar {
		position: absolute;
		bottom: 0;
		left: 0;
		width: 100%;
		height: 2px;
		background-color: rgba(100, 100, 100, 0.18);
	}

	.progressbar .progress {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		overflow: hidden;
	}

	.progress .shadow {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: var(--percent);
		background-color: var(--fg);
	}

	time {
		position: absolute;
		bottom: 10px;
		opacity: 0.3;
		font-size: 0.9em;
	}

	time.currtime {
		left: 8px;
	}

	time.remaintime {
		right: 8px;
	}

	.player_shell .icon {
		flex-grow: 1;
		text-align: center;
		opacity: 0.4;
		font-weight: bold;
	}

	/* button :global(svg) { */
	/* 	width: 100%; */
	/* 	height: 100%; */
	/* } */
</style>
