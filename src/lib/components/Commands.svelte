<script lang="ts">
	import List from 'lucide-svelte/icons/list';
	import MicVocal from 'lucide-svelte/icons/mic-vocal';
	import { getManager } from '$lib/manager.svelte';
	import { getCmds } from '$lib/commands.svelte';
	import { goto } from '$app/navigation';

	let cmds = getCmds();
	let manager = getManager();

	type ToActivate = 'lrc' | 'queue';

	function activate(t: ToActivate) {
		switch (t) {
			case 'lrc':
				cmds.queue = false;
				cmds.lrc = !cmds.lrc;
				goto('/lrc');
				break;
			case 'queue':
				cmds.lrc = false;
				cmds.queue = !cmds.queue;
				goto('/queue');
				break;
		}
	}
</script>

<div class="__commands" class:dead={typeof manager.currentTrack === 'undefined'}>
	<button
		class="lyrics"
		class:active={cmds.lrc}
		onclick={() => {
			activate('lrc');
		}}
	>
		<MicVocal size={'20px'} />
	</button>
	<button
		class="queue"
		class:active={cmds.queue}
		onclick={() => {
			activate('queue');
		}}
	>
		<List size={'20px'} />
	</button>
</div>

<style>
	.__commands {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		width: fit-content;
		gap: 0.5em;
		padding-inline: 1em;
	}

	.__commands button {
		color: var(--fg);
		width: 2.5em;
		height: 2.5em;
		display: flex;
		justify-content: center;
		align-items: center;
		opacity: 0.8;
		transition: opacity 0.1s ease-in-out;
		cursor: pointer;
		padding: 0.2em;
	}

	.__commands button:active {
		transform: scale(0.95);
	}

	.__commands button {
		background: none;
		border: none;
		border-radius: 4px;
		border: 1px solid rgba(100, 100, 100, 0);
	}

	.__commands button.active {
		opacity: 1;
		border: 1px solid rgba(100, 100, 100, 0.18);
		background: rgba(100, 100, 100, 0.18);
	}

	.__commands button:hover {
		opacity: 1;
	}

	.__commands.dead {
		opacity: 0.5;
		/* pointer-events: none; */
	}
</style>
