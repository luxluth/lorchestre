<script lang="ts">
	import { flip } from 'svelte/animate';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import { dndzone, type DndEvent } from 'svelte-dnd-action';

	import { _ } from 'svelte-i18n';
	import { getCoverUri } from '$lib/utils';
	import type { QueueTrack } from '$lib/type';
	import X from 'lucide-svelte/icons/x';
	import { getManager } from '$lib/manager.svelte';
	import { getAppConfig } from '$lib/config.svelte';

	let manager = getManager();
	let config = getAppConfig();

	const flipDurationMs = 200;

	function handleDndConsiderColumns(e: CustomEvent<DndEvent<QueueTrack>>) {
		manager.queue = e.detail.items;
	}
	function handleDndFinalizeColumns(e: CustomEvent<DndEvent<QueueTrack>>) {
		manager.queue = e.detail.items;
	}

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

	function trim(text: string, len = 20) {
		return text.slice(0, len) + (text.length > len ? '...' : '');
	}
</script>

<div class="__queue">
	<header>
		<h3>{$_('cmds.waitlist.title')}</h3>
		<button
			class:inactive={manager.queue.length === 0}
			class="btn"
			data-kind="desctructive"
			onclick={() => {
				manager.clearQueue();
			}}
		>
			<div class="icon">
				<Trash2 size={'1em'} />
			</div>
			{$_('cmds.waitlist.clear_queue')}
		</button>
	</header>
	<section
		class="songs"
		use:dndzone={{
			items: manager.queue,
			flipDurationMs,
			type: 'columns',
			dropTargetStyle: {
				backgroundColor: 'var(--bg)',
				opacity: '0.5'
			}
		}}
		onconsider={handleDndConsiderColumns}
		onfinalize={handleDndFinalizeColumns}
	>
		{#if manager.queue.length > 0}
			{#each manager.queue as track (track.id)}
				<div
					class="track ns"
					role="button"
					tabindex="0"
					animate:flip={{ duration: flipDurationMs }}
					draggable
					ondblclick={async () => {
						await manager.shiftTo(track);
					}}
				>
					<div
						class="cover"
						style="background-image: url({getCoverUri(track.album_id, track.cover_ext, config)});"
					></div>
					<div class="infos">
						<div class="details">
							<h4>{trim(track.title)}</h4>
							<p>{track.artists[0]}</p>
						</div>
						<div class="duration">
							<button
								onclick={() => {
									manager.remove(track);
								}}><X color={'var(--fg)'} /></button
							>
							<time>{formatTime(track.duration)}</time>
						</div>
					</div>
				</div>
			{/each}
		{:else}
			{$_('cmds.waitlist.empty')}
		{/if}
	</section>
</div>

<style>
	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 2em;
	}

	.__queue .songs {
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
		background: var(--highlight);
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
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.track .infos .duration button {
		background: none;
		border: none;
		display: none;
	}

	.track:hover .infos .duration {
		opacity: 1;
	}

	.track:hover .infos .duration button {
		display: block;
	}

	.track:hover .infos .duration time {
		display: none;
	}

	.infos .details p {
		opacity: 0.5;
		margin-top: 0.1em;
	}
</style>
