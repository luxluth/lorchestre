<script lang="ts">
	import type LrcManager from '$lib/lrc.svelte';
	import type Manager from '$lib/manager.svelte';
	import type { Line } from '$lib/type';
	import { getLNTime } from '$lib/utils';

	const {
		line,
		lrcMngr,
		idx,
		manager,
		blurActive
	}: {
		line: Line;
		lrcMngr: LrcManager;
		idx: number;
		manager: Manager;
		blurActive: boolean;
	} = $props();

	let isInstrumental = $derived(
		line.text == '♪' ||
			line.isInstrumental ||
			(line.text == '' && line.endTime - line.startTime >= 5)
	);
	let empty = $derived(line.text == '' && line.endTime - line.startTime < 5);
</script>

<div
	class="line ns"
	class:blurActive
	data-time={line.startTime}
	class:active={lrcMngr.activeLines.find((i) => i.id === line.id)}
	onclick={() => manager.seekTo(line.startTime)}
	onkeydown={() => {}}
	role="button"
	tabindex="0"
	class:isMainVocal={lrcMngr.isMainVocal(idx)}
	data-alignment={lrcMngr.getOrder(idx) % 2}
	data-artist={lrcMngr.getArtistName(idx)}
	class:instrumental={isInstrumental}
	class:empty
>
	{#if isInstrumental}
		<div class="dot"></div>
	{:else}
		<div class="text">
			{#each line.syllables as syllable}
				<span data-timems={getLNTime(syllable.start)}>{syllable.text}</span>
			{/each}
		</div>
	{/if}
	{#if line.vocals.length > 0}
		<div class="vocals">
			{#each line.vocals as vocal}
				<div class="vocal">
					{#each vocal.syllables as syllable}
						<span data-timems={getLNTime(syllable.start)}>{syllable.text}</span>
					{/each}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	* {
		--inactive-text-color: rgba(var(--rd), var(--gd), var(--bd), 0.2);
	}
	.line[data-alignment='1'] {
		text-align: right;
	}

	.line:first-child {
		margin-top: 50%;
	}

	.line:last-child {
		margin-bottom: 50%;
	}

	@keyframes activeLineAnimation {
		0% {
			opacity: 0.8;
			/* text-shadow: none; */
		}
		50% {
			opacity: 0.9;
			/* text-shadow: 0 0 10px rgba(255, 255, 255, 0.5); */
		}
		100% {
			opacity: 1;
			/* text-shadow: 0 0 10px rgba(255, 255, 255, 1); */
		}
	}

	.line {
		font-size: 3em;
		padding: 0.25em;
		font-weight: 600;
		opacity: 0.3;
		cursor: pointer;
		line-height: 1;
		border-radius: 8px;
		margin-block: 0.5em;
		transition: opacity 0.3s ease-in;
	}

	.line.active {
		animation: activeLineAnimation 0.3s ease-in-out forwards;
	}

	.line:active {
		transform: scale(0.98);
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
		background-color: var(--text, --fg);
		opacity: 1;
		height: 0.5em;
		width: 0.5em;
		border-radius: 50%;
	}

	.line.active.instrumental {
		scale: 1;
		height: auto;
	}

	.line.blurActive:hover {
		opacity: 0.5;
		background: rgba(255, 255, 255, 0.2);
	}

	.line:not(.blurActive):hover {
		opacity: 0.5;
	}

	.line.active:hover {
		opacity: 1;
		background: rgba(255, 255, 255, 0);
	}

	.line .vocals {
		padding-top: 0.3em;
	}

	.line .vocal {
		font-size: 1.5rem;
	}
</style>
