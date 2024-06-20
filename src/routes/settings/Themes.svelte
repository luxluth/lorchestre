<script lang="ts">
	import darkTheme from '$lib/assets/dark.svg';
	import lightTheme from '$lib/assets/white.svg';
	import autoTheme from '$lib/assets/auto.svg';
	import type AppConfig from '$lib/config.svelte';
	import Check from 'lucide-svelte/icons/check';

	let { appConf }: { appConf: AppConfig } = $props();
</script>

<section class="ns">
	<h4>Apparence de l'application</h4>
	<div class="themes">
		<div
			id="theme"
			class="auto"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'auto'}
		>
			<img src={autoTheme} alt="" />
			<div class="selected">
				<Check color="var(--fg)" size="1em" />
			</div>
		</div>
		<div
			id="theme"
			class="dark"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'dark'}
		>
			<img src={darkTheme} alt="" />
		</div>
		<div
			id="theme"
			class="light"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'light'}
		>
			<img src={lightTheme} alt="" />
		</div>
	</div>
</section>

<style>
	.themes {
		display: flex;
		align-items: center;
		padding-top: 0.5em;
		gap: 2em;
	}

	#theme.active {
		border-color: var(--fg);
	}

	#theme.active .selected {
		display: block;
	}

	#theme .selected {
		display: none;
		position: absolute;
		width: 1.5em;
		height: 1.5em;
		border: 2px solid var(--fg);
		background-color: var(--bg);
		right: 0.5em;
		top: 0.5em;
		border-radius: 50%;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	#theme {
		position: relative;
		border-radius: 8px;
		display: flex;
		justify-content: center;
		align-items: center;
		border: 2px solid var(--highlight);
		width: fit-content;
		height: fit-content;
		overflow: hidden;
		box-shadow:
			rgba(17, 17, 26, 0.1) 0px 4px 16px,
			rgba(17, 17, 26, 0.05) 0px 8px 32px;
	}

	#theme img {
		width: 100%;
		height: 100%;
	}

	h4 {
		padding-top: 1.5em;
		padding-bottom: 0.5em;
		opacity: 0.5;
	}
</style>
