<script lang="ts">
	import type SearchSupervisor from '$lib/search.svelte';
	import { getContext } from 'svelte';

	let search = getContext<SearchSupervisor>('ss');
</script>

<div class="search ns" class:active={search.query.length > 0}>
	<h1>Recherche</h1>
	{#if search.query.length > 0}
		<p class="tip">Résultat pour le terme <b>{search.query}</b></p>
	{:else}
		<p class="tip">Commencer à écrire pour chercher...</p>
	{/if}
</div>

<style>
	h1 {
		font-size: clamp(2.5rem, 1.8333rem + 5.3333vw, 7.5rem);
		font-family: var(--font-fantasy);
	}

	.tip {
		font-style: italic;
		text-wrap: wrap;
		word-wrap: break-word;
		width: 30vw;
	}
</style>
