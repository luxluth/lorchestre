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
	import { formatTime, getCoverUri } from '$lib/utils';
	import { PlayingMode } from '$lib/type';
	import { getManager } from '$lib/manager.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	let manager = getManager();
	let percentage = $derived((manager.currentTime * 100) / manager.duration);
	let config = getAppConfig();
</script>

<div
	class="mp"
	onclick={() => {
		console.log('rferf');
		manager.activatePlayer();
	}}
	role="presentation"
	tabindex="-1"
	onkeydown={() => {}}
	class:dead={manager.currentTrack === null}
></div>

<style>
	.mp {
		padding: 0.3em;
		position: fixed;
		bottom: 2vw;
		right: 2vw;
		width: 5em;
		height: 5em;
		border-radius: 12px;
		border: 1px solid #404040;
		background-color: #2f2f2f;
		box-shadow: rgba(0, 0, 0, 0.25) 1px 1px 8px;
	}

	.mp:active {
		transform: scale(0.98);
	}

	.mp.dead {
		display: none;
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
		color: var(--brand-color);
	}

	.controls button[data-mode='repeat'],
	.controls button[data-mode='repeat-all'] {
		opacity: 1;
		color: var(--brand-color);
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
