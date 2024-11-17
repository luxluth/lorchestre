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

	import Slider from './Slider.svelte';
	import Marquee from './Marquee.svelte';
	import { _ } from 'svelte-i18n';
	import { getCoverUri } from '$lib/utils';
	import { PlayingMode } from '$lib/type';
	import { getManager } from '$lib/manager.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	let manager = getManager();
	let percentage = $derived((manager.currentTime * 100) / manager.duration);
	let config = getAppConfig();

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
</script>

<div class="mp ns" class:dead={typeof manager.currentTrack === 'undefined'} data-tauri-drag-region>
	{#if manager.currentTrack}
		<section class="controls">
			<button
				class:active={manager.pmode === PlayingMode.Shuffle}
				onclick={async () => {
					await manager.toggleShuffle();
				}}
			>
				<Shuffle size={'1.5em'} />
			</button>
			<div class="actions">
				<button>
					<Rewind
						fill={'var(--fg)'}
						size={'20px'}
						onclick={async () => {
							await manager.prev();
						}}
					/>
				</button>
				<button
					class="playpause"
					onclick={async () => {
						await manager.togglepp();
					}}
				>
					{#if manager.paused}
						<Play fill={'var(--fg)'} size={'20px'} />
					{:else}
						<Pause fill={'var(--fg)'} size={'20px'} />
					{/if}
				</button>
				<button>
					<FastForward
						fill={'var(--fg)'}
						size={'20px'}
						onclick={async () => {
							await manager.next();
						}}
					/>
				</button>
			</div>
			<button
				data-mode={manager.qmode}
				onclick={() => {
					manager.cycleThroughQmode();
				}}
			>
				<Repeat size={'20px'} />
			</button>
		</section>
		<section
			class="player"
			style="--clr: {manager.currentTrack?.color
				? `rgb(${manager.currentTrack?.color.r}, ${manager.currentTrack?.color.g}, ${manager.currentTrack?.color.b})`
				: 'var(--bg)'};"
		>
			<div
				class="cover"
				style="background-image: url({getCoverUri(
					manager.currentTrack.album_id,
					manager.currentTrack.cover_ext,
					config,
					70
				)});"
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
			<div class="details">
				<div class="track-infos">
					<Marquee width={'20vw'}>
						<h4>{manager.currentTrack.title}</h4>
					</Marquee>
					<Marquee width={'14vw'}>
						<p>
							<span>{manager.currentTrack.artists.join(', ')}</span> <span>—</span>
							<span
								><a href="/album/{manager.currentTrack.album_id}">{manager.currentTrack.album}</a
								></span
							>
						</p>
					</Marquee>
				</div>
				<time class="currtime ns">{formatTime(manager.currentTime)}</time>
				<div class="progressbar" style="--percent: {percentage.toFixed(0)}%;">
					<Slider max={manager.duration} bind:value={manager.currentTime} style="minimal" />
				</div>
				<time class="remaintime ns">-{formatTime(manager.duration - manager.currentTime)}</time>
			</div>
		</section>
		<section class="volume">
			<div class="vol-icon">
				{#if manager.sliderValue <= 0}
					<Volume size={'20px'} />
				{:else if manager.sliderValue >= 0.7}
					<Volume2 size={'20px'} />
				{:else if manager.sliderValue > 0}
					<Volume1 size={'20px'} />
				{:else}
					<Volume size={'1.5em'} />
				{/if}
			</div>
			<Slider bind:value={manager.sliderValue} style="classic" />
		</section>
	{:else}
		<section class="controls">
			<button>
				<Shuffle size={'1.5em'} />
			</button>
			<div class="actions">
				<button>
					<Rewind fill={'var(--fg)'} size={'20px'} />
				</button>
				<button class="playpause">
					{#if manager.paused}
						<Play fill={'var(--fg)'} size={'20px'} />
					{:else}
						<Pause fill={'var(--fg)'} size={'20px'} />
					{/if}
				</button>
				<button>
					<FastForward fill={'var(--fg)'} size={'20px'} />
				</button>
			</div>
			<button>
				<Repeat size={'20px'} />
			</button>
		</section>
		<section class="player_shell">
			<div class="fakecover">
				<Music3 />
			</div>
			<div class="icon">{$_('no_media')}</div>
		</section>
		<section class="volume" style="opacity: .5;">
			<div class="vol-icon">
				{#if manager.sliderValue <= 0}
					<Volume size={'20px'} />
				{:else if manager.sliderValue >= 0.7}
					<Volume2 size={'20px'} />
				{:else if manager.sliderValue > 0}
					<Volume1 size={'20px'} />
				{:else}
					<Volume size={'1.5em'} />
				{/if}
			</div>
			<Slider value={manager.sliderValue} style="classic" />
		</section>
	{/if}
</div>

<style>
	.mp {
		width: 100%;
		padding: 0.3em;
		display: flex;
		gap: 1em;
		justify-content: space-evenly;
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
		gap: 1em;
	}

	.controls .actions {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1em;
	}

	.controls button {
		background: none;
		position: relative;
		border: none;
		opacity: 0.5;
		cursor: pointer;
		transition: opacity 0.2s ease-in-out;
		color: var(--fg);
	}

	.controls button.active {
		opacity: 1;
		color: var(--fg);
	}

	.controls button[data-mode='repeat'],
	.controls button[data-mode='repeat-all'] {
		opacity: 1;
		color: var(--fg);
	}

	.controls button[data-mode='repeat']::after {
		content: '1';
		position: absolute;
		width: 0.5em;
		height: 0.5em;
		border-radius: 50%;
		font-size: 0.5em;
		top: 6px;
		left: 8px;
	}

	.controls button:hover {
		opacity: 1;
	}

	.controls button:active {
		transform: scale(0.95);
	}

	.player,
	.player_shell {
		width: 27vw;
		height: 3.5em;
		padding: 0.2em;
		border-radius: 6px;
		overflow: hidden;
		border: 1px solid rgba(100, 100, 100, 0.18);
		display: flex;
		gap: 1em;
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
		border-radius: 4px;
		overflow: hidden;
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

	.cover:hover .expand {
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
		display: flex;
		flex-direction: column;
		align-items: center;
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
		font-family: var(--font-mono);
		bottom: 8px;
		opacity: 0.3;
		font-size: 0.85vw;
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
