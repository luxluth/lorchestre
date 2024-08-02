<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	import close from '$lib/assets/window-icons/windows/close.svg?raw';
	import maximize from '$lib/assets/window-icons/windows/maximize.svg?raw';
	import minimize from '$lib/assets/window-icons/windows/minimize.svg?raw';
	// import restore from '$lib/assets/window-icons/windows/restore.svg?raw';

	let maximized = $state(false);

	(async () => {
		let window = getCurrentWindow();
		maximized = await window.isMaximized();
	})();
</script>

<div class="controls" data-tauri-drag-region>
	<div
		role="button"
		class="minimze"
		onclick={async () => {
			let window = getCurrentWindow();
			await window.minimize();
		}}
		tabindex="-1"
		onkeydown={() => {}}
	>
		{@html minimize}
	</div>
	<div
		role="button"
		class="maximize"
		onclick={async () => {
			let window = getCurrentWindow();
			maximized = await window.isMaximized();

			if (maximized) {
				await window.unmaximize();
				maximized = false;
			} else {
				await window.maximize();
				maximized = true;
			}
		}}
		tabindex="-1"
		onkeydown={() => {}}
	>
		{@html maximize}
	</div>

	<div
		role="button"
		class="close"
		onclick={async () => {
			let window = getCurrentWindow();
			await window.close();
		}}
		tabindex="-1"
		onkeydown={() => {}}
	>
		{@html close}
	</div>
</div>

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
</style>
