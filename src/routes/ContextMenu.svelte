<script lang="ts">
	import type Ctx from '$lib/ctx.svelte';
	import { ContextMenuItemType } from '$lib/type';
	import { clickOutside } from '$lib/utils';
	import { getContext } from 'svelte';

	let ctx = getContext<Ctx>('ctx');
</script>

<div
	class="__ctxmenu"
	class:visible={ctx.visible}
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

<style>
	.__ctxmenu {
		display: none;
		position: fixed;
		z-index: 200;
		border: 1px solid rgba(100, 100, 100, 0.18);
		border-radius: 4px;
		background-color: var(--bg);
		color: var(--fg);
		padding: 2px;
	}

	.__ctxmenu.visible {
		display: block;
	}

	.item {
		display: flex;
		gap: 1em;
		padding: 0.5em;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
	}

	.item:hover {
		background: rgba(100, 100, 100, 0.18);
		cursor: pointer;
	}
</style>
