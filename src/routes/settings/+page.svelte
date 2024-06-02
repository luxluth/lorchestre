<script lang="ts">
	import { _, locale } from 'svelte-i18n';
	import { Select } from 'bits-ui';
	import langs from '$lib/i18n/langs';
	import { flyAndScale } from '$lib/utils/transitions';

	import CaretUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import Check from 'lucide-svelte/icons/check';
	import Languages from 'lucide-svelte/icons/languages';
	import { invoke } from '@tauri-apps/api/core';
</script>

<h1>{$_('settings')}</h1>
<p class="tips">{$_('settings_page.tip_top')}</p>
<hr />

<section class="ui-ux">
	<h2>{$_('settings_page.section_ui_ux.title')}</h2>

	<div class="lang ns">
		<h4>{$_('settings_page.section_ui_ux.language.title')}</h4>

		<Select.Root
			items={langs}
			selected={langs.find((l) => l.value === $locale?.split('-')[0])}
			onSelectedChange={(e) => {
				locale.set(e?.value);
				(async () => {
					await invoke('set_locale', { locale: e?.value });
				})();
			}}
		>
			<Select.Trigger class="select-trigger" aria-label="Choisissez une langue">
				<Languages class="icon" />
				<Select.Value class="text" placeholder="Choisissez une langue" />
				<CaretUpDown class="icon caret" />
			</Select.Trigger>
			<Select.Content class="select-content" sideOffset={8} transition={flyAndScale}>
				{#each langs as lang}
					<Select.Item class="select-item" value={lang.value} label={lang.label}>
						{lang.label}
						<Select.ItemIndicator class="ml-auto" asChild={false}>
							<Check />
						</Select.ItemIndicator>
					</Select.Item>
				{/each}
			</Select.Content>
			<Select.Input name="favoriteFruit" />
		</Select.Root>
	</div>
</section>

<!-- <section class="backend"> -->
<!-- 	<h2>Fournisseur de musique</h2> -->
<!-- 	<div class="selections"> -->
<!-- 		<button class="local active"> -->
<!-- 			<div class="title">Fichiers Locaux</div> -->
<!-- 		</button> -->
<!-- 		<button class="spotify"> -->
<!-- 			<div class="title">Spotify</div> -->
<!-- 		</button> -->
<!-- 		<button class="mpd"> -->
<!-- 			<div class="title">MPD (Music Player Daemon)</div> -->
<!-- 		</button> -->
<!-- 	</div> -->
<!-- </section> -->

<style>
	/* .backend .selections { */
	/* 	display: grid; */
	/* 	height: 20rem; */
	/* 	grid-template-columns: 1fr 1fr 1fr; */
	/* 	gap: 2em; */
	/* } */
	/**/
	/* .selections button { */
	/* 	border: 2px solid var(--highlight); */
	/* 	border-radius: 20px; */
	/* 	background: none; */
	/* 	color: var(--fg); */
	/* 	display: flex; */
	/* 	padding: 2em; */
	/* } */
	/**/
	/* .selections button .title { */
	/* 	font-weight: bold; */
	/* 	font-size: 2em; */
	/* } */
	/**/
	/* .selections button.active { */
	/* 	border-color: var(--fg); */
	/* 	background: var(--highlight); */
	/* } */

	section {
		padding-block: 1.5em;
	}

	section h2 {
		padding-bottom: 1em;
	}

	h4 {
		padding-bottom: 0.5em;
		opacity: 0.5;
	}

	:global(.select-item) {
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: space-between;
		padding: 0.2em;
		padding-inline: 0.5em;
		height: 2em;
		border-radius: 4px;
	}

	:global(.select-item:hover) {
		background: var(--highlight);
	}

	:global(.select-content) {
		width: 100%;
		border-radius: 9px;
		border: 2px solid rgba(100, 100, 100, 0.18);
		background: var(--bg);
		padding: 0.3em;
	}

	:global(.select-trigger) {
		display: inline-flex;
		color: var(--fg);
		width: 256px;
		justify-content: flex-start;
		gap: 0.5em;
		align-items: center;
		border-radius: 9px;
		border: 2px solid rgba(100, 100, 100, 0.18);
		background: var(--bg);
		font-size: 1em;
		padding-inline: 0.5em;
		padding-block: 0.4em;
		cursor: pointer;
	}

	:global(.select-trigger .text) {
		flex-grow: 1;
		text-align: left;
	}

	:global(.select-trigger svg.icon) {
		opacity: 0.4;
	}

	h1 {
		padding-bottom: 0.2em;
	}

	hr {
		margin-top: 1.5em;
		opacity: 0.2;
	}

	p.tips {
		opacity: 0.5;
	}
</style>
