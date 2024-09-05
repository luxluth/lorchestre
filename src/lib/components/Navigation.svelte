<script lang="ts">
	import { goto } from '$app/navigation';
	import { getNav } from '$lib/nav.svelte';

	const n = getNav();
	let active = $state(false);

	const pages = [
		{ path: '/', name: 'Recommendation' },
		{ path: '/songs', name: 'Songs' },
		{ path: '/settings', name: 'Settings' }
	];
</script>

<section class="ns" class:active>
	{#if active}
		<div class="gotos">
			{#each pages as p}
				{#if n.pageRouteId != p.path}
					<div
						class="btn"
						role="button"
						tabindex="-1"
						onclick={() => {
							active = false;
							goto(p.path);
						}}
						onkeydown={() => {}}
					>
						{p.name}
					</div>
				{/if}
			{/each}
		</div>
	{/if}
	<div
		class="currentpage btn"
		class:active
		onclick={() => {
			active = !active;
		}}
		onkeydown={() => {}}
		role="button"
		tabindex="-1"
	>
		{n.pageName}
	</div>
</section>

<style>
	.gotos {
		padding-bottom: 1em;
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	.gotos .btn:hover {
		color: rgba(255, 255, 255, 0.7);
	}

	section.active {
		width: 100%;
		height: 100%;
		background: rgba(56, 56, 56, 0.25);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);

		display: flex;
		flex-direction: column;
		justify-content: flex-end;
		padding: 2vw;
		bottom: 0;
		left: 0;
	}

	section {
		transition: background 0.3s ease-in-out;
		position: fixed;
		bottom: 2vw;
		left: 2vw;
	}

	div.btn {
		width: fit-content;
		font-weight: 900;
		color: rgba(255, 255, 255, 0.5);
		transition: color 0.2s ease-in-out;
	}

	div.currentpage.active {
		color: rgba(255, 255, 255, 1);
	}
</style>
