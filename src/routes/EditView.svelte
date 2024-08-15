<script lang="ts">
	import { getEvc } from '$lib/editviewController.svelte';
	import { flyAndScale } from '$lib/utils/transitions';

	const evc = getEvc();
</script>

{#if evc.visible}
	<div class="__edit_view" transition:flyAndScale>
		{#if evc.componnentToRender}
			<evc.componnentToRender
				data={evc.data}
				onClose={() => {
					evc.visible = false;
				}}
			/>
		{/if}
	</div>
{/if}

<style>
	.__edit_view {
		z-index: var(--edit-view-z-index);
		position: fixed;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
	}
</style>
