<script lang="ts">
	import type { Track } from '$lib/type';
	import AlignJustify from 'lucide-svelte/icons/align-justify';

	let {
		song,
		selected,
		toggleSelection
	}: {
		song: Track;
		selected: boolean;
		toggleSelection: (p: string) => void;
	} = $props();

	function trim(text: string, len = 20) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
	class="edit_song"
	role="presentation"
	tabindex="0"
	onkeydown={(e) => {
		if (e.key.toLowerCase() == 'enter') {
			toggleSelection(song.file_path);
		}
	}}
	onclick={() => {
		toggleSelection(song.file_path);
	}}
>
	<div
		class="checkbox"
		role="gridcell"
		tabindex="0"
		aria-colindex="1"
		data-checked={selected}
		onkeydown={(e) => {
			if (e.key.toLowerCase() == 'enter') {
				toggleSelection(song.file_path);
			}
		}}
		onclick={() => {
			toggleSelection(song.file_path);
		}}
	></div>
	<div class="title-part" aria-colindex="2" role="gridcell">
		<h4 class="title">{trim(song.title, 30)}</h4>
	</div>
	<div class="artist" aria-colindex="3" role="gridcell">
		{trim(song.artists.join(', '))}
	</div>
	<div class="album" aria-colindex="4" role="gridcell">
		<a tabindex="-1" href="/album/{song.album_id}">{trim(song.album)}</a>
	</div>
	<div class="move-icon" aria-colindex="5" role="gridcell">
		<AlignJustify />
	</div>
</div>

<style>
	* {
		color: var(--fg);
	}

	.move-icon {
		opacity: 0.3;
		display: flex;
		align-items: center;
		justify-content: flex-end;
	}

	h4 {
		font-weight: 400;
	}

	a {
		text-decoration: none;
		opacity: 0.5;
	}

	.artist {
		opacity: 0.3;
		font-size: 0.875em;
		margin: 0;
	}

	.album {
		font-size: 0.875em;
		opacity: 0.6;
	}

	a:hover {
		text-decoration: underline;
	}

	.edit_song {
		margin-bottom: 0.5em;
		background: var(--highlight);
		padding-block: 0.2em;
		width: 100%;
		display: grid;
		grid-template-columns:
			[index] 16px
			[first] minmax(120px, var(--col1, 6fr))
			[var1] minmax(120px, var(--col2, 4fr)) [var2] minmax(120px, var(--col3, 3fr)) [last] minmax(12px, var(--col4, 1fr));
		align-items: center;
		grid-gap: 16px;
		padding-block: 4px;
		padding-inline: 20px;
		height: 2.5em;
		border-radius: 4px;
	}
	.checkbox {
		width: 1em;
		height: 1em;
		border: 2px solid var(--fg);
		border-radius: 6px;
		display: flex;
		justify-content: center;
		align-items: center;
		background: none;
		color: var(--bg);
	}

	[data-checked='true'] {
		background-color: var(--fg);
	}
</style>
