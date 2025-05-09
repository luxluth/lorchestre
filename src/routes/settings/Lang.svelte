<script lang="ts">
	import { _, locale } from 'svelte-i18n';
	import { Select } from 'bits-ui';
	import langs from '$lib/i18n/langs';
	import { flyAndScale } from '$lib/utils/transitions';

	import CaretUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import Check from 'lucide-svelte/icons/check';
	import Languages from 'lucide-svelte/icons/languages';
	import type AppConfig from '$lib/config.svelte';

	let { appConf }: { appConf: AppConfig } = $props();
	let value = $state(appConf.config.global?.lang);

	const selectedLang = $derived(
		value
			? langs.find((lang) => lang.value === value)?.label
			: $_('settings_page.section_ui_ux.language.choose_a_lang')
	);
</script>

<section class="lang ns">
	<h4>{$_('settings_page.section_ui_ux.language.title')}</h4>

	<Select.Root
		type="single"
		items={langs}
		{value}
		onValueChange={(e: string) => {
			console.log(e);
			locale.set(e);
			(async () => {
				await appConf.setLocale(e as string);
			})();
			value = e;
		}}
	>
		<Select.Trigger
			class="select-trigger"
			aria-label={$_('settings_page.section_ui_ux.language.choose_a_lang')}
		>
			<Languages class="icon" />
			<div class="text">
				{selectedLang}
			</div>
			<CaretUpDown class="icon caret" />
		</Select.Trigger>
		<Select.Portal>
			<!-- transition:flyAndScale -->
			<Select.Content class="select-content" sideOffset={8}>
				{#each langs as lang}
					<Select.Item class="select-item" value={lang.value} label={lang.label}>
						{#snippet children({ selected })}
							{lang.label}
							{#if selected}
								<Check />
							{/if}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Portal>
	</Select.Root>
</section>

<style>
	h4 {
		padding-top: 1.5em;
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

	:global(.select-item[data-highlighted]) {
		background: var(--highlight);
	}

	:global(.select-content) {
		width: var(--bits-select-anchor-width);
		min-width: var(--bits-select-anchor-width);
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
</style>
