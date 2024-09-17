<script lang="ts">
	import banner from '$lib/assets/banner-bw.svg?raw';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import { fade } from 'svelte/transition';
	let transitionParams = { duration: 300, delay: 0 };
</script>

<div class="page" out:fade={transitionParams}>
	<div class="icon ns">
		{@html banner}
	</div>

	<div class="indicator">
		<div class="icon load">
			<LoaderCircle size={'1em'} />
		</div>
	</div>
</div>

<style>
	.page {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		z-index: var(--starting-screen-z-index);
		display: flex;
		justify-content: center;
		align-items: center;
		background: var(--bg);
	}

	.indicator {
		position: absolute;
		bottom: 1em;
		right: 1em;
		width: 1em;
		height: 1em;
	}

	.icon.load {
		animation: rotate 500ms infinite ease-in-out;
	}

	.load {
		width: fit-content;
		height: fit-content;
		display: flex;
		justify-content: center;
		align-items: center;
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
