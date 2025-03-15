<script lang="ts">
	import { getToastManager } from '$lib/toast.svelte';
	import { ToastKind, type Toast } from '$lib/type';
	import { flyAndScale } from '$lib/utils/transitions';
	import CircleX from 'lucide-svelte/icons/circle-x';
	import CircleCheck from 'lucide-svelte/icons/circle-check';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';

	let { toast }: { toast: Toast } = $props();
	const tm = getToastManager();
</script>

<div
	class="glass toast ns"
	transition:flyAndScale
	role="button"
	tabindex="-1"
	data-kind={toast.kind}
	onkeydown={() => {}}
	id={toast.id.toString()}
	ondblclick={() => {
		if (toast.kind !== ToastKind.Loading) {
			tm.close(toast.id);
		}
	}}
>
	{#if toast.kind === ToastKind.Error}
		<div class="icon">
			<CircleX size="14px" />
		</div>
	{:else if toast.kind === ToastKind.Loading}
		<div class="icon loading">
			<LoaderCircle size="14px" />
		</div>
	{:else if toast.kind === ToastKind.Success}
		<div class="icon">
			<CircleCheck size="14px" />
		</div>
	{/if}

	<div class="details">
		{#if toast.title}
			<h4>{toast.title}</h4>
		{/if}
		<p>{toast.message}</p>
	</div>
</div>

<style>
	.toast {
		display: flex;
		align-items: center;
		width: 20em;
		border: 0px;
		padding: 0.7em;
		gap: 0.5em;

		background: var(--bg);
		color: var(--fg);
		box-shadow: rgba(17, 12, 46, 0.15) 0px 2px 10px 0px;
		border-radius: 10px;
		border: 1px solid var(--highlight);
	}

	.details {
		display: flex;
		flex-direction: column;
		gap: 0.2em;
	}

	.icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.icon.loading {
		animation: rotate 500ms infinite ease-in-out;
	}

	@keyframes rotate {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
