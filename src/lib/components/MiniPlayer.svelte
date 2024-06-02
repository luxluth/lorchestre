<script lang="ts">
	import Shuffle from 'lucide-svelte/icons/shuffle';
	import Repeat from 'lucide-svelte/icons/repeat';
	import Play from 'lucide-svelte/icons/play';
	import Pause from 'lucide-svelte/icons/pause';
	import FastForward from 'lucide-svelte/icons/fast-forward';
	import Rewind from 'lucide-svelte/icons/rewind';
	import Music3 from 'lucide-svelte/icons/music-3';
	import Maximize2 from 'lucide-svelte/icons/maximize-2';
	import Volume from 'lucide-svelte/icons/volume';
	import Volume1 from 'lucide-svelte/icons/volume-1';
	import Volume2 from 'lucide-svelte/icons/volume-2';

	import { convertFileSrc } from '@tauri-apps/api/core';
	import Slider from './Slider.svelte';
	import type Manager from '$lib/manager.svelte';
	import { getContext } from 'svelte';
	import { _ } from 'svelte-i18n';

	let manager = getContext<Manager>('manager');
	let percentage = $derived((manager.currentTime * 100) / manager.duration);

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

	function playAt(pos: number) {
		manager.timeTo(pos * manager.duration);
	}
</script>

<div class="mp" class:dead={typeof manager.currentTrack === 'undefined'}>
	{#if manager.currentTrack}
		<section
			class="player"
			style="--clr: {manager.currentTrack?.color
				? `rgb(${manager.currentTrack?.color.r}, ${manager.currentTrack?.color.g}, ${manager.currentTrack?.color.b})`
				: 'var(--bg)'};"
		>
			{#if manager.currentTrack.cover}
				<div
					class="cover"
					style="background-image: url({convertFileSrc(manager.currentTrack.cover)});"
				>
					<div
						class="expand"
						onclick={() => {
							manager.activatePlayer();
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
							manager.activatePlayer();
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
					<h4>{manager.currentTrack.title}</h4>
					<p>
						<span>{manager.currentTrack.artists.join(', ')}</span> <span>—</span>
						<span
							><a href="/album/{manager.currentTrack.album_id}">{manager.currentTrack.album}</a
							></span
						>
					</p>
				</div>
				<time class="currtime ns">{formatTime(manager.currentTime)}</time>
				<div class="progressbar" style="--percent: {percentage}%;">
					<Slider
						value={percentage / 100}
						style="minimal"
						oninput={(data) => {
							playAt(data);
						}}
					/>
				</div>
				<time class="remaintime ns">-{formatTime(manager.duration - manager.currentTime)}</time>
			</div>
		</section>
		<section class="controls">
			<button>
				<Shuffle size={'1.5em'} />
			</button>
			<div class="actions">
				<button>
					<Rewind
						fill={'var(--fg)'}
						size={'2.5em'}
						onclick={async () => {
							await manager.prev();
						}}
					/>
				</button>
				<button
					class="playpause"
					onclick={async () => {
						await manager.tooglepp();
					}}
				>
					{#if manager.paused}
						<Play fill={'var(--fg)'} size={'2.5em'} />
					{:else}
						<Pause fill={'var(--fg)'} size={'2.5em'} />
					{/if}
				</button>
				<button>
					<FastForward
						fill={'var(--fg)'}
						size={'2.5em'}
						onclick={async () => {
							await manager.next();
						}}
					/>
				</button>
			</div>
			<button>
				<Repeat size={'1.5em'} />
			</button>
		</section>
		<section class="volume">
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
				oninput={(data) => {
					manager.volumeTo(data);
				}}
			/>
		</section>
	{:else}
		<section class="player_shell">
			<div class="fakecover">
				<Music3 />
			</div>
			<div class="icon">{$_('no_media')}</div>
		</section>
	{/if}
</div>

<style>
	.mp {
		width: 100%;
		height: 5em;
		padding: 0.3em;
		display: flex;
		justify-content: flex-start;
		gap: 1em;
		flex-grow: 1;
	}

	.mp.dead {
		pointer-events: none;
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
