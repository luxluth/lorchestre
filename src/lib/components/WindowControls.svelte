<script lang="ts">
	import LinuxWindowControls from './LinuxWindowControls.svelte';
	import WindowsWindowControls from './WindowsWindowControls.svelte';

	let { platform }: { platform: string } = $props();
	console.log(platform);
</script>

{#if platform === 'linux'}
	<LinuxWindowControls />
{:else if platform === 'windows'}
	<WindowsWindowControls />
{/if}
