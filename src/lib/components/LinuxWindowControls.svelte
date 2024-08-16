<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	import gnomeWindowMinimize from '$lib/assets/window-icons/linux/gnome/window-minimize-symbolic.svg?raw';
	import gnomeWindowMaximize from '$lib/assets/window-icons/linux/gnome/window-maximize-symbolic.svg?raw';
	import gnomeWindowClose from '$lib/assets/window-icons/linux/gnome/window-close-symbolic.svg?raw';

	import kdeWindowMinimize from '$lib/assets/window-icons/linux/kde/window-minimize.svg?raw';
	import kdeWindowMaximize from '$lib/assets/window-icons/linux/kde/window-maximize.svg?raw';
	import kdeWindowClose from '$lib/assets/window-icons/linux/kde/window-close.svg?raw';

	let desktop: 'gnome' | 'kde' = $state('gnome');
	let buttons: ('minimize' | 'maximize' | 'close')[] = $state(['minimize', 'maximize', 'close']);

	(async () => {
		desktop = await invoke('desktop');
		if (desktop === 'gnome') {
			buttons = await invoke('gnome_window_controls');
		}
	})();
</script>

{#if desktop === 'gnome' || desktop === 'kde'}
	<div class="controls" data-tauri-drag-region>
		{#each buttons as button}
			{#if button === 'minimize'}
				<div
					role="button"
					class="minimze"
					data-desktop={desktop}
					onclick={async () => {
						let window = getCurrentWindow();
						await window.minimize();
					}}
					tabindex="-1"
					onkeydown={() => {}}
				>
					{#if desktop === 'kde'}
						{@html kdeWindowMinimize}
					{:else}
						{@html gnomeWindowMinimize}
					{/if}
				</div>
			{/if}
			{#if button === 'maximize'}
				<div
					role="button"
					class="maximize"
					data-desktop={desktop}
					onclick={async () => {
						let window = getCurrentWindow();
						await window.maximize();
					}}
					tabindex="-1"
					onkeydown={() => {}}
				>
					{#if desktop === 'kde'}
						{@html kdeWindowMaximize}
					{:else}
						{@html gnomeWindowMaximize}
					{/if}
				</div>
			{/if}
			{#if button === 'close'}
				<div
					role="button"
					class="close"
					data-desktop={desktop}
					onclick={async () => {
						let window = getCurrentWindow();
						await window.close();
					}}
					tabindex="-1"
					onkeydown={() => {}}
				>
					{#if desktop === 'kde'}
						{@html kdeWindowClose}
					{:else}
						{@html gnomeWindowClose}
					{/if}
				</div>
			{/if}
		{/each}
	</div>
{/if}

<style>
	.controls {
		display: flex;
		padding-right: calc(0.625em * 2);
		gap: 0.8125em;
	}

	div[role='button'] {
		color: var(--fg);
		height: 1.5em;
		width: 1.5em;
		border-radius: 50%;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	div[data-desktop='gnome'] {
		background: var(--highlight);
	}
</style>
