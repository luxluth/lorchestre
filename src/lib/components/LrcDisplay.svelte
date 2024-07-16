<script lang="ts">
	import type { LyricLine } from '$lib/type';
	import { formatTime } from '$lib/utils';

	let { lyrics }: { lyrics: LyricLine[] } = $props();
</script>

<div class="lyrics ns">
	{#each lyrics as lyric}
		<div class="lrc">
			<span class="time">{formatTime(lyric.start_time / 1000)}</span>
			<span class="text">{lyric.text}</span>
		</div>
	{/each}
</div>

<style>
	.lrc {
		width: 35vw;
		display: flex;
		align-items: center;
		background: var(--highlight);
		padding: 1em;
		gap: 0.5em;
		border-radius: 8px;
	}

	.text {
		font-weight: bold;
	}

	.lyrics {
		height: 100%;
		overflow-block: scroll;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.time {
		font-family: var(--font-mono);
		opacity: 0.6;
	}
</style>
