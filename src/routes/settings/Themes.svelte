<script lang="ts">
	import darkTheme from '$lib/assets/dark.svg';
	import lightTheme from '$lib/assets/white.svg';
	import autoTheme from '$lib/assets/auto.svg';
	import type AppConfig from '$lib/config.svelte';
	import Check from 'lucide-svelte/icons/check';
	import { _ } from 'svelte-i18n';

	let { appConf }: { appConf: AppConfig } = $props();
</script>

<section class="ns">
	<h4>{$_('settings_page.section_ui_ux.theme.title')}</h4>
	<div class="themes">
		<div
			role="button"
			tabindex="0"
			id="theme"
			class="auto"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'auto'}
			onclick={async () => {
				await appConf.setTheme('auto');
			}}
			onkeydown={async (e) => {
				if (e.key.toLowerCase() === 'enter' || e.key.toLowerCase() === ' ') {
					await appConf.setTheme('auto');
				}
			}}
		>
			<img src={autoTheme} alt="" />
			<div class="selected">
				<Check color="var(--fg)" size="1em" />
			</div>
		</div>
		<div
			role="button"
			tabindex="0"
			id="theme"
			class="dark"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'dark'}
			onclick={async () => {
				await appConf.setTheme('dark');
			}}
			onkeydown={async (e) => {
				if (e.key.toLowerCase() === 'enter' || e.key.toLowerCase() === ' ') {
					await appConf.setTheme('dark');
				}
			}}
		>
			<img src={darkTheme} alt="" />
			<div class="selected">
				<Check color="var(--fg)" size="1em" />
			</div>
		</div>
		<div
			role="button"
			tabindex="0"
			id="theme"
			class="light"
			class:active={(appConf.config?.global?.theme ?? appConf.defaults.global.theme) == 'light'}
			onclick={async () => {
				await appConf.setTheme('light');
			}}
			onkeydown={async (e) => {
				if (e.key.toLowerCase() === 'enter' || e.key.toLowerCase() === ' ') {
					await appConf.setTheme('light');
				}
			}}
		>
			<div class="selected">
				<Check color="var(--fg)" size="1em" />
			</div>
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

	#theme:active {
		transform: scale(0.98);
	}

	#theme.active .selected {
		display: flex;
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
		justify-content: center;
		align-items: center;
	}

	#theme {
		position: relative;
		border-radius: 16px;
		display: flex;
		justify-content: center;
		align-items: center;
		border: 3px solid var(--highlight);
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
