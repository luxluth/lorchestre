<script lang="ts">
	import '@fontsource-variable/noto-sans-kr';
	import '@fontsource-variable/noto-sans-jp';
	import '@fontsource-variable/noto-sans-sc';
	import '@fontsource-variable/readex-pro';
	import '../styles/global.css';

	import Toast from './Toast.svelte';
	import Player from './Player.svelte';

	let { children } = $props();
</script>

<div class="layout">
	<main>
		{@render children()}
	</main>
</div>
<Toast />
<Player />

<style>
	.layout {
		padding-inline: 3em;
		padding-block: 3em;
	}
</style>
