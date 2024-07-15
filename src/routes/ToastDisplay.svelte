<script lang="ts">
	import Toast from '$lib/components/Toast.svelte';
	import { getToastManager } from '$lib/toast.svelte';

	const tm = getToastManager();
</script>

{#if tm.toasts.length > 0}
	<div class="__toast">
		{#each tm.toasts as toast}
			<Toast {toast} />
		{/each}
	</div>
{/if}

<style>
	.__toast {
		z-index: var(--toast-z-index);
		position: fixed;
		bottom: 1.5em;
		right: 1.5em;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}
</style>
