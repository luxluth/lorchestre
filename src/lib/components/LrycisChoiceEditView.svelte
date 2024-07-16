<script lang="ts">
	import { getLrc } from '$lib/lrc.svelte';
	import type { Lrc, LyricsResponse, Track } from '$lib/type';
	import { invoke } from '@tauri-apps/api/core';
	import LrcDisplay from './LrcDisplay.svelte';
	import { _ } from 'svelte-i18n';

	let {
		onClose,
		data
	}: {
		onClose: () => void;
		data: {
			response: LyricsResponse;
			track: Track;
		};
	} = $props();
	let selected = $state(false);
	//@ts-ignore
	let selection: Lrc = $state(null);
	const lrc = getLrc();

	async function save() {
		if (selection) {
			lrc.resetFromLines(data.track.duration, selection.parsed);
		}

		await invoke('save_lyrics', { input: selection.raw, path: data.track.file_path });
		onClose();
	}
</script>

<div class="frame">
	<p class="copy">{$_('lrc_related.copy')}</p>
	<h2>{$_('lrc_related.title_search')}</h2>

	<div class="choices">
		<div class="container">
			{#each data.response.lyrics as lrc}
				<div
					class="choice"
					role="radio"
					tabindex="0"
					aria-checked={selection !== null ? selection.raw === lrc.raw : false}
					onclick={() => {
						selected = true;
						selection = lrc;
					}}
					onkeydown={() => {}}
					class:selected={selection !== null ? selection.raw === lrc.raw : false}
				>
					<LrcDisplay lyrics={lrc.parsed} />
				</div>
			{/each}
		</div>
	</div>

	<div class="frame-actions">
		<button
			class="btn save"
			onclick={async () => {
				await save();
			}}
			class:inactive={!selected}>{$_('view.btn_save')}</button
		>
		<button
			class="close btn"
			onclick={() => {
				onClose();
			}}>{$_('view.btn_cancel')}</button
		>
	</div>
</div>

<style>
	.copy {
		position: absolute;
		right: 1em;
		top: 1em;
		font-weight: bold;
		opacity: 0.5;
	}

	.frame {
		width: 80vw;
		height: 82vh;
		background: var(--bg);
		border-radius: 10px;
		padding: 1em;
		position: relative;
	}

	.frame h2 {
		padding-bottom: 1em;
	}

	.frame-actions {
		display: flex;
		position: absolute;
		bottom: 1em;
		right: 1em;
		gap: 0.5em;
	}

	.choice {
		height: calc(65vh - 1em);
		width: fit-content;
		overflow: auto;
		padding: 1em;
		border-radius: 14px;
		padding-block: 1em;
		border: 4px var(--highlight) solid;
	}

	.choice::-webkit-scrollbar {
		display: none;
	}

	.choice.selected {
		border-color: var(--brand-color);
	}

	.choices {
		width: 100%;
		overflow: auto;
		display: flex;
		height: fit-content;
	}

	.choices .container {
		display: flex;
		gap: 1em;
		padding-bottom: 1vh;
	}

	.save {
		color: var(--bg);
		background-color: var(--fg);
	}
</style>
