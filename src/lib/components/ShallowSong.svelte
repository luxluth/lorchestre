<script lang="ts">
	import type { Track } from '$lib/type';

	let {
		song,
		selected,
		toggleSelection
	}: {
		song: Track;
		selected: boolean;
		toggleSelection: (p: string) => void;
	} = $props();
</script>

<div class="edit_song">
	<div class="checkbox" data-checked={selected}></div>
</div>

<style>
	.checkbox {
		width: 1em;
		height: 1em;
		border: 2px solid var(--brand-color);
		border-radius: 6px;
		display: flex;
		justify-content: center;
		align-items: center;
		background: none;
		color: var(--bg);
	}

	[data-checked='true'] {
		background-color: var(--brand-color);
	}
</style>
