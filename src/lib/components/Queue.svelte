<script lang="ts">
	import type Cmds from '$lib/commands.svelte';
	import { getContext } from 'svelte';
	import type Manager from '$lib/manager.svelte';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { flip } from 'svelte/animate';

	let cmds = getContext<Cmds>('cmds');
	let manager = getContext<Manager>('manager');

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

<div class="__queue glass" class:active={cmds.queue}>
	<h3>File d'attente</h3>
	<section class="songs">
		{#if manager.queue.length > 0}
			{#each manager.queue as track (track.id)}
				<div class="track ns" role="button" tabindex="0" animate:flip={{ duration: 200 }}>
					{#if track.cover}
						<div class="cover" style="background-image: url({convertFileSrc(track.cover)});"></div>
					{/if}
					<div class="infos">
						<div class="details">
							<h4>{track.title}</h4>
							<p>{track.artists[0]}</p>
						</div>
						<div class="duration">{formatTime(track.duration)}</div>
					</div>
				</div>
			{/each}
		{:else}
			Aucune chanson dans la file d'attente
		{/if}
	</section>
</div>

<style>
	.__queue {
		position: fixed;
		z-index: 50;
		margin-top: 5em;
		height: 50em;
		width: 25.3em;
		padding: 1em;
		right: 2em;
		top: 2em;
		border: 1px solid rgba(100, 100, 100, 0.18);
		transform: translateX(200%);
		transition: transform 0.3s ease-in-out;
		overflow: scroll;
	}

	.__queue.active {
		transform: translateX(0);
	}

	.__queue h3 {
		padding-bottom: 1em;
	}

	.__queue .songs {
		overflow-y: auto;
		overflow-x: hidden;
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	.songs .track {
		display: flex;
		gap: 1em;
		padding: 0.5em;
		align-items: center;
		border-radius: 8px;
	}

	.songs .track:hover {
		background: rgba(100, 100, 100, 0.18);
		cursor: pointer;
	}

	.songs .track:active {
		transform: scale(0.99);
	}

	.track .cover {
		min-width: 3em;
		aspect-ratio: 1/1;
		background-size: cover;
		border-radius: 4px;
	}

	.track .infos {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.track .infos .duration {
		opacity: 0.3;
	}

	.infos .details p {
		opacity: 0.5;
		margin-top: 0.1em;
	}
</style>