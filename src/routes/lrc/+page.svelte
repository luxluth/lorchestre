<script lang="ts">
	import { getLrc } from '$lib/lrc.svelte';
	import { isElementVisible } from '$lib/utils';

	import { _ } from 'svelte-i18n';
	import { getManager } from '$lib/manager.svelte';
	import { onDestroy, onMount } from 'svelte';
	import Line from '../Line.svelte';

	let lrcParent: HTMLDivElement;

	let manager = getManager();
	let lrcMngr = getLrc();

	const hookRemove = lrcMngr.oncuechange(() => {
		const activeLines = lrcMngr.activeLines;
		if (activeLines.length > 0) {
			let child = lrcParent.children[activeLines[0].id];
			if (isElementVisible(child as HTMLElement)) {
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	});

	const unregister = manager.afterplay(() => {
		if (lrcParent) {
			setTimeout(() => {
				lrcParent.scrollTo({ behavior: 'smooth', top: 0 });
			}, 70);
		}
	});

	onDestroy(() => {
		unregister();
		hookRemove();
	});

	onMount(() => {
		const activeLines = lrcMngr.activeLines;
		if (activeLines.length > 0) {
			let child = lrcParent.children[activeLines[0].id];
			if (isElementVisible(child as HTMLElement)) {
				child.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	});
</script>

<div class="__lrc" style="--top: 5%; --bottom: 5%;">
	<div class="lines" bind:this={lrcParent}>
		{#if manager.currentTrack}
			{#if lrcMngr.lines.length > 0}
				{#each lrcMngr.lines as line, i}
					<Line {line} {lrcMngr} {manager} idx={i} blurActive={false} />
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
		margin-top: 5em;
		padding: 1em;
	}
</style>
