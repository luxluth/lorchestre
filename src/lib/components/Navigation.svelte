<script lang="ts">
	import { getNav } from '$lib/nav.svelte';

	const n = getNav();
	let active = $state(false);
</script>

<section class="ns" class:active>
	<div
		class="nav btn"
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
		position: absolute;
		bottom: 2vw;
		left: 2vw;
	}

	div.nav {
		width: fit-content;
		color: rgba(255, 255, 255, 0.5);
		font-weight: 900;
	}
</style>
