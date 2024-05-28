<script lang="ts">
	import type { Media } from '$lib/type';

	let media = $state<Media>();
</script>

<div class="__mp">
	{#if media}
		lul
	{:else}
		<div class="shell"></div>
	{/if}
</div>

<style>
	.__mp {
		width: 100%;
		height: 5em;
		padding: 0.5em;
	}

	.shell {
		width: 100%;
		height: 100%;
	}
</style>
