<script lang="ts">
	import type Ctx from '$lib/ctx.svelte';
	import { ContextMenuItemType } from '$lib/type';
	import { clickOutside } from '$lib/utils';
	import { getContext } from 'svelte';

	import { flyAndScale } from '$lib/utils/transitions';

	let ctx = getContext<Ctx>('ctx');
</script>

{#if ctx.visible}
	<div
		transition:flyAndScale
		class="__ctxmenu glass"
		style="left: {ctx.x}px; top: {ctx.y}px;"
		use:clickOutside={() => {
			ctx.visible = false;
		}}
		role="menu"
	>
		{#each ctx.items as item}
			{#if item.type === ContextMenuItemType.Action}
				<div
					onkeydown={() => {}}
					class="item"
					role="menuitem"
					tabindex="0"
					onclick={async () => {
						await item.action(ctx.data);
						ctx.visible = false;
					}}
				>
					<div class="icon">
						{#if item.icon}
							<svelte:component this={item.icon} size={18} />
						{/if}
					</div>
					<div class="label">
						<p>{item.label}</p>
					</div>
				</div>
			{/if}
			{#if item.type === ContextMenuItemType.Separator}
				<hr />
			{/if}
		{/each}
	</div>
{/if}

<style>
	.__ctxmenu {
		transform-origin: top left;
		position: fixed;
		z-index: 200;
		width: 16rem;
		border: 2px solid rgba(100, 100, 100, 0.18);
		border-radius: 6px;
		background-color: var(--bg);
		color: var(--fg);
		padding: 2px;
	}

	@keyframes reveal-in {
		0% {
			opacity: 0;
			transform: scale(0.8);
		}
		to {
			transform: scale(1);
			opacity: 1;
		}
	}

	@keyframes reveal-out {
		0% {
			opacity: 1;
			transform: scale(1);
		}
		to {
			opacity: 0;
			transform: scale(0.8);
		}
	}

	.item {
		display: flex;
		gap: 1em;
		padding: 0.5em;
		align-items: center;
		border-radius: 4px;
	}

	.item:hover {
		background: var(--highlight);
		cursor: pointer;
	}
</style>
