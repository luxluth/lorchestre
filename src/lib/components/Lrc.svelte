<script lang="ts">
	import { getLrc } from '$lib/lrc.svelte';
	import { isElementVisible } from '$lib/utils';

	import { _ } from 'svelte-i18n';
	import { getManager } from '$lib/manager.svelte';
	import { getCmds } from '$lib/commands.svelte';

	let lrcParent: HTMLDivElement;

	let cmds = getCmds();
	let manager = getManager();
	let lrcMngr = getLrc();

	lrcMngr.oncuechange = () => {
		const activeLines = lrcMngr.activeLines;
		if (activeLines.length > 0) {
			let child = lrcParent.children[activeLines[0].id];
			if (isElementVisible(child as HTMLElement) || !cmds.lrc) {
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	};

	manager.afterplay = () => {
		if (lrcParent) {
			setTimeout(() => {
				lrcParent.scrollTo({ behavior: 'smooth', top: 0 });
			}, 70);
		}
	};
</script>

<div class="__lrc glass" class:active={cmds.lrc}>
	<div class="lines" bind:this={lrcParent}>
		{#if manager.currentTrack}
			{#if lrcMngr.lines.length > 0}
				{#each lrcMngr.lines as { text, endTime, startTime, id }}
					<div
						role="button"
						tabindex="0"
						onkeydown={() => {}}
						onclick={() => {
							manager.currentTime = startTime;
						}}
						class="line ns"
						id={id.toString()}
						data-star={startTime}
						data-end={endTime}
						class:active={lrcMngr.activeLines.find((i) => i.id === id)}
					>
						{text}
					</div>
				{/each}
			{:else}
				{$_('cmds.lrc.empty')}
			{/if}
		{:else}
			{$_('cmds.lrc.empty')}
		{/if}
	</div>
</div>

<style>
	.__lrc {
		position: fixed;
		z-index: var(--overlay-z-index);
		margin-top: 5em;
		height: 85%;
		width: 25.3em;
		padding: 1em;
		right: 2em;
		top: 0.5em;
		border: 2px solid rgba(100, 100, 100, 0.18);
		transform: translateX(200%);
		transition: transform 0.3s ease-in-out;
		overflow-y: scroll;
		border-radius: 8px;
	}

	.__lrc.active {
		transform: translateX(0);
	}

	.lines {
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.line {
		opacity: 0.3;
		font-weight: bold;
		font-size: 2rem;
		padding: 0.2em;
		border-radius: 4px;
		transition: all 0.2s ease-in-out;
	}

	.line:active {
		transform: scale(0.98);
	}

	.line:hover {
		opacity: 0.5;
		background-color: rgba(100, 100, 100, 0.18);
		cursor: pointer;
	}

	.line.active {
		opacity: 1;
	}
</style>
